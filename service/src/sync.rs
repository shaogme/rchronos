use std::{
    net::UdpSocket,
    path::Path,
    time::Duration,
};

use crate::config::AppConfig;
use crate::{AppError, Result};
use chrono::{DateTime, Local, TimeZone, Utc};
use httpdate::parse_http_date;
use rchronos_shared::{Agreement, RequestType, SyncMode};
use reqwest::blocking::Client;

pub const NTP_PORT: u16 = 123;
pub const NTP_EPOCH_UNIX_OFFSET: i64 = 2_208_988_800;

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

pub fn perform_sync(config: &AppConfig, _config_path: &Path) -> Result<SyncResult> {
    let _ = rchronos_windows::apply_windows_time_policy(config.disable_win32_time);

    let client = Client::builder()
        .timeout(Duration::from_millis(config.network_timeout_ms.max(1)))
        .build()
        .map_err(|e| AppError::msg(format!("build HTTP client: {e}")))?;

    let mut failed_attempts = Vec::new();
    for host in collect_candidates(config) {
        if !request_allowed(config, host.request_type) {
            continue;
        }

        match fetch_time(config, &client, &host) {
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
                        SyncMode::Slew => {
                            rchronos_windows::slew_system_time(adjusted_utc)
                                .map_err(|e| AppError::msg(e.to_string()))?
                                .to_string()
                        }
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

fn fetch_time(config: &AppConfig, client: &Client, host: &HostCandidate) -> Result<DateTime<Utc>> {
    match host.request_type {
        RequestType::Ntp => fetch_ntp_time(host.name.as_str(), config.network_timeout_ms),
        RequestType::Http => fetch_http_time(
            client,
            "http",
            config.user_agent.as_str(),
            host.name.as_str(),
        ),
        RequestType::Https => fetch_http_time(
            client,
            "https",
            config.user_agent.as_str(),
            host.name.as_str(),
        ),
    }
}

fn fetch_ntp_time(host: &str, timeout_ms: u64) -> Result<DateTime<Utc>> {
    let address = format!("{host}:{NTP_PORT}");
    let socket =
        UdpSocket::bind("0.0.0.0:0").map_err(|e| AppError::msg(format!("bind UDP socket: {e}")))?;
    let timeout = Duration::from_millis(timeout_ms.max(1));
    socket
        .set_read_timeout(Some(timeout))
        .map_err(|e| AppError::msg(format!("set UDP read timeout: {e}")))?;
    socket
        .set_write_timeout(Some(timeout))
        .map_err(|e| AppError::msg(format!("set UDP write timeout: {e}")))?;

    let mut packet = [0_u8; 48];
    packet[0] = 0x1B;

    let now = Utc::now();
    let unix_seconds = now.timestamp() + NTP_EPOCH_UNIX_OFFSET;
    let nanos = now.timestamp_subsec_nanos() as u64;
    let fraction = ((nanos << 32) / 1_000_000_000) as u32;
    let seconds = unix_seconds as u32;
    packet[40..44].copy_from_slice(&seconds.to_be_bytes());
    packet[44..48].copy_from_slice(&fraction.to_be_bytes());

    socket
        .send_to(&packet, &address)
        .map_err(|e| AppError::msg(format!("send NTP packet to {address}: {e}")))?;
    let mut response = [0_u8; 48];
    socket
        .recv_from(&mut response)
        .map_err(|e| AppError::msg(format!("receive NTP packet from {address}: {e}")))?;

    let seconds = u32::from_be_bytes(
        response[40..44]
            .try_into()
            .map_err(|_| AppError::msg("decode NTP seconds"))?,
    ) as i64;
    let fraction = u32::from_be_bytes(
        response[44..48]
            .try_into()
            .map_err(|_| AppError::msg("decode NTP fraction"))?,
    ) as i64;
    let unix_seconds = seconds - NTP_EPOCH_UNIX_OFFSET;
    let nanos = ((fraction as i128 * 1_000_000_000i128) >> 32) as u32;
    let remote = Utc
        .timestamp_opt(unix_seconds, nanos)
        .single()
        .ok_or_else(|| AppError::msg("decode NTP time"))?;
    Ok(remote)
}

fn fetch_http_time(
    client: &Client,
    scheme: &str,
    user_agent: &str,
    host: &str,
) -> Result<DateTime<Utc>> {
    let url = if host.starts_with("http://") || host.starts_with("https://") {
        host.to_string()
    } else {
        format!("{scheme}://{host}")
    };

    let response = client
        .head(&url)
        .header(reqwest::header::USER_AGENT, user_agent)
        .send()
        .map_err(|e| AppError::msg(format!("HEAD {url}: {e}")))?
        .error_for_status()
        .map_err(|e| AppError::msg(format!("HTTP status for {url}: {e}")))?;

    let header = response
        .headers()
        .get(reqwest::header::DATE)
        .ok_or_else(|| AppError::msg("missing Date header"))?;
    let date = parse_http_date(
        header
            .to_str()
            .map_err(|e| AppError::msg(format!("invalid Date header: {e}")))?,
    )
    .map_err(|e| AppError::msg(format!("parse Date header: {e}")))?;
    Ok(DateTime::<Utc>::from(date))
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
