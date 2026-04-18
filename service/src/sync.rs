use std::{
    net::UdpSocket,
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

use crate::config::AppConfig;
use crate::{AppError, Result};
use chrono::{DateTime, Datelike, Local, TimeZone, Timelike, Utc};
use httpdate::parse_http_date;
use rchronos_shared::{Agreement, RequestType, SyncMode};
use reqwest::blocking::Client;

use windows::Win32::{
    Foundation::{CloseHandle, HANDLE, LUID, SYSTEMTIME},
    Security::{
        AdjustTokenPrivileges, LUID_AND_ATTRIBUTES, LookupPrivilegeValueW, SE_PRIVILEGE_ENABLED,
        TOKEN_ADJUST_PRIVILEGES, TOKEN_PRIVILEGES, TOKEN_QUERY,
    },
    System::{
        Registry::{
            HKEY_LOCAL_MACHINE, KEY_QUERY_VALUE, KEY_SET_VALUE, REG_SZ, RegCloseKey, RegOpenKeyExW,
            RegSetValueExW,
        },
        SystemInformation::{
            GetSystemTimeAdjustment, GetSystemTimeAdjustmentPrecise, SetSystemTime,
            SetSystemTimeAdjustment, SetSystemTimeAdjustmentPrecise,
        },
        Threading::{GetCurrentProcess, OpenProcessToken},
    },
};
use windows::core::w;

pub const NTP_PORT: u16 = 123;
pub const NTP_EPOCH_UNIX_OFFSET: i64 = 2_208_988_800;

static SYSTEM_TIME_PRIVILEGE_GRANTED: AtomicBool = AtomicBool::new(false);

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
    pub corrected: bool,
    pub method: String,
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

pub fn perform_sync(config: &AppConfig, config_path: &Path) -> Result<SyncReport> {
    let _ = apply_windows_time_policy(config.disable_win32_time);

    let client = Client::builder()
        .timeout(Duration::from_millis(config.network_timeout_ms.max(1)))
        .build()
        .map_err(|e| AppError::msg(format!("build HTTP client: {e}")))?;

    let mut errors = Vec::new();
    for host in collect_candidates(config) {
        if !request_allowed(config, host.request_type) {
            continue;
        }

        match fetch_time(config, &client, &host) {
            Ok(remote_utc) => {
                let local_before = Local::now();
                let adjusted_utc = remote_utc
                    + chrono::Duration::milliseconds(config.offset_ms as i64);
                let now_utc = Utc::now();
                let deviation_ms = (adjusted_utc - now_utc).num_milliseconds().unsigned_abs();

                if deviation_ms <= config.deviation_offset_ms {
                    return Ok(SyncReport {
                        server: host.name,
                        request_type: host.request_type,
                        remote_utc,
                        applied_utc: adjusted_utc,
                        local_before,
                        deviation_ms,
                        corrected: false,
                        method: "skip-within-deviation".to_string(),
                    });
                }

                let method = match config.sync_mode {
                    SyncMode::Off => {
                        return Ok(SyncReport {
                            server: host.name,
                            request_type: host.request_type,
                            remote_utc,
                            applied_utc: adjusted_utc,
                            local_before,
                            deviation_ms,
                            corrected: false,
                            method: "disabled".to_string(),
                        });
                    }
                    SyncMode::Immediate => set_system_time_direct(adjusted_utc)?,
                    SyncMode::Slew => slew_system_time(adjusted_utc)?,
                };

                return Ok(SyncReport {
                    server: host.name,
                    request_type: host.request_type,
                    remote_utc,
                    applied_utc: adjusted_utc,
                    local_before,
                    deviation_ms,
                    corrected: true,
                    method,
                });
            }
            Err(err) => {
                errors.push(format!("{}: {err}", host.name));
            }
        }
    }

    let config_note = format!(
        "config={}, hosts={}",
        config_path.display(),
        config.hosts.len()
    );
    Err(AppError::msg(format!(
        "all network time sources failed ({config_note}): {}",
        errors.join(" | ")
    )))
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

fn ensure_system_time_privilege() -> Result<()> {
    if SYSTEM_TIME_PRIVILEGE_GRANTED.load(Ordering::Relaxed) {
        return Ok(());
    }

    unsafe {
        let mut token = HANDLE::default();
        OpenProcessToken(
            GetCurrentProcess(),
            TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY,
            &mut token,
        )?;

        let mut luid = LUID::default();
        LookupPrivilegeValueW(None, w!("SeSystemtimePrivilege"), &mut luid)?;

        let token_privileges = TOKEN_PRIVILEGES {
            PrivilegeCount: 1,
            Privileges: [LUID_AND_ATTRIBUTES {
                Luid: luid,
                Attributes: SE_PRIVILEGE_ENABLED,
            }],
        };

        AdjustTokenPrivileges(token, false, Some(&token_privileges), 0, None, None)?;
        CloseHandle(token)?;
    }

    SYSTEM_TIME_PRIVILEGE_GRANTED.store(true, Ordering::Relaxed);
    Ok(())
}

fn set_system_time_direct(time: DateTime<Utc>) -> Result<String> {
    ensure_system_time_privilege()?;
    unsafe {
        let system_time = utc_to_system_time(time);
        SetSystemTime(&system_time)?;
    }
    Ok("direct".to_string())
}

fn slew_system_time(target: DateTime<Utc>) -> Result<String> {
    ensure_system_time_privilege()?;
    unsafe {
        let mut p_adj = 0u64;
        let mut p_inc = 0u64;
        let mut p_dis = windows::core::BOOL(0);

        // Try Precise API first (Windows 10 2004+)
        if GetSystemTimeAdjustmentPrecise(&mut p_adj, &mut p_inc, &mut p_dis).is_ok() {
            let now = Utc::now();
            let diff_ms = (target - now).num_milliseconds();
            if diff_ms.abs() < 2 {
                return Ok("slew-skipped-small".to_string());
            }

            let slew_rate = 0.1;
            let increment = p_inc as f64;
            let adj_delta = increment * slew_rate;
            let new_adj = if diff_ms > 0 { increment + adj_delta } else { increment - adj_delta };

            let distance_100ns = diff_ms.abs() as f64 * 10000.0;
            let interrupts_needed = distance_100ns / adj_delta;
            let seconds_to_wait = (interrupts_needed * increment) / 10_000_000.0;

            SetSystemTimeAdjustmentPrecise(new_adj.round() as u64, false)?;
            wait_for_slew(seconds_to_wait);
            SetSystemTimeAdjustmentPrecise(0, true)?;

            return Ok("slew-precise".to_string());
        }

        // Fallback to Legacy API
        let mut l_adj = 0u32;
        let mut l_inc = 0u32;
        let mut l_dis = windows::core::BOOL(0);
        GetSystemTimeAdjustment(&mut l_adj, &mut l_inc, &mut l_dis)?;

        let now = Utc::now();
        let diff_ms = (target - now).num_milliseconds();
        if diff_ms.abs() < 2 {
            return Ok("slew-skipped-small".to_string());
        }

        let slew_rate = 0.1;
        let increment = l_inc as f64;
        let adj_delta = increment * slew_rate;
        let new_adj = if diff_ms > 0 { increment + adj_delta } else { increment - adj_delta };

        let distance_100ns = diff_ms.abs() as f64 * 10000.0;
        let interrupts_needed = distance_100ns / adj_delta;
        let seconds_to_wait = (interrupts_needed * increment) / 10_000_000.0;

        SetSystemTimeAdjustment(new_adj.round() as u32, false)?;
        wait_for_slew(seconds_to_wait);
        SetSystemTimeAdjustment(0, true)?;

        Ok("slew-legacy".to_string())
    }
}

fn wait_for_slew(seconds: f64) {
    let mut remaining = Duration::from_secs_f64(seconds);
    while remaining > Duration::from_millis(1) {
        let chunk = remaining.min(Duration::from_millis(500));
        std::thread::sleep(chunk);
        remaining = remaining.saturating_sub(chunk);
    }
}

fn utc_to_system_time(time: DateTime<Utc>) -> SYSTEMTIME {
    SYSTEMTIME {
        wYear: time.year() as u16,
        wMonth: time.month() as u16,
        wDayOfWeek: time.weekday().num_days_from_sunday() as u16,
        wDay: time.day() as u16,
        wHour: time.hour() as u16,
        wMinute: time.minute() as u16,
        wSecond: time.second() as u16,
        wMilliseconds: time.timestamp_subsec_millis() as u16,
    }
}

pub fn apply_windows_time_policy(disable_win32_time: bool) -> Result<()> {
    unsafe {
        let mut key = Default::default();
        RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            w!("SYSTEM\\CurrentControlSet\\Services\\W32Time\\Parameters"),
            None,
            KEY_SET_VALUE | KEY_QUERY_VALUE,
            &mut key,
        )
        .ok()?;

        let value = if disable_win32_time { "NoSync" } else { "NTP" };
        let data: Vec<u16> = value.encode_utf16().chain(std::iter::once(0)).collect();
        let bytes = std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len() * 2);

        RegSetValueExW(key, w!("Type"), None, REG_SZ, Some(bytes)).ok()?;
        let _ = RegCloseKey(key);
    }
    Ok(())
}

fn request_allowed(config: &AppConfig, request_type: RequestType) -> bool {
    match config.agreement {
        Agreement::NtpOnly => request_type == RequestType::Ntp,
        Agreement::HttpOnly => request_type == RequestType::Http || request_type == RequestType::Https,
        Agreement::Mixed => true,
    }
}

fn collect_candidates(config: &AppConfig) -> Vec<HostCandidate> {
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
