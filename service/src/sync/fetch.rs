use std::{
    net::UdpSocket,
    time::Duration,
};

use crate::config::AppConfig;
use crate::{AppError, Result};
use chrono::{DateTime, TimeZone, Utc};
use httpdate::parse_http_date;
use rchronos_shared::RequestType;
use reqwest::blocking::Client;

use super::HostCandidate;

pub const NTP_PORT: u16 = 123;
pub const NTP_EPOCH_UNIX_OFFSET: i64 = 2_208_988_800;

/// Represents an NTP protocol packet as defined in RFC 5905.
/// Provides safe serialization and deserialization routines.
#[derive(Debug, Clone, PartialEq, Eq)]
struct NtpPacket {
    li: u8,
    vn: u8,
    mode: u8,
    stratum: u8,
    poll: i8,
    precision: i8,
    root_delay: u32,
    root_dispersion: u32,
    reference_id: u32,
    reference_timestamp: u64,
    originate_timestamp: u64,
    receive_timestamp: u64,
    transmit_timestamp: u64,
}

impl NtpPacket {
    /// Creates a default client request packet with the transmit timestamp set to now.
    fn new_client_request(now_utc: DateTime<Utc>) -> Self {
        let unix_seconds = now_utc.timestamp() + NTP_EPOCH_UNIX_OFFSET;
        let nanos = now_utc.timestamp_subsec_nanos() as u64;
        let fraction = ((nanos << 32) / 1_000_000_000) as u32;
        let transmit_timestamp = ((unix_seconds as u64) << 32) | (fraction as u64);

        Self {
            li: 0,
            vn: 3, // NTP Version 3
            mode: 3, // Client Mode
            stratum: 0,
            poll: 0,
            precision: 0,
            root_delay: 0,
            root_dispersion: 0,
            reference_id: 0,
            reference_timestamp: 0,
            originate_timestamp: 0,
            receive_timestamp: 0,
            transmit_timestamp,
        }
    }

    /// Serializes the packet into standard 48-byte buffer.
    fn to_bytes(&self) -> [u8; 48] {
        let mut bytes = [0u8; 48];
        bytes[0] = (self.li << 6) | (self.vn << 3) | self.mode;
        bytes[1] = self.stratum;
        bytes[2] = self.poll as u8;
        bytes[3] = self.precision as u8;
        bytes[4..8].copy_from_slice(&self.root_delay.to_be_bytes());
        bytes[8..12].copy_from_slice(&self.root_dispersion.to_be_bytes());
        bytes[12..16].copy_from_slice(&self.reference_id.to_be_bytes());
        bytes[16..24].copy_from_slice(&self.reference_timestamp.to_be_bytes());
        bytes[24..32].copy_from_slice(&self.originate_timestamp.to_be_bytes());
        bytes[32..40].copy_from_slice(&self.receive_timestamp.to_be_bytes());
        bytes[40..48].copy_from_slice(&self.transmit_timestamp.to_be_bytes());
        bytes
    }

    /// Deserializes a standard 48-byte NTP response packet.
    fn from_bytes(bytes: &[u8; 48]) -> Result<Self> {
        let li = (bytes[0] >> 6) & 0x03;
        let vn = (bytes[0] >> 3) & 0x07;
        let mode = bytes[0] & 0x07;
        let stratum = bytes[1];
        let poll = bytes[2] as i8;
        let precision = bytes[3] as i8;

        let root_delay = u32::from_be_bytes(bytes[4..8].try_into().unwrap());
        let root_dispersion = u32::from_be_bytes(bytes[8..12].try_into().unwrap());
        let reference_id = u32::from_be_bytes(bytes[12..16].try_into().unwrap());
        let reference_timestamp = u64::from_be_bytes(bytes[16..24].try_into().unwrap());
        let originate_timestamp = u64::from_be_bytes(bytes[24..32].try_into().unwrap());
        let receive_timestamp = u64::from_be_bytes(bytes[32..40].try_into().unwrap());
        let transmit_timestamp = u64::from_be_bytes(bytes[40..48].try_into().unwrap());

        Ok(Self {
            li,
            vn,
            mode,
            stratum,
            poll,
            precision,
            root_delay,
            root_dispersion,
            reference_id,
            reference_timestamp,
            originate_timestamp,
            receive_timestamp,
            transmit_timestamp,
        })
    }

    /// Extracts the server's transmit timestamp as a `DateTime<Utc>`.
    fn parse_transmit_time(&self) -> Result<DateTime<Utc>> {
        let seconds = (self.transmit_timestamp >> 32) as i64;
        let fraction = (self.transmit_timestamp & 0xFFFF_FFFF) as i64;
        let unix_seconds = seconds - NTP_EPOCH_UNIX_OFFSET;
        let nanos = ((fraction as i128 * 1_000_000_000i128) >> 32) as u32;

        Utc.timestamp_opt(unix_seconds, nanos)
            .single()
            .ok_or_else(|| AppError::msg("decode NTP time"))
    }
}

/// The core entry point for fetching remote network time.
/// Private to parent module `sync.rs`.
pub(crate) fn fetch_time(
    config: &AppConfig,
    client: &Client,
    host: &HostCandidate,
) -> Result<DateTime<Utc>> {
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

/// Fetch network time via UDP NTP protocol.
fn fetch_ntp_time(host: &str, timeout_ms: u64) -> Result<DateTime<Utc>> {
    let address = format!("{host}:{NTP_PORT}");
    let socket = UdpSocket::bind("0.0.0.0:0")
        .map_err(|e| AppError::msg(format!("bind UDP socket: {e}")))?;
    let timeout = Duration::from_millis(timeout_ms.max(1));
    socket
        .set_read_timeout(Some(timeout))
        .map_err(|e| AppError::msg(format!("set UDP read timeout: {e}")))?;
    socket
        .set_write_timeout(Some(timeout))
        .map_err(|e| AppError::msg(format!("set UDP write timeout: {e}")))?;

    let request = NtpPacket::new_client_request(Utc::now());
    let request_bytes = request.to_bytes();

    socket
        .send_to(&request_bytes, &address)
        .map_err(|e| AppError::msg(format!("send NTP packet to {address}: {e}")))?;

    let mut response_bytes = [0_u8; 48];
    socket
        .recv_from(&mut response_bytes)
        .map_err(|e| AppError::msg(format!("receive NTP packet from {address}: {e}")))?;

    let response = NtpPacket::from_bytes(&response_bytes)?;
    
    // Validate server response mode (should be Server = 4)
    if response.mode != 4 {
        return Err(AppError::msg(format!(
            "invalid NTP server mode: {}, expected 4",
            response.mode
        )));
    }

    response.parse_transmit_time()
}

/// Fetch network time via HTTP/HTTPS protocols.
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

    let date_str = header
        .to_str()
        .map_err(|e| AppError::msg(format!("invalid Date header: {e}")))?;

    let date = parse_http_date(date_str)
        .map_err(|e| AppError::msg(format!("parse Date header: {e}")))?;

    Ok(DateTime::<Utc>::from(date))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ntp_packet_serialization_roundtrip() {
        let packet = NtpPacket {
            li: 1,
            vn: 4,
            mode: 4,
            stratum: 2,
            poll: 6,
            precision: -18,
            root_delay: 1024,
            root_dispersion: 2048,
            reference_id: 123456,
            reference_timestamp: 0xAAAA_BBBB_CCCC_DDDD,
            originate_timestamp: 0x1111_2222_3333_4444,
            receive_timestamp: 0x5555_6666_7777_8888,
            transmit_timestamp: 0x9999_AAAA_BBBB_CCCC,
        };

        let bytes = packet.to_bytes();
        let decoded = NtpPacket::from_bytes(&bytes).unwrap();
        assert_eq!(packet, decoded);
    }

    #[test]
    fn test_ntp_packet_new_client_request() {
        let now = Utc.with_ymd_and_hms(2026, 5, 24, 22, 0, 0).unwrap();
        let packet = NtpPacket::new_client_request(now);

        assert_eq!(packet.li, 0);
        assert_eq!(packet.vn, 3);
        assert_eq!(packet.mode, 3);
        assert_eq!(packet.stratum, 0);

        let decoded_time = packet.parse_transmit_time().unwrap();
        // Since nanos is 0, the time should be exactly equal
        assert_eq!(decoded_time, now);
    }

    #[test]
    fn test_ntp_packet_time_boundary_handling() {
        // Test an exact epoch boundary transition
        let now = Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap();
        let packet = NtpPacket::new_client_request(now);
        let decoded_time = packet.parse_transmit_time().unwrap();
        assert_eq!(decoded_time, now);

        // Test a date far in the future, just before the standard 32-bit NTP Era 0 overflow boundary (2036-02-07 06:28:16 UTC)
        let future_time = Utc.with_ymd_and_hms(2035, 12, 31, 23, 59, 59).unwrap();
        let packet_future = NtpPacket::new_client_request(future_time);
        let decoded_future = packet_future.parse_transmit_time().unwrap();
        assert_eq!(decoded_future, future_time);
    }

    #[test]
    fn test_ntp_packet_fractional_precision() {
        // Test sub-second fraction accuracy (nanoseconds level)
        let original_time = Utc::now();
        let packet = NtpPacket::new_client_request(original_time);
        let decoded_time = packet.parse_transmit_time().unwrap();

        // High-precision fractional seconds: delta must be within 1 microsecond (1000 nanoseconds)
        let diff_nanos = (original_time - decoded_time).num_nanoseconds().unwrap().abs();
        assert!(
            diff_nanos <= 1000,
            "NTP conversion accuracy lost sub-second precision. Delta: {}ns",
            diff_nanos
        );
    }

    #[test]
    fn test_fetch_time_allowed_types() {
        // Test config structures or mock scenarios
        let client = Client::builder().build().unwrap();
        
        // NTP server that is fast and reliable for offline detection check
        let host_candidate_invalid = HostCandidate {
            name: "invalid.hostname.that.does.not.exist.rchronos".to_string(),
            request_type: RequestType::Ntp,
            priority: 0,
        };
        
        let config = AppConfig {
            network_timeout_ms: 100,
            ..Default::default()
        };

        // Should return a standard connection error gracefully rather than panicking
        let result = fetch_time(&config, &client, &host_candidate_invalid);
        assert!(result.is_err());
    }

    #[test]
    fn test_fetch_time_ntp_real() {
        let client = Client::builder().build().unwrap();
        let host = HostCandidate {
            name: "ntp.aliyun.com".to_string(),
            request_type: RequestType::Ntp,
            priority: 0,
        };
        let config = AppConfig {
            network_timeout_ms: 5000,
            ..Default::default()
        };

        match fetch_time(&config, &client, &host) {
            Ok(time) => {
                let now = Utc::now();
                let diff_secs = (time - now).num_seconds().abs();
                assert!(
                    diff_secs < 10,
                    "NTP 真实时间与本地时间相差过大，可能本地时钟未同步: {}s",
                    diff_secs
                );
                println!("NTP E2E Test Success: Server Time = {}, Local Time = {}", time, now);
            }
            Err(e) => {
                println!("Warning: 真实 NTP 端到端测试因网络/超时被跳过: {e}");
            }
        }
    }

    #[test]
    fn test_fetch_time_http_real() {
        let client = Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();
        let host = HostCandidate {
            name: "www.baidu.com".to_string(),
            request_type: RequestType::Http,
            priority: 0,
        };
        let config = AppConfig {
            user_agent: crate::config::AppConfig::default().user_agent,
            ..Default::default()
        };

        match fetch_time(&config, &client, &host) {
            Ok(time) => {
                let now = Utc::now();
                let diff_secs = (time - now).num_seconds().abs();
                assert!(
                    diff_secs < 10,
                    "HTTP 真实时间与本地时间相差过大，可能本地时钟未同步: {}s",
                    diff_secs
                );
                println!("HTTP E2E Test Success: Server Time = {}, Local Time = {}", time, now);
            }
            Err(e) => {
                println!("Warning: 真实 HTTP 端到端测试因网络/超时被跳过: {e}");
            }
        }
    }

    #[test]
    fn test_fetch_time_https_real() {
        let client = Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();
        let host = HostCandidate {
            name: "www.baidu.com".to_string(),
            request_type: RequestType::Https,
            priority: 0,
        };
        let config = AppConfig {
            user_agent: crate::config::AppConfig::default().user_agent,
            ..Default::default()
        };

        match fetch_time(&config, &client, &host) {
            Ok(time) => {
                let now = Utc::now();
                let diff_secs = (time - now).num_seconds().abs();
                assert!(
                    diff_secs < 10,
                    "HTTPS 真实时间与本地时间相差过大，可能本地时钟未同步: {}s",
                    diff_secs
                );
                println!("HTTPS E2E Test Success: Server Time = {}, Local Time = {}", time, now);
            }
            Err(e) => {
                println!("Warning: 真实 HTTPS 端到端测试因网络/超时被跳过: {e}");
            }
        }
    }
}
