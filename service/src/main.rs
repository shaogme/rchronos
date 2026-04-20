#[cfg(not(windows))]
compile_error!("rchronos only supports Windows service builds.");

use std::{
    ffi::OsString,
    io::Write,
    net::{Ipv4Addr, SocketAddr},
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use chrono::Local;
use thiserror::Error;
use tokio::{
    net::TcpListener,
    sync::{mpsc, oneshot},
};
use tracing::{error, info, warn};
use windows::Win32::System::EventLog::{
    DeregisterEventSource, EVENTLOG_ERROR_TYPE, REPORT_EVENT_TYPE, RegisterEventSourceW,
    ReportEventW,
};
use windows::core::w;

use rchronos_shared::{HostStatus, RuntimeSnapshot, RuntimeStatus};

mod config;
mod sync;
mod web;

use config::{AppConfig, AppConfigExt, ConfigError, config_path};
use sync::{SyncResult, SyncTrigger, collect_candidates, perform_sync, request_mode_name};

use windows_service::{
    define_windows_service,
    service::{
        ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
        ServiceType,
    },
    service_control_handler::{self, ServiceControlHandlerResult, ServiceStatusHandle},
    service_dispatcher,
};

pub const SERVICE_TYPE: ServiceType = ServiceType::OWN_PROCESS;

fn report_event_log(level: REPORT_EVENT_TYPE, message: &str) {
    unsafe {
        let source = w!("rchronos");
        if let Ok(handle) = RegisterEventSourceW(None, source) {
            let message_wide: Vec<u16> = message.encode_utf16().chain(std::iter::once(0)).collect();
            let pcwstr = windows::core::PCWSTR(message_wide.as_ptr());
            let strings = [pcwstr];
            let _ = ReportEventW(handle, level, 0, 1, None, 0, Some(&strings), None);
            let _ = DeregisterEventSource(handle);
        }
    }
}

fn setup_logging() -> (tracing_appender::non_blocking::WorkerGuard, PathBuf) {
    let exe_path =
        std::env::current_exe().unwrap_or_else(|_| PathBuf::from("rchronos-service.exe"));
    let log_dir = exe_path.parent().unwrap_or(Path::new(".")).join("logs");
    let _ = std::fs::create_dir_all(&log_dir);

    let file_appender = tracing_appender::rolling::daily(&log_dir, "rchronos.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_writer(non_blocking)
        .with_ansi(false)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // Setup panic hook to log panics to the file
    std::panic::set_hook(Box::new(|panic_info| {
        let message = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            *s
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            &s[..]
        } else {
            "Unknown panic"
        };
        let location = panic_info
            .location()
            .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
            .unwrap_or_else(|| "unknown location".to_string());

        // Use a standard error! call - though this might be buffered
        error!("PANIC in service process at {}: {}", location, message);

        // Also write to a "panic.txt" in the log directory directly (unbuffered)
        let exe_path = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
        let log_dir = exe_path.parent().unwrap_or(Path::new(".")).join("logs");
        let _ = std::fs::create_dir_all(&log_dir);
        let panic_file = log_dir.join("panic.txt");
        if let Ok(mut f) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(panic_file)
        {
            let _ = writeln!(
                f,
                "[{}] PANIC at {}: {}",
                chrono::Local::now(),
                location,
                message
            );
        }
    }));

    (guard, log_dir)
}

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    Message(String),
    #[error(transparent)]
    Config(#[from] ConfigError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    TomlSer(#[from] toml::ser::Error),
    #[error(transparent)]
    TomlDe(#[from] toml::de::Error),
    #[cfg(windows)]
    #[error(transparent)]
    Windows(#[from] windows::core::Error),
    #[cfg(windows)]
    #[error(transparent)]
    Service(#[from] windows_service::Error),
}

impl AppError {
    pub fn msg(message: impl Into<String>) -> Self {
        Self::Message(message.into())
    }
}

fn load_config_or_default(path: &Path) -> AppConfig {
    match AppConfig::load(path) {
        Ok(config) => config,
        Err(err) => {
            error!("failed to load config: {err}");
            AppConfig::default()
        }
    }
}

fn service_status(
    current_state: ServiceState,
    controls_accepted: ServiceControlAccept,
    checkpoint: u32,
    wait_hint: Duration,
) -> ServiceStatus {
    ServiceStatus {
        service_type: SERVICE_TYPE,
        current_state,
        controls_accepted,
        exit_code: ServiceExitCode::Win32(0),
        checkpoint,
        wait_hint,
        process_id: None,
    }
}

async fn run_application(
    app: Arc<AppRuntime>,
    status_handle: Option<ServiceStatusHandle>,
) -> Result<()> {
    let config = app.snapshot().await.config;
    let web_port = config.web_port;

    app.log(format!("Loaded config from {}", app.config_path.display()))
        .await;
    app.log(format!("Serving web UI on http://127.0.0.1:{web_port}/"))
        .await;

    info!("Starting periodic loop...");
    app.clone().start_periodic_loop().await;

    info!("Binding TcpListener to 127.0.0.1:{}...", web_port);
    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, web_port));
    let listener = TcpListener::bind(addr).await.map_err(|e| {
        let msg = format!("Failed to bind http listener on port {web_port}: {e}");
        error!("{}", msg);
        AppError::msg(msg)
    })?;

    info!("Successfully bound to port {web_port}");

    if let Some(handle) = status_handle {
        info!("Reporting ServiceState::Running to SCM");
        handle.set_service_status(service_status(
            ServiceState::Running,
            ServiceControlAccept::STOP,
            0,
            Duration::default(),
        ))?;
    }

    info!("Triggering Startup synchronization...");
    app.clone().request(SyncTrigger::Startup).await;

    info!("Building Axum router...");
    let router = web::build_router(app.clone());

    let shutdown = async move {
        info!("Shutdown listener active");
        app.shutdown.notified().await;
        info!("Shutdown signal received");
    };

    info!("Starting axum server...");
    let result = axum::serve(listener, router)
        .with_graceful_shutdown(shutdown)
        .await
        .map_err(|e| {
            let msg = format!("http server error: {e}");
            error!("{}", msg);
            AppError::msg(msg)
        });

    info!("Axum server stopped. Result: {:?}", result);

    if let Some(handle) = status_handle {
        let _ = handle.set_service_status(service_status(
            ServiceState::Stopped,
            ServiceControlAccept::empty(),
            0,
            Duration::default(),
        ));
    }

    result
}

fn run_windows_service() -> Result<()> {
    let (_guard, log_dir) = setup_logging();
    let config_path = config_path();
    let config = load_config_or_default(&config_path);

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(|e| AppError::msg(format!("build tokio runtime: {e}")))?;

    // Enter the runtime context so that tokio::spawn calls in AppRuntime::new work
    let _runtime_guard = runtime.enter();

    info!(
        "Starting {} service, logs in {}",
        config.service_name,
        log_dir.display()
    );
    let service_name = config.service_name.clone();
    let app = AppRuntime::new(config_path.clone(), config);

    let control_app = app.clone();
    let status_handle =
        service_control_handler::register(&service_name, move |event| match event {
            ServiceControl::Stop => {
                control_app.stop();
                ServiceControlHandlerResult::NoError
            }
            ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
            _ => ServiceControlHandlerResult::NotImplemented,
        })?;

    status_handle.set_service_status(service_status(
        ServiceState::StartPending,
        ServiceControlAccept::empty(),
        1,
        Duration::from_secs(5),
    ))?;

    let result = runtime.block_on(async move { run_application(app, Some(status_handle)).await });

    if let Err(err) = &result {
        error!("service failed: {err}");
    }

    result
}

define_windows_service!(ffi_service_main, service_main);

fn service_main(_arguments: Vec<OsString>) {
    if let Err(err) = run_windows_service() {
        let msg = format!("windows service error: {err}");
        error!("{}", msg);
        report_event_log(EVENTLOG_ERROR_TYPE, &msg);
    }
}

fn main() -> windows_service::Result<()> {
    let config_path = config_path();
    let config = load_config_or_default(&config_path);
    let service_name = config.service_name.clone();

    if let Err(e) = service_dispatcher::start(service_name, ffi_service_main) {
        report_event_log(
            EVENTLOG_ERROR_TYPE,
            &format!("Failed to start service dispatcher: {e}"),
        );
        return Err(e);
    }
    Ok(())
}

#[derive(Debug)]
enum AppMessage {
    GetSnapshot(oneshot::Sender<RuntimeSnapshot>),
    Log(String),
    SetStatus(String),
    ReloadConfig(oneshot::Sender<Result<()>>),
    SaveConfig(oneshot::Sender<Result<()>>),
    UpdateConfigFromToml(String, oneshot::Sender<Result<()>>),
    RequestSync(SyncTrigger),
    SyncFinished(Result<SyncResult>, SyncTrigger),
    GetConfigDelay(oneshot::Sender<u64>),
}

#[derive(Debug)]
struct RuntimeState {
    config: AppConfig,
    logs: Vec<String>,
    status: RuntimeStatus,
    host_failures: std::collections::BTreeMap<String, (u32, Option<String>)>,
    syncing: bool,
}

struct AppActor {
    receiver: mpsc::Receiver<AppMessage>,
    sender: mpsc::Sender<AppMessage>,
    state: RuntimeState,
    config_path: PathBuf,
}

impl AppActor {
    fn new(
        receiver: mpsc::Receiver<AppMessage>,
        sender: mpsc::Sender<AppMessage>,
        config: AppConfig,
        config_path: PathBuf,
    ) -> Self {
        let state = RuntimeState {
            config,
            logs: vec![],
            status: RuntimeStatus {
                current_operation: "Ready".to_string(),
                hosts: vec![],
            },
            host_failures: std::collections::BTreeMap::new(),
            syncing: false,
        };
        let mut actor = Self {
            receiver,
            sender,
            state,
            config_path,
        };
        actor.update_hosts_in_status();
        actor
    }

    fn update_hosts_in_status(&mut self) {
        let candidates = collect_candidates(&self.state.config);
        self.state.status.hosts = candidates
            .into_iter()
            .map(|c| {
                let (fail_count, last_error) = self
                    .state
                    .host_failures
                    .get(&c.name)
                    .cloned()
                    .unwrap_or((0, None));
                HostStatus {
                    name: c.name,
                    request_type: c.request_type,
                    priority: c.priority,
                    fail_count,
                    last_error,
                }
            })
            .collect();
    }

    async fn run(&mut self) {
        while let Some(msg) = self.receiver.recv().await {
            match msg {
                AppMessage::GetSnapshot(tx) => {
                    let snapshot = RuntimeSnapshot {
                        config: self.state.config.clone(),
                        logs: self.state.logs.clone(),
                        status: self.state.status.clone(),
                        syncing: self.state.syncing,
                        config_path: self.config_path.display().to_string(),
                    };
                    let _ = tx.send(snapshot);
                }
                AppMessage::Log(message) => {
                    self.state.logs.push(message);
                    if self.state.logs.len() > self.state.config.max_log_lines {
                        let overflow = self.state.logs.len() - self.state.config.max_log_lines;
                        self.state.logs.drain(0..overflow);
                    }
                }
                AppMessage::SetStatus(status) => {
                    self.state.status.current_operation = status;
                }
                AppMessage::ReloadConfig(tx) => {
                    let res = match AppConfig::load(&self.config_path) {
                        Ok(config) => {
                            self.state.config = config;
                            self.update_hosts_in_status();
                            Ok(())
                        }
                        Err(err) => Err(err.into()),
                    };
                    let _ = tx.send(res);
                }
                AppMessage::SaveConfig(tx) => {
                    let res = self
                        .state
                        .config
                        .save(&self.config_path)
                        .map_err(Into::into);
                    let _ = tx.send(res);
                }
                AppMessage::UpdateConfigFromToml(content, tx) => {
                    let res = (|| {
                        let config: AppConfig = toml::from_str(&content)
                            .map_err(|e| AppError::msg(format!("parse config form: {e}")))?;
                        self.state.config = config;
                        self.update_hosts_in_status();
                        self.state
                            .config
                            .save(&self.config_path)
                            .map_err(AppError::from)?;
                        Ok(())
                    })();
                    let _ = tx.send(res);
                }
                AppMessage::RequestSync(trigger) => {
                    if self.state.syncing {
                        self.state
                            .logs
                            .push(format!("Sync ignored: already running ({trigger})"));
                    } else {
                        self.state.syncing = true;
                        self.state.status.current_operation = format!("Syncing ({trigger})");
                        self.update_hosts_in_status();
                        self.state.logs.push(format!("Starting {trigger} sync"));

                        let config = self.state.config.clone();
                        let config_path = self.config_path.clone();
                        let tx = self.sender.clone();

                        tokio::spawn(async move {
                            let result = tokio::task::spawn_blocking(move || {
                                perform_sync(&config, &config_path)
                            })
                            .await
                            .map_err(|e| AppError::msg(format!("join sync worker: {e}")));

                            let flat_result = match result {
                                Ok(res) => res,
                                Err(e) => Err(e),
                            };

                            let _ = tx
                                .send(AppMessage::SyncFinished(flat_result, trigger))
                                .await;
                        });
                    }
                    if self.state.logs.len() > self.state.config.max_log_lines {
                        let overflow = self.state.logs.len() - self.state.config.max_log_lines;
                        self.state.logs.drain(0..overflow);
                    }
                }
                AppMessage::SyncFinished(result, trigger) => {
                    self.state.syncing = false;
                    match result {
                        Ok(sync_res) => {
                            // Update failure states for failed attempts in this run
                            for attempt in sync_res.failed_attempts {
                                let entry =
                                    self.state.host_failures.entry(attempt.host).or_default();
                                entry.0 += 1;
                                entry.1 = Some(attempt.error);
                            }

                            if let Some(report) = sync_res.report {
                                // Success - clear failure state for THIS host
                                self.state.host_failures.remove(&report.server);

                                self.state.status.current_operation = "Ready".to_string();
                                self.state.logs.push(format!(
                                    "[OK] {} ({}) -> {} | deviation={}ms | {}",
                                    report.server,
                                    request_mode_name(report.request_type),
                                    report
                                        .applied_utc
                                        .with_timezone(&Local)
                                        .format("%Y-%m-%d %H:%M:%S"),
                                    report.deviation_ms,
                                    report.method
                                ));
                                self.state.logs.push(format!(
                                    "Remote UTC: {}, Local before: {}",
                                    report.remote_utc.format("%Y-%m-%d %H:%M:%S"),
                                    report.local_before.format("%Y-%m-%d %H:%M:%S")
                                ));
                            } else {
                                // All failed
                                self.state.status.current_operation =
                                    format!("Sync failed ({trigger})");
                                self.state
                                    .logs
                                    .push(format!("E: all hosts failed in {trigger} sync"));
                            }
                        }
                        Err(err) => {
                            self.state.status.current_operation = "Engine error".to_string();
                            self.state.logs.push(format!("E: sync engine error: {err}"));
                        }
                    }
                    self.update_hosts_in_status();

                    if self.state.logs.len() > self.state.config.max_log_lines {
                        let overflow = self.state.logs.len() - self.state.config.max_log_lines;
                        self.state.logs.drain(0..overflow);
                    }
                }
                AppMessage::GetConfigDelay(tx) => {
                    let _ = tx.send(self.state.config.delay_ms);
                }
            }
        }
    }
}

pub struct AppRuntime {
    pub config_path: PathBuf,
    sender: mpsc::Sender<AppMessage>,
    pub shutdown: tokio::sync::Notify,
    periodic_started: AtomicBool,
}

impl AppRuntime {
    pub fn new(config_path: PathBuf, config: AppConfig) -> Arc<Self> {
        let (tx, rx) = mpsc::channel(100);
        let runtime = Arc::new(Self {
            config_path: config_path.clone(),
            sender: tx.clone(),
            shutdown: tokio::sync::Notify::new(),
            periodic_started: AtomicBool::new(false),
        });

        let mut actor = AppActor::new(rx, tx, config, config_path);
        tokio::spawn(async move {
            actor.run().await;
        });

        runtime
    }

    pub async fn snapshot(&self) -> RuntimeSnapshot {
        let (tx, rx) = oneshot::channel();
        let _ = self.sender.send(AppMessage::GetSnapshot(tx)).await;
        rx.await.unwrap_or_else(|_| RuntimeSnapshot {
            config: AppConfig::default(),
            logs: vec!["E: Actor died".to_string()],
            status: RuntimeStatus {
                current_operation: "Error".to_string(),
                hosts: vec![],
            },
            syncing: false,
            config_path: self.config_path.display().to_string(),
        })
    }

    pub async fn log(&self, message: impl Into<String>) {
        let message = message.into();
        if message.starts_with("E: ") || message.starts_with("Error: ") {
            error!("{}", message);
        } else if message.starts_with("W: ") || message.starts_with("Warning: ") {
            warn!("{}", message);
        } else {
            info!("{}", message);
        }

        let _ = self.sender.send(AppMessage::Log(message)).await;
    }

    pub async fn set_status(&self, status: impl Into<String>) {
        let _ = self.sender.send(AppMessage::SetStatus(status.into())).await;
    }

    pub async fn reload_config(&self) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        let _ = self.sender.send(AppMessage::ReloadConfig(tx)).await;
        match rx.await {
            Ok(res) => {
                if res.is_ok() {
                    self.log(format!(
                        "Reloaded config from {}",
                        self.config_path.display()
                    ))
                    .await;
                    self.set_status("Config reloaded").await;
                }
                res
            }
            Err(_) => Err(AppError::msg("Actor died")),
        }
    }

    pub async fn save_config(&self) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        let _ = self.sender.send(AppMessage::SaveConfig(tx)).await;
        match rx.await {
            Ok(res) => {
                if res.is_ok() {
                    self.log(format!("Saved config to {}", self.config_path.display()))
                        .await;
                    self.set_status("Config saved").await;
                }
                res
            }
            Err(_) => Err(AppError::msg("Actor died")),
        }
    }

    pub async fn update_config_from_toml(&self, content: &str) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        let _ = self
            .sender
            .send(AppMessage::UpdateConfigFromToml(content.to_string(), tx))
            .await;
        match rx.await {
            Ok(res) => res,
            Err(_) => Err(AppError::msg("Actor died")),
        }
    }

    pub async fn request(self: Arc<Self>, trigger: SyncTrigger) {
        let _ = self.sender.send(AppMessage::RequestSync(trigger)).await;
    }

    pub async fn start_periodic_loop(self: Arc<Self>) {
        if self.periodic_started.swap(true, Ordering::SeqCst) {
            return;
        }

        tokio::spawn(async move {
            loop {
                let (tx, rx) = oneshot::channel();
                let _ = self.sender.send(AppMessage::GetConfigDelay(tx)).await;
                let delay_ms = rx.await.unwrap_or(60_000).max(1);

                tokio::select! {
                    _ = tokio::time::sleep(Duration::from_millis(delay_ms)) => {},
                    _ = self.shutdown.notified() => break,
                }

                self.clone().request(SyncTrigger::Timer).await;
            }
        });
    }

    pub fn stop(&self) {
        self.shutdown.notify_waiters();
    }
}
