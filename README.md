# Cosmic Niri Session - README

## Overview

Cosmic Niri Session is a lightweight system management utility that provides cross-distro service control capabilities. It supports both **systemd** and **OpenRC** init systems with automatic detection.

## Features
- 📦 **Multi-backend Support**: Systemd + OpenRC via Cargo features
- 🔧 **Zero-config Detection**: Automatically chooses appropriate backend based on available tools
- 💻 **Cross-platform**: Works across all Linux distributions using systemd or OpenRC
- 🚀 **Rust-powered**: Fast, safe, and efficient system operations

## Installation

### Via Cargo (Recommended)
```bash
cargo install cosmic-niri-session 
```

### Manual Build 
```bash
cd /path/to/cosmic-niri-session
cargo build --release 
```

## Usage Examples

### Environment Management
#### Set environment variables for current user
```bash
cosmic-niri-session set-env KEY=value
```

#### Systemd (when available)
```bash
cosmic-niri-session set-env KEY=value  # systemd backend
systemctl --user set-environment KEY=value
```

#### OpenRC (when available)
```bash
cosmic-niri-session set-env KEY=value  # openrc backend
env-update --user "KEY=value"
```

### Target Management
#### Start cosmic-session target
```bash
cosmic-niri-session start-target cosmic-session.target
```

#### Stop cosmic-session target (non-blocking)
```bash
cosmic-niri-session stop-target cosmic-session.target
```

### Service Control via Spawn
#### Run a process with PID monitoring
```bash
cosmic-niri-session spawn-service "my-process" [pid1 pid2 pid3]
```

#### OpenRC service spawning example
```bash
cosmic-niri-session spawn-service "rc-service-name" [pids...]
```

### PID-based Service Management
#### List active PIDs for a service
```bash
cosmic-niri-session get-pids my-process
```

## Development```

## Contributing

1. Fork the project
2. Create your feature branch (`git checkout -b my-feature`)
3. Make your changes and run tests (`cargo test`)
4. Commit your changes (`git commit -am "Add: new functionality"`)
5. Push to the branch (`git push origin my-feature`)
6. Open a pull request

## License

This project is licensed under GPL-3.0-only.

**Note**: This README assumes you have Rust and Cargo installed on your system.

