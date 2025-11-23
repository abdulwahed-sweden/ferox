# Installation Guide

## Prerequisites

### Required Software

- **Rust**: 1.75+ (install via [rustup](https://rustup.rs/))
- **Node.js**: 20+ (for frontend development)
- **npm**: 10+ (comes with Node.js)

### Platform-Specific Requirements

#### macOS
```bash
# Install Xcode Command Line Tools
xcode-select --install

# Install Homebrew dependencies
brew install pkg-config
```

#### Linux (Ubuntu/Debian)
```bash
sudo apt update
sudo apt install -y build-essential libssl-dev pkg-config \
    libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev
```

#### Windows
- Visual Studio Build Tools 2019+ with C++ workload
- WebView2 Runtime (usually pre-installed on Windows 10/11)

## Quick Start

### 1. Clone Repository
```bash
git clone https://github.com/your-org/ferox.git
cd ferox
```

### 2. Build Ferox Core
```bash
cargo build --release
```

### 3. Build Ferox Desktop
```bash
cd ferox-desktop
npm install
cargo tauri build
```

The built application will be in `ferox-desktop/src-tauri/target/release/`.

## Development Setup

### Run Desktop App (Development)
```bash
cd ferox-desktop
npm install
cargo tauri dev
```

### Run Tests
```bash
# Rust tests
cargo test --workspace

# Frontend tests
cd ferox-desktop
npm test
```

### Build Documentation
```bash
cargo doc --workspace --no-deps
```

## Configuration

### Security Configuration
Copy and customize the security configuration:
```bash
cp ferox_security.toml.example ferox_security.toml
# Edit ferox_security.toml with your settings
```

### Theme Configuration
Customize the UI theme in `ferox_theme.toml`.

## Verification

After installation, verify everything works:
```bash
# Check Rust build
cargo check --workspace

# Check frontend build
cd ferox-desktop && npm run build

# Run all tests
cargo test --workspace
cd ferox-desktop && npm test
```

## Troubleshooting

See [Troubleshooting Guide](user-guide/troubleshooting.md) for common issues.
