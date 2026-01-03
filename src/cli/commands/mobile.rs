//! Mobile Analysis CLI Commands
//!
//! CLI interface for Android APK and iOS IPA analysis, plus app reconnaissance.

use anyhow::{anyhow, Result};
use clap::{Args, Subcommand};
use colored::Colorize;
use std::collections::HashMap;
use std::path::Path;

use crate::cli::progress::{Spinner, SpinnerStyle};
use crate::cli::theme::Theme;
use crate::core::module::Module;
use crate::modules::mobile::{ApkAnalyzer, AppRecon, IpaAnalyzer};

/// Mobile analysis subcommands
#[derive(Subcommand, Debug, Clone)]
pub enum MobileCommands {
    /// Auto-detect and analyze APK or IPA file
    Analyze(AnalyzeArgs),

    /// Analyze Android APK file
    Apk(ApkArgs),

    /// Analyze iOS IPA file
    Ipa(IpaArgs),

    /// App store reconnaissance
    Recon(ReconArgs),

    /// Generate analysis report
    Report(ReportArgs),

    /// Check mobile analysis dependencies
    Check,
}

#[derive(Args, Debug, Clone)]
pub struct AnalyzeArgs {
    /// Path to APK or IPA file
    #[arg(value_name = "FILE")]
    pub path: String,

    /// Enable deep analysis (slower but more thorough)
    #[arg(short, long)]
    pub deep: bool,

    /// Output report to file (JSON format)
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<String>,

    /// Show verbose output
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Args, Debug, Clone)]
pub struct ApkArgs {
    /// Path to APK file
    #[arg(value_name = "FILE")]
    pub path: String,

    /// Enable deep analysis
    #[arg(short, long)]
    pub deep: bool,

    /// Check for hardcoded secrets
    #[arg(long, default_value = "true")]
    pub secrets: bool,

    /// Analyze native libraries (.so files)
    #[arg(long)]
    pub native: bool,

    /// Output report to file
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<String>,

    /// Show verbose output
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Args, Debug, Clone)]
pub struct IpaArgs {
    /// Path to IPA file
    #[arg(value_name = "FILE")]
    pub path: String,

    /// Enable deep analysis
    #[arg(short, long)]
    pub deep: bool,

    /// Check for hardcoded secrets
    #[arg(long, default_value = "true")]
    pub secrets: bool,

    /// Analyze embedded frameworks
    #[arg(long)]
    pub frameworks: bool,

    /// Output report to file
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<String>,

    /// Show verbose output
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Args, Debug, Clone)]
pub struct ReconArgs {
    /// App identifier (package name or bundle ID)
    #[arg(value_name = "APP_ID")]
    pub app_id: String,

    /// Target platform (android, ios, auto)
    #[arg(short, long, default_value = "auto")]
    pub platform: String,

    /// Enable deep reconnaissance
    #[arg(short, long)]
    pub deep: bool,

    /// Output report to file
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<String>,
}

#[derive(Args, Debug, Clone)]
pub struct ReportArgs {
    /// Path to APK or IPA file
    #[arg(value_name = "FILE")]
    pub path: String,

    /// Output file path
    #[arg(short, long, value_name = "FILE", default_value = "mobile_report.json")]
    pub output: String,

    /// Report format (json, html, pdf)
    #[arg(short, long, default_value = "json")]
    pub format: String,
}

/// Mobile command handler
pub struct MobileCommandHandler;

impl MobileCommandHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn describe() -> &'static str {
        "Mobile app security analysis (APK/IPA)"
    }

    pub fn print_usage() {
        Theme::section("Mobile Analysis CLI");
        Theme::command_help("ferox mobile analyze <file>", "Auto-detect and analyze APK/IPA");
        Theme::command_help("ferox mobile apk <file>", "Analyze Android APK file");
        Theme::command_help("ferox mobile ipa <file>", "Analyze iOS IPA file");
        Theme::command_help("ferox mobile recon <app_id>", "App store reconnaissance");
        Theme::command_help("ferox mobile report <file>", "Generate analysis report");
        Theme::command_help("ferox mobile check", "Check analysis dependencies");
    }

    pub async fn run(&self, command: MobileCommands) -> Result<()> {
        match command {
            MobileCommands::Analyze(args) => self.handle_analyze(args).await,
            MobileCommands::Apk(args) => self.handle_apk(args).await,
            MobileCommands::Ipa(args) => self.handle_ipa(args).await,
            MobileCommands::Recon(args) => self.handle_recon(args).await,
            MobileCommands::Report(args) => self.handle_report(args).await,
            MobileCommands::Check => self.handle_check(),
        }
    }

    /// Auto-detect file type and analyze
    async fn handle_analyze(&self, args: AnalyzeArgs) -> Result<()> {
        let path = &args.path;

        if !Path::new(path).exists() {
            return Err(anyhow!("File not found: {}", path));
        }

        if path.ends_with(".apk") {
            let apk_args = ApkArgs {
                path: path.clone(),
                deep: args.deep,
                secrets: true,
                native: false,
                output: args.output,
                verbose: args.verbose,
            };
            self.handle_apk(apk_args).await
        } else if path.ends_with(".ipa") {
            let ipa_args = IpaArgs {
                path: path.clone(),
                deep: args.deep,
                secrets: true,
                frameworks: false,
                output: args.output,
                verbose: args.verbose,
            };
            self.handle_ipa(ipa_args).await
        } else {
            Err(anyhow!(
                "Unknown file type. Expected .apk or .ipa: {}",
                path
            ))
        }
    }

    /// Handle APK analysis
    async fn handle_apk(&self, args: ApkArgs) -> Result<()> {
        let path = &args.path;

        if !Path::new(path).exists() {
            return Err(anyhow!("APK file not found: {}", path));
        }

        // Print header
        self.print_analysis_header("APK", path);

        // Create and configure analyzer
        let mut analyzer = ApkAnalyzer::new();
        analyzer.set_option("APK_PATH", path)?;
        analyzer.set_option("DEEP_ANALYSIS", &args.deep.to_string())?;
        analyzer.set_option("CHECK_SECRETS", &args.secrets.to_string())?;
        analyzer.set_option("ANALYZE_NATIVE", &args.native.to_string())?;

        // Run with spinner
        let mut spinner = Spinner::new("Analyzing APK...")
            .with_style(SpinnerStyle::Dots)
            .with_color("cyan");
        spinner.start();

        let result = analyzer.run().await;
        spinner.stop();

        match result {
            Ok(module_result) => {
                self.display_apk_results(&module_result.data, args.verbose)?;

                // Save report if requested
                if let Some(output_path) = &args.output {
                    let json = serde_json::to_string_pretty(&module_result.data)?;
                    std::fs::write(output_path, json)?;
                    println!(
                        "\n{} Report saved: {}",
                        "💾".green(),
                        output_path.cyan()
                    );
                }

                Ok(())
            }
            Err(e) => {
                println!("{} Analysis failed: {}", "✗".red(), e);
                Err(e)
            }
        }
    }

    /// Handle IPA analysis
    async fn handle_ipa(&self, args: IpaArgs) -> Result<()> {
        let path = &args.path;

        if !Path::new(path).exists() {
            return Err(anyhow!("IPA file not found: {}", path));
        }

        // Print header
        self.print_analysis_header("IPA", path);

        // Create and configure analyzer
        let mut analyzer = IpaAnalyzer::new();
        analyzer.set_option("IPA_PATH", path)?;
        analyzer.set_option("DEEP_ANALYSIS", &args.deep.to_string())?;
        analyzer.set_option("CHECK_SECRETS", &args.secrets.to_string())?;
        analyzer.set_option("ANALYZE_FRAMEWORKS", &args.frameworks.to_string())?;

        // Run with spinner
        let mut spinner = Spinner::new("Analyzing IPA...")
            .with_style(SpinnerStyle::Dots)
            .with_color("cyan");
        spinner.start();

        let result = analyzer.run().await;
        spinner.stop();

        match result {
            Ok(module_result) => {
                self.display_ipa_results(&module_result.data, args.verbose)?;

                // Save report if requested
                if let Some(output_path) = &args.output {
                    let json = serde_json::to_string_pretty(&module_result.data)?;
                    std::fs::write(output_path, json)?;
                    println!(
                        "\n{} Report saved: {}",
                        "💾".green(),
                        output_path.cyan()
                    );
                }

                Ok(())
            }
            Err(e) => {
                println!("{} Analysis failed: {}", "✗".red(), e);
                Err(e)
            }
        }
    }

    /// Handle app reconnaissance
    async fn handle_recon(&self, args: ReconArgs) -> Result<()> {
        println!("{}", "━".repeat(60).dimmed());
        println!(
            "{} App Reconnaissance: {}",
            "🔍".cyan(),
            args.app_id.bright_white()
        );
        println!("{}", "━".repeat(60).dimmed());

        // Create and configure recon module
        let mut recon = AppRecon::new();
        recon.set_option("APP_ID", &args.app_id)?;
        recon.set_option("PLATFORM", &args.platform)?;
        recon.set_option("DEEP_RECON", &args.deep.to_string())?;

        // Run with spinner
        let mut spinner = Spinner::new("Gathering app information...")
            .with_style(SpinnerStyle::Dots)
            .with_color("cyan");
        spinner.start();

        let result = recon.run().await;
        spinner.stop();

        match result {
            Ok(module_result) => {
                self.display_recon_results(&module_result.data)?;

                // Save report if requested
                if let Some(output_path) = &args.output {
                    let json = serde_json::to_string_pretty(&module_result.data)?;
                    std::fs::write(output_path, json)?;
                    println!(
                        "\n{} Report saved: {}",
                        "💾".green(),
                        output_path.cyan()
                    );
                }

                Ok(())
            }
            Err(e) => {
                println!("{} Reconnaissance failed: {}", "✗".red(), e);
                Err(e)
            }
        }
    }

    /// Handle report generation
    async fn handle_report(&self, args: ReportArgs) -> Result<()> {
        let path = &args.path;

        if !Path::new(path).exists() {
            return Err(anyhow!("File not found: {}", path));
        }

        println!(
            "{} Generating {} report for: {}",
            "📄".cyan(),
            args.format.bright_white(),
            path.cyan()
        );

        // Determine file type and analyze
        let data = if path.ends_with(".apk") {
            let mut analyzer = ApkAnalyzer::new();
            analyzer.set_option("APK_PATH", path)?;
            analyzer.set_option("DEEP_ANALYSIS", "true")?;
            analyzer.set_option("CHECK_SECRETS", "true")?;
            analyzer.run().await?.data
        } else if path.ends_with(".ipa") {
            let mut analyzer = IpaAnalyzer::new();
            analyzer.set_option("IPA_PATH", path)?;
            analyzer.set_option("DEEP_ANALYSIS", "true")?;
            analyzer.set_option("CHECK_SECRETS", "true")?;
            analyzer.run().await?.data
        } else {
            return Err(anyhow!("Unknown file type: {}", path));
        };

        // Generate report based on format
        match args.format.as_str() {
            "json" => {
                let json = serde_json::to_string_pretty(&data)?;
                std::fs::write(&args.output, json)?;
            }
            "html" => {
                // Basic HTML report
                let html = self.generate_html_report(&data)?;
                std::fs::write(&args.output, html)?;
            }
            _ => {
                return Err(anyhow!("Unsupported format: {}", args.format));
            }
        }

        println!(
            "{} Report saved: {}",
            "✓".green(),
            args.output.cyan()
        );
        Ok(())
    }

    /// Check mobile analysis dependencies
    fn handle_check(&self) -> Result<()> {
        Theme::section("Mobile Analysis Dependencies");

        // Check Android tools
        println!("\n{}", "Android Tools:".bright_white().bold());
        self.check_tool("aapt", &["aapt", "aapt2"]);
        self.check_tool("apktool", &["apktool"]);
        self.check_tool("jadx", &["jadx"]);
        self.check_tool("dex2jar", &["d2j-dex2jar", "d2j-dex2jar.sh"]);

        // Check iOS tools
        println!("\n{}", "iOS Tools:".bright_white().bold());
        self.check_tool("plistutil", &["plistutil"]);
        self.check_tool("otool", &["otool"]);
        self.check_tool("codesign", &["codesign"]);
        self.check_tool("security", &["security"]);

        // Check common tools
        println!("\n{}", "Common Tools:".bright_white().bold());
        self.check_tool("unzip", &["unzip"]);
        self.check_tool("strings", &["strings"]);
        self.check_tool("file", &["file"]);

        println!();
        Ok(())
    }

    // === Helper methods ===

    fn print_analysis_header(&self, app_type: &str, path: &str) {
        let filename = Path::new(path)
            .file_name()
            .map(|n| n.to_string_lossy())
            .unwrap_or_default();

        println!("{}", "━".repeat(60).dimmed());
        println!(
            "{} Analyzing {}: {}",
            "🔍".cyan(),
            app_type,
            filename.bright_white()
        );
        println!("{}", "━".repeat(60).dimmed());
    }

    fn display_apk_results(
        &self,
        data: &HashMap<String, serde_json::Value>,
        verbose: bool,
    ) -> Result<()> {
        // Package Info
        if let Some(metadata) = data.get("metadata") {
            println!("\n{}", "📦 Package Info".bright_white().bold());
            self.print_tree_item(
                "Name",
                metadata.get("package_id").and_then(|v| v.as_str()),
                true,
            );
            self.print_tree_item(
                "Version",
                metadata.get("version").and_then(|v| v.as_str()),
                true,
            );
            self.print_tree_item(
                "Min SDK",
                metadata.get("min_sdk").and_then(|v| v.as_str()),
                true,
            );
            self.print_tree_item(
                "Target SDK",
                metadata.get("target_sdk").and_then(|v| v.as_str()),
                false,
            );
        }

        // Security Findings
        if let Some(findings) = data.get("findings").and_then(|v| v.as_array()) {
            self.display_findings(findings, verbose);
        }

        // Permissions
        if let Some(permissions) = data.get("permissions").and_then(|v| v.as_array()) {
            self.display_permissions(permissions, verbose);
        }

        // Exported Components
        if let Some(components) = data.get("exported_components").and_then(|v| v.as_array()) {
            self.display_components(components);
        }

        // Summary
        if let Some(summary) = data.get("summary") {
            self.display_summary(summary);
        }

        Ok(())
    }

    fn display_ipa_results(
        &self,
        data: &HashMap<String, serde_json::Value>,
        verbose: bool,
    ) -> Result<()> {
        // App Info
        if let Some(metadata) = data.get("metadata") {
            println!("\n{}", "📦 App Info".bright_white().bold());
            self.print_tree_item(
                "Bundle ID",
                metadata.get("package_id").and_then(|v| v.as_str()),
                true,
            );
            self.print_tree_item(
                "Version",
                metadata.get("version").and_then(|v| v.as_str()),
                true,
            );
            self.print_tree_item(
                "Min iOS",
                metadata.get("min_sdk").and_then(|v| v.as_str()),
                true,
            );
            self.print_tree_item(
                "Target iOS",
                metadata.get("target_sdk").and_then(|v| v.as_str()),
                false,
            );
        }

        // Binary Analysis
        if let Some(binary) = data.get("binary") {
            println!("\n{}", "🔐 Binary Protections".bright_white().bold());
            let pie = binary
                .get("pie_enabled")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let arc = binary
                .get("arc_enabled")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let canaries = binary
                .get("stack_canaries")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let encrypted = binary
                .get("is_encrypted")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            println!(
                "├─ PIE: {}",
                if pie {
                    "Enabled".green()
                } else {
                    "Disabled".red()
                }
            );
            println!(
                "├─ ARC: {}",
                if arc {
                    "Enabled".green()
                } else {
                    "Disabled".red()
                }
            );
            println!(
                "├─ Stack Canaries: {}",
                if canaries {
                    "Enabled".green()
                } else {
                    "Disabled".red()
                }
            );
            println!(
                "└─ Encrypted: {}",
                if encrypted {
                    "Yes".green()
                } else {
                    "No".yellow()
                }
            );
        }

        // Security Findings
        if let Some(findings) = data.get("findings").and_then(|v| v.as_array()) {
            self.display_findings(findings, verbose);
        }

        // URL Schemes
        if let Some(schemes) = data.get("url_schemes").and_then(|v| v.as_array()) {
            if !schemes.is_empty() {
                println!("\n{}", "🔗 URL Schemes".bright_white().bold());
                for (i, scheme) in schemes.iter().enumerate() {
                    let is_last = i == schemes.len() - 1;
                    let prefix = if is_last { "└─" } else { "├─" };
                    if let Some(s) = scheme.get("scheme").and_then(|v| v.as_str()) {
                        println!("{} {}", prefix, s.cyan());
                    }
                }
            }
        }

        // Summary
        if let Some(summary) = data.get("summary") {
            self.display_summary(summary);
        }

        Ok(())
    }

    fn display_recon_results(
        &self,
        data: &HashMap<String, serde_json::Value>,
    ) -> Result<()> {
        // App Info
        if let Some(app_info) = data.get("app_info") {
            println!("\n{}", "📱 App Information".bright_white().bold());

            if let Some(name) = app_info.get("app_name").and_then(|v| v.as_str()) {
                println!("├─ Name: {}", name.cyan());
            }
            if let Some(store) = app_info.get("store").and_then(|v| v.as_str()) {
                println!("├─ Store: {}", store);
            }
            if let Some(version) = app_info.get("current_version").and_then(|v| v.as_str()) {
                println!("├─ Version: {}", version);
            }
            if let Some(rating) = app_info.get("rating").and_then(|v| v.as_f64()) {
                println!("├─ Rating: {}", format!("{:.1} ⭐", rating).yellow());
            }
            if let Some(downloads) = app_info.get("downloads").and_then(|v| v.as_str()) {
                println!("├─ Downloads: {}", downloads.green());
            }
            if let Some(updated) = app_info.get("last_updated").and_then(|v| v.as_str()) {
                println!("└─ Last Updated: {}", updated);
            }

            // Developer Info
            if let Some(developer) = app_info.get("developer") {
                println!("\n{}", "👤 Developer".bright_white().bold());
                if let Some(name) = developer.get("name").and_then(|v| v.as_str()) {
                    println!("├─ Name: {}", name);
                }
                if let Some(email) = developer.get("email").and_then(|v| v.as_str()) {
                    println!("├─ Email: {}", email.cyan());
                }
                if let Some(website) = developer.get("website").and_then(|v| v.as_str()) {
                    println!("└─ Website: {}", website.blue());
                }
            }
        }

        // Detected SDKs
        if let Some(sdks) = data.get("detected_sdks").and_then(|v| v.as_array()) {
            if !sdks.is_empty() {
                println!("\n{}", "📚 Detected SDKs".bright_white().bold());
                for (i, sdk) in sdks.iter().enumerate() {
                    let is_last = i == sdks.len() - 1;
                    let prefix = if is_last { "└─" } else { "├─" };
                    let name = sdk.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
                    let vendor = sdk.get("vendor").and_then(|v| v.as_str()).unwrap_or("");
                    let confidence = sdk.get("confidence").and_then(|v| v.as_u64()).unwrap_or(0);

                    println!(
                        "{} {} ({}) - {}% confidence",
                        prefix,
                        name.cyan(),
                        vendor.dimmed(),
                        confidence
                    );
                }
            }
        }

        // Version History
        if let Some(versions) = data.get("version_history").and_then(|v| v.as_array()) {
            if !versions.is_empty() {
                println!("\n{}", "📋 Version History".bright_white().bold());
                for (i, ver) in versions.iter().take(5).enumerate() {
                    let is_last = i == versions.len().min(5) - 1;
                    let prefix = if is_last { "└─" } else { "├─" };
                    let version = ver.get("version").and_then(|v| v.as_str()).unwrap_or("?");
                    let date = ver.get("release_date").and_then(|v| v.as_str()).unwrap_or("");

                    println!("{} {} ({})", prefix, version.cyan(), date.dimmed());
                }
            }
        }

        Ok(())
    }

    fn display_findings(&self, findings: &[serde_json::Value], verbose: bool) {
        if findings.is_empty() {
            return;
        }

        // Group by severity
        let mut critical = Vec::new();
        let mut high = Vec::new();
        let mut medium = Vec::new();
        let mut low = Vec::new();
        let mut info = Vec::new();

        for finding in findings {
            let severity = finding
                .get("severity")
                .and_then(|v| v.as_str())
                .unwrap_or("Info");

            match severity {
                "Critical" => critical.push(finding),
                "High" => high.push(finding),
                "Medium" => medium.push(finding),
                "Low" => low.push(finding),
                _ => info.push(finding),
            }
        }

        println!("\n{}", "⚠️  Security Findings".bright_white().bold());

        // Print table header
        println!(
            "┌{:─<40}┬{:─<12}┬{:─<35}┐",
            "", "", ""
        );
        println!(
            "│ {:38} │ {:10} │ {:33} │",
            "Finding".bold(),
            "Severity".bold(),
            "Recommendation".bold()
        );
        println!(
            "├{:─<40}┼{:─<12}┼{:─<35}┤",
            "", "", ""
        );

        // Print findings by severity
        let all_findings: Vec<_> = critical
            .iter()
            .chain(high.iter())
            .chain(medium.iter())
            .chain(low.iter())
            .chain(if verbose { info.iter().collect() } else { vec![] })
            .collect();

        for finding in all_findings.iter().take(10) {
            let title = finding
                .get("title")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown");
            let severity = finding
                .get("severity")
                .and_then(|v| v.as_str())
                .unwrap_or("Info");
            let remediation = finding
                .get("remediation")
                .and_then(|v| v.as_str())
                .unwrap_or("-");

            let severity_colored = match severity {
                "Critical" => severity.bright_red().bold(),
                "High" => severity.red(),
                "Medium" => severity.yellow(),
                "Low" => severity.blue(),
                _ => severity.dimmed(),
            };

            let title_truncated = if title.len() > 38 {
                format!("{}...", &title[..35])
            } else {
                title.to_string()
            };

            let rem_truncated = if remediation.len() > 33 {
                format!("{}...", &remediation[..30])
            } else {
                remediation.to_string()
            };

            println!(
                "│ {:38} │ {:>10} │ {:33} │",
                title_truncated, severity_colored, rem_truncated
            );
        }

        println!(
            "└{:─<40}┴{:─<12}┴{:─<35}┘",
            "", "", ""
        );

        if !verbose && findings.len() > 10 {
            println!(
                "  {} (use {} for full list)",
                format!("... and {} more", findings.len() - 10).dimmed(),
                "--verbose".cyan()
            );
        }
    }

    fn display_permissions(&self, permissions: &[serde_json::Value], verbose: bool) {
        if permissions.is_empty() {
            return;
        }

        let dangerous: Vec<_> = permissions
            .iter()
            .filter(|p| p.get("is_dangerous").and_then(|v| v.as_bool()).unwrap_or(false))
            .collect();

        let total = permissions.len();
        let dangerous_count = dangerous.len();

        println!(
            "\n{} ({} total, {} dangerous)",
            "🔐 Permissions".bright_white().bold(),
            total,
            dangerous_count.to_string().red()
        );

        // Show dangerous permissions
        for (i, perm) in dangerous.iter().enumerate() {
            let is_last = i == dangerous.len() - 1 && !verbose;
            let prefix = if is_last { "└─" } else { "├─" };
            let name = perm
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown");
            let short_name = name.split('.').next_back().unwrap_or(name);

            println!("{} {} {}", prefix, "🔴".red(), short_name);
        }

        // Show normal permissions if verbose
        if verbose {
            let normal: Vec<_> = permissions
                .iter()
                .filter(|p| !p.get("is_dangerous").and_then(|v| v.as_bool()).unwrap_or(false))
                .collect();

            for (i, perm) in normal.iter().enumerate() {
                let is_last = i == normal.len() - 1;
                let prefix = if is_last { "└─" } else { "├─" };
                let name = perm
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown");
                let short_name = name.split('.').next_back().unwrap_or(name);

                println!("{} {} {}", prefix, "🟢".green(), short_name.dimmed());
            }
        } else if permissions.len() > dangerous_count {
            println!(
                "└─ {} (use {} for full list)",
                format!("... and {} normal permissions", permissions.len() - dangerous_count)
                    .dimmed(),
                "--verbose".cyan()
            );
        }
    }

    fn display_components(&self, components: &[serde_json::Value]) {
        if components.is_empty() {
            return;
        }

        let mut activities = 0;
        let mut services = 0;
        let mut receivers = 0;
        let mut providers = 0;
        let mut unprotected = 0;

        for comp in components {
            let comp_type = comp
                .get("component_type")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let has_perm = comp.get("permission").is_some()
                && !comp.get("permission").unwrap().is_null();

            match comp_type {
                "Activity" => activities += 1,
                "Service" => services += 1,
                "Receiver" => receivers += 1,
                "Provider" => providers += 1,
                _ => {}
            }

            if !has_perm {
                unprotected += 1;
            }
        }

        println!("\n{}", "📤 Exported Components".bright_white().bold());
        println!(
            "├─ Activities: {} ({})",
            activities,
            if unprotected > 0 {
                format!("{} without permission", unprotected).yellow()
            } else {
                "all protected".green().to_string().into()
            }
        );
        println!("├─ Services: {}", services);
        println!("├─ Receivers: {}", receivers);
        println!("└─ Providers: {}", providers);
    }

    fn display_summary(&self, summary: &serde_json::Value) {
        let total = summary.get("total_findings").and_then(|v| v.as_u64()).unwrap_or(0);
        let critical = summary.get("critical_count").and_then(|v| v.as_u64()).unwrap_or(0);
        let high = summary.get("high_count").and_then(|v| v.as_u64()).unwrap_or(0);
        let medium = summary.get("medium_count").and_then(|v| v.as_u64()).unwrap_or(0);
        let low = summary.get("low_count").and_then(|v| v.as_u64()).unwrap_or(0);
        let risk_score = summary.get("risk_score").and_then(|v| v.as_u64()).unwrap_or(0);
        let duration = summary.get("duration_ms").and_then(|v| v.as_u64()).unwrap_or(0);

        println!("\n{}", "📊 Summary".bright_white().bold());
        println!(
            "├─ Total Findings: {} (Critical: {}, High: {}, Medium: {}, Low: {})",
            total,
            critical.to_string().bright_red(),
            high.to_string().red(),
            medium.to_string().yellow(),
            low.to_string().blue()
        );

        let risk_bar = self.render_risk_bar(risk_score as u8);
        println!("├─ Risk Score: {} {}", risk_score, risk_bar);
        println!("└─ Analysis Time: {}ms", duration);
    }

    fn render_risk_bar(&self, score: u8) -> String {
        let filled = (score as f32 / 10.0).ceil() as usize;
        let empty = 10 - filled;

        let color = if score >= 75 {
            "red"
        } else if score >= 50 {
            "yellow"
        } else if score >= 25 {
            "blue"
        } else {
            "green"
        };

        let bar: String = "█".repeat(filled);
        let empty_bar: String = "░".repeat(empty);

        match color {
            "red" => format!("[{}{}]", bar.bright_red(), empty_bar.dimmed()),
            "yellow" => format!("[{}{}]", bar.yellow(), empty_bar.dimmed()),
            "blue" => format!("[{}{}]", bar.blue(), empty_bar.dimmed()),
            _ => format!("[{}{}]", bar.green(), empty_bar.dimmed()),
        }
    }

    fn print_tree_item(&self, label: &str, value: Option<&str>, has_more: bool) {
        let prefix = if has_more { "├─" } else { "└─" };
        let val = value.unwrap_or("Unknown");
        println!("{} {}: {}", prefix, label, val.cyan());
    }

    fn check_tool(&self, name: &str, binaries: &[&str]) {
        let found = binaries.iter().any(|bin| self.is_binary_available(bin));

        if found {
            println!("  {} {} - {}", "✓".green(), name, "available".green());
        } else {
            println!("  {} {} - {}", "✗".red(), name, "not found".red());
        }
    }

    fn is_binary_available(&self, name: &str) -> bool {
        if let Ok(path_var) = std::env::var("PATH") {
            for dir in std::env::split_paths(&path_var) {
                let candidate = dir.join(name);
                if candidate.is_file() {
                    return true;
                }
            }
        }
        false
    }

    fn generate_html_report(
        &self,
        data: &HashMap<String, serde_json::Value>,
    ) -> Result<String> {
        let title = "Ferox Mobile Security Analysis Report";
        let json_data = serde_json::to_string_pretty(data)?;

        let html = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <style>
        body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; margin: 40px; background: #0d1117; color: #c9d1d9; }}
        h1 {{ color: #58a6ff; border-bottom: 1px solid #30363d; padding-bottom: 10px; }}
        h2 {{ color: #8b949e; }}
        pre {{ background: #161b22; padding: 16px; border-radius: 8px; overflow-x: auto; border: 1px solid #30363d; }}
        .critical {{ color: #f85149; }}
        .high {{ color: #da3633; }}
        .medium {{ color: #d29922; }}
        .low {{ color: #58a6ff; }}
        .info {{ color: #8b949e; }}
    </style>
</head>
<body>
    <h1>{}</h1>
    <h2>Raw Analysis Data</h2>
    <pre>{}</pre>
</body>
</html>"#,
            title, title, json_data
        );

        Ok(html)
    }
}

impl Default for MobileCommandHandler {
    fn default() -> Self {
        Self::new()
    }
}
