use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/144.0.0.0 Safari/537.36 Edg/144.0.0.0";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RequestType {
    Ntp = 0,
    Http = 1,
    Https = 2,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HostConfig {
    pub request_type: u8,
    pub priority: u32,
    pub enabled: bool,
}

impl Default for HostConfig {
    fn default() -> Self {
        Self {
            request_type: RequestType::Ntp as u8,
            priority: 0,
            enabled: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub sync_mode: u8,
    pub high_precision_supported: bool,
    pub offset_seconds: f64,
    pub deviation_offset_seconds: f64,
    pub verbose: bool,
    pub disable_win32_time: bool,
    pub delay_seconds: f64,
    pub timeout_ms: f64,
    pub network_timeout_ms: f64,
    pub agreement: u8,
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
                request_type: RequestType::Ntp as u8,
                priority: 0,
                enabled: true,
            },
        );
        hosts.insert(
            "ntp.aliyun.com".to_string(),
            HostConfig {
                request_type: RequestType::Ntp as u8,
                priority: 0,
                enabled: true,
            },
        );
        hosts.insert(
            "time.cloudflare.com".to_string(),
            HostConfig {
                request_type: RequestType::Ntp as u8,
                priority: 0,
                enabled: true,
            },
        );
        hosts.insert(
            "time.asia.apple.com".to_string(),
            HostConfig {
                request_type: RequestType::Ntp as u8,
                priority: 0,
                enabled: true,
            },
        );
        hosts.insert(
            "rhel.pool.ntp.org".to_string(),
            HostConfig {
                request_type: RequestType::Ntp as u8,
                priority: 0,
                enabled: true,
            },
        );
        hosts.insert(
            "www.baidu.com".to_string(),
            HostConfig {
                request_type: RequestType::Http as u8,
                priority: 1,
                enabled: true,
            },
        );
        hosts.insert(
            "www.qq.com".to_string(),
            HostConfig {
                request_type: RequestType::Http as u8,
                priority: 1,
                enabled: true,
            },
        );
        hosts.insert(
            "www.163.com".to_string(),
            HostConfig {
                request_type: RequestType::Http as u8,
                priority: 1,
                enabled: true,
            },
        );

        Self {
            sync_mode: 0,
            high_precision_supported: false,
            offset_seconds: 0.0,
            deviation_offset_seconds: 0.0,
            verbose: false,
            disable_win32_time: false,
            delay_seconds: 3600.0,
            timeout_ms: 30_000.0,
            network_timeout_ms: 5_000.0,
            agreement: 2,
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
