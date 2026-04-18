use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/144.0.0.0 Safari/537.36 Edg/144.0.0.0";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum RequestType {
    #[default]
    Ntp,
    Http,
    Https,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum SyncMode {
    #[default]
    Off,
    Immediate,
    Slew,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum Agreement {
    NtpOnly,
    HttpOnly,
    #[default]
    Mixed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HostConfig {
    pub request_type: RequestType,
    pub priority: u32,
    pub enabled: bool,
}

impl Default for HostConfig {
    fn default() -> Self {
        Self {
            request_type: RequestType::default(),
            priority: 0,
            enabled: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub sync_mode: SyncMode,
    pub offset_ms: u64,
    pub deviation_offset_ms: u64,
    pub disable_win32_time: bool,
    pub delay_ms: u64,
    pub timeout_ms: u64,
    pub network_timeout_ms: u64,
    pub agreement: Agreement,
    pub user_agent: String,
    pub hosts: BTreeMap<String, HostConfig>,
    pub web_port: u16,
    pub max_log_lines: usize,
    pub service_name: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        let mut hosts = BTreeMap::new();
        hosts.insert(
            "ntp.tencent.com".to_string(),
            HostConfig {
                request_type: RequestType::Ntp,
                priority: 0,
                enabled: true,
            },
        );
        hosts.insert(
            "ntp.aliyun.com".to_string(),
            HostConfig {
                request_type: RequestType::Ntp,
                priority: 0,
                enabled: true,
            },
        );
        hosts.insert(
            "time.cloudflare.com".to_string(),
            HostConfig {
                request_type: RequestType::Ntp,
                priority: 0,
                enabled: true,
            },
        );
        hosts.insert(
            "time.asia.apple.com".to_string(),
            HostConfig {
                request_type: RequestType::Ntp,
                priority: 0,
                enabled: true,
            },
        );
        hosts.insert(
            "rhel.pool.ntp.org".to_string(),
            HostConfig {
                request_type: RequestType::Ntp,
                priority: 0,
                enabled: true,
            },
        );
        hosts.insert(
            "www.baidu.com".to_string(),
            HostConfig {
                request_type: RequestType::Http,
                priority: 1,
                enabled: true,
            },
        );
        hosts.insert(
            "www.qq.com".to_string(),
            HostConfig {
                request_type: RequestType::Http,
                priority: 1,
                enabled: true,
            },
        );
        hosts.insert(
            "www.163.com".to_string(),
            HostConfig {
                request_type: RequestType::Http,
                priority: 1,
                enabled: true,
            },
        );

        Self {
            sync_mode: SyncMode::default(),
            offset_ms: 0,
            deviation_offset_ms: 0,
            disable_win32_time: false,
            delay_ms: 3_600_000,
            timeout_ms: 30_000,
            network_timeout_ms: 5_000,
            agreement: Agreement::default(),
            user_agent: DEFAULT_USER_AGENT.to_string(),
            hosts,
            web_port: 8081,
            max_log_lines: 200,
            service_name: "Rchronos".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RuntimeSnapshot {
    pub config: AppConfig,
    pub logs: Vec<String>,
    pub status: String,
    pub last_result: String,
    pub syncing: bool,
    pub config_path: String,
}

pub fn format_ms_adaptive(ms: u64) -> String {
    if ms == 0 {
        return "0ms".to_string();
    }

    let mut remaining = ms;
    let mut parts = Vec::new();

    if remaining >= 3_600_000 {
        parts.push(format!("{}h", remaining / 3_600_000));
        remaining %= 3_600_000;
    }
    if remaining >= 60_000 {
        parts.push(format!("{}m", remaining / 60_000));
        remaining %= 60_000;
    }
    if remaining >= 1_000 {
        parts.push(format!("{}s", remaining / 1_000));
        remaining %= 1_000;
    }
    if remaining > 0 {
        parts.push(format!("{}ms", remaining));
    }

    parts.join(" ")
}
