use std::{path::Path, time::Duration};

use crate::config::AppConfig;
use crate::{AppError, Result};
use chrono::{DateTime, Local, Utc};
use rchronos_shared::{Agreement, RequestType, SyncMode};

mod fetch;

#[derive(Debug, Clone)]
pub struct HostCandidate {
    pub name: String,
    pub request_type: RequestType,
    pub priority: u32,
}

#[derive(Debug, Clone)]
pub struct SyncReport {
    pub server: String,
    pub request_type: RequestType,
    pub remote_utc: DateTime<Utc>,
    pub applied_utc: DateTime<Utc>,
    pub local_before: DateTime<Local>,
    pub deviation_ms: u64,
    pub method: String,
}

#[derive(Debug, Clone)]
pub struct HostAttempt {
    pub host: String,
    pub error: String,
}

#[derive(Debug, Clone)]
pub struct SyncResult {
    pub report: Option<SyncReport>,
    pub failed_attempts: Vec<HostAttempt>,
}

#[derive(Debug, Clone, Copy)]
pub enum SyncTrigger {
    Startup,
    Manual,
    Timer,
}

impl std::fmt::Display for SyncTrigger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyncTrigger::Startup => write!(f, "startup"),
            SyncTrigger::Manual => write!(f, "manual"),
            SyncTrigger::Timer => write!(f, "timer"),
        }
    }
}

pub async fn perform_sync(config: &AppConfig, _config_path: &Path) -> Result<SyncResult> {
    let _ = rchronos_windows::apply_windows_time_policy(config.disable_win32_time);

    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(config.network_timeout_ms.max(1)))
        .build()
        .map_err(|e| AppError::msg(format!("build HTTP client: {e}")))?;

    let mut failed_attempts = Vec::new();
    for host in collect_candidates(config) {
        if !request_allowed(config, host.request_type) {
            continue;
        }

        match fetch::fetch_time(config, &client, &host).await {
            Ok(remote_utc) => {
                let local_before = Local::now();
                let adjusted_utc =
                    remote_utc + chrono::Duration::milliseconds(config.offset_ms as i64);
                let now_utc = Utc::now();
                let deviation_ms = (adjusted_utc - now_utc).num_milliseconds().unsigned_abs();

                let report = if deviation_ms <= config.deviation_offset_ms {
                    SyncReport {
                        server: host.name,
                        request_type: host.request_type,
                        remote_utc,
                        applied_utc: adjusted_utc,
                        local_before,
                        deviation_ms,
                        method: "skip-within-deviation".to_string(),
                    }
                } else {
                    let method = match config.sync_mode {
                        SyncMode::Off => "disabled".to_string(),
                        SyncMode::Immediate => {
                            rchronos_windows::set_system_time_direct(adjusted_utc)
                                .map_err(|e| AppError::msg(e.to_string()))?;
                            "direct".to_string()
                        }
                        SyncMode::Slew => rchronos_windows::slew_system_time(adjusted_utc)
                            .map_err(|e| AppError::msg(e.to_string()))?
                            .to_string(),
                    };

                    SyncReport {
                        server: host.name,
                        request_type: host.request_type,
                        remote_utc,
                        applied_utc: adjusted_utc,
                        local_before,
                        deviation_ms,
                        method,
                    }
                };

                return Ok(SyncResult {
                    report: Some(report),
                    failed_attempts,
                });
            }
            Err(err) => {
                failed_attempts.push(HostAttempt {
                    host: host.name.clone(),
                    error: err.to_string(),
                });
            }
        }
    }

    Ok(SyncResult {
        report: None,
        failed_attempts,
    })
}

fn request_allowed(config: &AppConfig, request_type: RequestType) -> bool {
    match config.agreement {
        Agreement::NtpOnly => request_type == RequestType::Ntp,
        Agreement::HttpOnly => {
            request_type == RequestType::Http || request_type == RequestType::Https
        }
        Agreement::Mixed => true,
    }
}

pub(crate) fn collect_candidates(config: &AppConfig) -> Vec<HostCandidate> {
    let mut hosts = Vec::with_capacity(config.hosts.len());
    for (name, host) in &config.hosts {
        if !host.enabled {
            continue;
        }

        hosts.push(HostCandidate {
            name: name.clone(),
            request_type: host.request_type,
            priority: host.priority,
        });
    }

    hosts.sort_by(|a, b| {
        a.priority
            .cmp(&b.priority)
            .then_with(|| a.name.cmp(&b.name))
    });
    hosts
}

pub fn request_mode_name(mode: RequestType) -> &'static str {
    match mode {
        RequestType::Ntp => "NTP",
        RequestType::Http => "HTTP",
        RequestType::Https => "HTTPS",
    }
}
