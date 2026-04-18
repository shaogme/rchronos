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
sync_mode = 0             # 0: Direct, 2: Precise, 3: Legacy
delay_seconds = 3600.0    # Sync interval
network_timeout_ms = 5000.0

[hosts]
"ntp.tencent.com" = { request_type = 0, priority = 0, enabled = true }
"www.baidu.com"   = { request_type = 1, priority = 1, enabled = true }
"www.google.com"  = { request_type = 2, priority = 1, enabled = true }
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
