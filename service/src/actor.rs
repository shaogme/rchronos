use std::{
    collections::BTreeMap,
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use chrono::Local;
use tokio::sync::{Notify, mpsc, oneshot, watch};
use tracing::{error, info, warn};

use crate::config::AppConfigExt;
use crate::sync::{SyncResult, SyncTrigger, collect_candidates, perform_sync, request_mode_name};
use crate::{AppError, Result};
use rchronos_shared::{AppConfig, HostStatus, RuntimeSnapshot, RuntimeStatus};

#[derive(Debug)]
pub enum AppMessage {
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
    host_failures: BTreeMap<String, (u32, Option<String>)>,
    syncing: bool,
}

struct AppActor {
    receiver: mpsc::Receiver<AppMessage>,
    sender: mpsc::Sender<AppMessage>,
    state: RuntimeState,
    config_path: PathBuf,
    snapshot_sender: watch::Sender<Arc<RuntimeSnapshot>>,
}

impl AppActor {
    fn new(
        receiver: mpsc::Receiver<AppMessage>,
        sender: mpsc::Sender<AppMessage>,
        config: AppConfig,
        config_path: PathBuf,
        snapshot_sender: watch::Sender<Arc<RuntimeSnapshot>>,
    ) -> Self {
        let state = RuntimeState {
            config,
            logs: vec![],
            status: RuntimeStatus {
                current_operation: "Ready".to_string(),
                hosts: vec![],
            },
            host_failures: BTreeMap::new(),
            syncing: false,
        };
        let mut actor = Self {
            receiver,
            sender,
            state,
            config_path,
            snapshot_sender,
        };
        actor.update_hosts_in_status();
        actor.update_snapshot_cache();
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

    fn update_snapshot_cache(&self) {
        let snapshot = RuntimeSnapshot {
            config: self.state.config.clone(),
            logs: self.state.logs.clone(),
            status: self.state.status.clone(),
            syncing: self.state.syncing,
            config_path: self.config_path.display().to_string(),
        };
        let _ = self.snapshot_sender.send(Arc::new(snapshot));
    }

    async fn run(&mut self) {
        while let Some(msg) = self.receiver.recv().await {
            match msg {
                AppMessage::Log(message) => {
                    self.state.logs.push(message);
                    if self.state.logs.len() > self.state.config.max_log_lines {
                        let overflow = self.state.logs.len() - self.state.config.max_log_lines;
                        self.state.logs.drain(0..overflow);
                    }
                    self.update_snapshot_cache();
                }
                AppMessage::SetStatus(status) => {
                    self.state.status.current_operation = status;
                    self.update_snapshot_cache();
                }
                AppMessage::ReloadConfig(tx) => {
                    let res = match AppConfig::load(&self.config_path) {
                        Ok(config) => {
                            self.state.config = config;
                            self.update_hosts_in_status();
                            self.update_snapshot_cache();
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
                    // 保存后也刷新缓存
                    self.update_snapshot_cache();
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
                    self.update_snapshot_cache();
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
                            // 这里暂时用异步 perform_sync，后文会将 perform_sync 异步化
                            let result = perform_sync(&config, &config_path).await;
                            let _ = tx.send(AppMessage::SyncFinished(result, trigger)).await;
                        });
                    }
                    if self.state.logs.len() > self.state.config.max_log_lines {
                        let overflow = self.state.logs.len() - self.state.config.max_log_lines;
                        self.state.logs.drain(0..overflow);
                    }
                    self.update_snapshot_cache();
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
                    self.update_snapshot_cache();
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
    pub shutdown: Notify,
    snapshot_receiver: watch::Receiver<Arc<RuntimeSnapshot>>,
    periodic_started: AtomicBool,
}

impl AppRuntime {
    pub fn new(config_path: PathBuf, config: AppConfig) -> Arc<Self> {
        let (tx, rx) = mpsc::channel(100);

        let initial_snapshot = RuntimeSnapshot {
            config: config.clone(),
            logs: vec![],
            status: RuntimeStatus {
                current_operation: "Ready".to_string(),
                hosts: vec![],
            },
            syncing: false,
            config_path: config_path.display().to_string(),
        };

        let (snapshot_sender, snapshot_receiver) = watch::channel(Arc::new(initial_snapshot));

        let runtime = Arc::new(Self {
            config_path: config_path.clone(),
            sender: tx.clone(),
            shutdown: Notify::new(),
            snapshot_receiver,
            periodic_started: AtomicBool::new(false),
        });

        let mut actor = AppActor::new(rx, tx, config, config_path, snapshot_sender);
        tokio::spawn(async move {
            actor.run().await;
        });

        runtime
    }

    /// 高性能快照获取接口：原子性返回 Arc 共享快照，实现 O(1) 并发无锁无拷贝读取。
    pub fn snapshot(&self) -> Arc<RuntimeSnapshot> {
        self.snapshot_receiver.borrow().clone()
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
