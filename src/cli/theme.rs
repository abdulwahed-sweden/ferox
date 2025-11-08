use colored::*;

pub struct Theme;

impl Theme {
    /// Display the main Ferox banner
    pub fn banner() {
        println!("{}", r#"
   ╔═══════════════════════════════════════════════════════════════════╗
   ║                                                                   ║
   ║    ███████╗███████╗██████╗  ██████╗ ██╗  ██╗                     ║
   ║    ██╔════╝██╔════╝██╔══██╗██╔═══██╗╚██╗██╔╝                     ║
   ║    █████╗  █████╗  ██████╔╝██║   ██║ ╚███╔╝                      ║
   ║    ██╔══╝  ██╔══╝  ██╔══██╗██║   ██║ ██╔██╗                      ║
   ║    ██║     ███████╗██║  ██║╚██████╔╝██╔╝ ██╗                     ║
   ║    ╚═╝     ╚══════╝╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝                     ║
   ║                                                                   ║
   ║          🦊  FEROCIOUS SECURITY FRAMEWORK  🦊                     ║
   ║                                                                   ║
   ╚═══════════════════════════════════════════════════════════════════╝
"#.bright_red());
        
        println!("    {}   Ferox Framework {}", "🦊".bright_red(), "v2.0.0".bright_yellow());
        println!("    {}   Fast. Fierce. Fearless.\n", "⚡".bright_yellow());
    }

    /// Success message
    pub fn success(msg: &str) {
        println!("{} {}", "✓".bright_green().bold(), msg.bright_white());
    }

    /// Error message
    pub fn error(msg: &str) {
        println!("{} {}", "✗".bright_red().bold(), msg.bright_white());
    }

    /// Warning message
    pub fn warning(msg: &str) {
        println!("{} {}", "⚠".bright_yellow().bold(), msg.bright_white());
    }

    /// Info message
    pub fn info(msg: &str) {
        println!("{} {}", "ℹ".bright_blue().bold(), msg.bright_white());
    }

    /// Module header
    pub fn module_header(name: &str) {
        println!("\n{}", "═".repeat(70).bright_blue());
        println!("  {} {}", "📦".bright_blue(), name.bright_white().bold());
        println!("{}", "═".repeat(70).bright_blue());
    }

    /// Section header
    pub fn section(title: &str) {
        println!("\n  {}", format!("──────────────────[ {} ]──────────────────", title)
            .bright_cyan());
    }

    /// Prompt with Ferox branding
    pub fn prompt(context: &str) -> String {
        if context.is_empty() {
            format!("{} ", "ferox>".bright_red().bold())
        } else {
            format!("{} ", format!("ferox({})>", context).bright_red().bold())
        }
    }

    /// Command help
    pub fn command_help(cmd: &str, desc: &str) {
        println!("    {}  →  {}", 
            cmd.bright_green().bold(),
            desc.bright_white()
        );
    }

    /// Status indicator
    pub fn status(status: &str, message: &str) {
        let icon = match status {
            "ready" => "🟢",
            "running" => "🟡",
            "error" => "🔴",
            _ => "⚪",
        };
        println!("  {}  {}: {}", icon, status.to_uppercase().bright_yellow(), message.bright_white());
    }

    /// Progress bar style
    pub fn progress_style() -> indicatif::ProgressStyle {
        indicatif::ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("█▓▒░")
    }

    /// Spinner style
    pub fn spinner_style() -> indicatif::ProgressStyle {
        indicatif::ProgressStyle::default_spinner()
            .template("{spinner:.red} {msg}")
            .unwrap()
            .tick_strings(&["🦊", "🔥", "⚡", "💥", "✨", "🎯", "🚀", "⚔️"])
    }

    /// Ferox-specific ascii art
    pub fn fox_art() {
        println!("{}", r#"
              /\   /\
             //\\_//\\     ____
             \_     _/    /   /
              / * * \    /^^^]
              \_\O/_/    [   ]
               /   \_    [   /
               \     \_  /  /
                [ [ /  \/ _/
               _[ [ \  /_/
              [  [  [  [
        "#.bright_red());
    }
}
