# Rchronos (WINtp)

[![License: MIT/Apache-2.0](https://img.shields.io/badge/License-MIT%2FApache--2.0-blue.svg)](#license)
[![Platform: Windows](https://img.shields.io/badge/Platform-Windows-0078d7.svg)](#)

**Rchronos** is a high-precision, industrial-grade Windows time synchronization service written in Rust. It serves as a modern, transparent, and highly configurable alternative to the built-in Windows Time service (`W32Time`).

## Features

- **High Precision**: Leverages Windows APIs like `SetSystemTimeAdjustmentPrecise` for smooth clock slewing and minimal deviation.
- **Dual Protocol Support**: Fetch time via traditional **NTP** (UDP) or **HTTP/HTTPS** (via `Date` headers) for environments where NTP might be blocked.
- **Embedded Web Dashboard**: A reactive, modern management interface built with [Silex](https://github.com/shaogme/silex) (Rust WASM) baked directly into the binary.
- **Live Configuration**: Edit TOML settings and apply them instantly via the web UI without restarting the service.
- **Windows Integration**: Runs as a standard Windows service with robust event logging and panic reporting.
- **Customizable Logic**: Configure host priorities, offsets, and sync intervals to fit your network topology.

## Project Structure

- **`service`**: The core Windows service backend (Axum web server + Sync engine).
- **`web`**: Reactive WASM frontend for the control center.
- **`shared`**: Common data structures and configuration logic.

## Getting Started

### Prerequisites

- **Rust**: stable toolchain (2024 edition).
- **Trunk**: For building the WASM web interface (`cargo install trunk`).
- **Windows**: Required for service integration and precision time APIs.

### Building

Rchronos uses a custom `build.rs` in the service crate to automatically compile the WASM frontend and embed it into the final executable.

```bash
# Clone the repository
git clone https://github.com/shaogme/rchronos.git
cd rchronos

# Build the service (and the web frontend automatically)
cargo build --release
```

The resulting binary will be located at `target/release/rchronos-service.exe`.

### Installation as a Service

You can install `rchronos-service.exe` using the standard Windows `sc` command or tools like [NSSM](https://nssm.cc/).

```powershell
# Create the service
sc.exe create Rchronos binPath= "C:\path\to\rchronos-service.exe" start= auto

# Start the service
sc.exe start Rchronos
```

## Configuration

The service looks for a `.toml` file with the same name as the executable (e.g., `rchronos.toml`) in its directory.

```toml
sync_mode = "slew"
offset_ms = 0
deviation_offset_ms = 0
disable_win32_time = false
delay_ms = 3600000
timeout_ms = 30000
network_timeout_ms = 5000
agreement = "mixed"
user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/144.0.0.0 Safari/537.36 Edg/144.0.0.0"
web_port = 8081
max_log_lines = 200
service_name = "Rchronos"

[hosts."ntp.aliyun.com"]
request_type = "ntp"
priority = 0
enabled = true

[hosts."ntp.tencent.com"]
request_type = "ntp"
priority = 0
enabled = true

[hosts."rhel.pool.ntp.org"]
request_type = "ntp"
priority = 0
enabled = true

[hosts."time.asia.apple.com"]
request_type = "ntp"
priority = 0
enabled = true

[hosts."time.cloudflare.com"]
request_type = "ntp"
priority = 0
enabled = true

[hosts."www.163.com"]
request_type = "http"
priority = 1
enabled = true

[hosts."www.baidu.com"]
request_type = "http"
priority = 1
enabled = true

[hosts."www.qq.com"]
request_type = "http"
priority = 1
enabled = true
```

## Web Control Center

Once running, the dashboard is accessible at `http://127.0.0.1:8081/`. 

- **Overview**: Real-time metrics including status, last sync result, and deviation.
- **Config**: A live TOML editor with persistent draft support.
- **Logs**: Real-time stream of service events and synchronization reports.
- **Control**: Buttons to manually trigger sync, reload config, or gracefully stop the service.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
