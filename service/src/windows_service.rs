use std::{
    ffi::OsString,
    io::Write,
    net::{Ipv4Addr, SocketAddr},
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use tokio::net::TcpListener;
use tracing::{error, info};

use windows_service::{
    define_windows_service,
    service::{
        ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
        ServiceType,
    },
    service_control_handler::{self, ServiceControlHandlerResult, ServiceStatusHandle},
    service_dispatcher,
};

use crate::actor::AppRuntime;
use crate::config::{AppConfig, AppConfigExt, config_path};
use crate::sync::SyncTrigger;
use crate::{AppError, Result};

pub const SERVICE_TYPE: ServiceType = ServiceType::OWN_PROCESS;

pub fn report_event_log(level: rchronos_windows::EventLogLevel, message: &str) {
    let _ = rchronos_windows::report_event_log(level, message);
}

pub fn setup_logging() -> (tracing_appender::non_blocking::WorkerGuard, PathBuf) {
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

        error!("PANIC in service process at {}: {}", location, message);

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
    let config = app.snapshot().config.clone();
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
    let router = crate::web::build_router(app.clone());

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

pub fn run_windows_service() -> Result<()> {
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

pub fn service_main(_arguments: Vec<OsString>) {
    if let Err(err) = run_windows_service() {
        let msg = format!("windows service error: {err}");
        error!("{}", msg);
        report_event_log(rchronos_windows::EventLogLevel::Error, &msg);
    }
}

pub fn dispatch_service(service_name: String) -> windows_service::Result<()> {
    service_dispatcher::start(service_name, ffi_service_main)
}
