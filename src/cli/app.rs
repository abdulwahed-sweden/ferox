use crate::cli::theme::Theme;
use crate::core::module::{ModuleRegistry, ModuleResult, ModuleType};
use crate::core::payload::PayloadGenerator;
use crate::core::session::SessionManager;
use anyhow::Result;
use colored::Colorize;
use rustyline::{Editor, Config, CompletionType};
use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::Helper;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

// Command aliases mapping
fn get_aliases() -> HashMap<&'static str, &'static str> {
    let mut aliases = HashMap::new();
    aliases.insert("ls", "modules");
    aliases.insert("s", "set");
    aliases.insert("x", "run");
    aliases.insert("e", "execute");
    aliases.insert("o", "options");
    aliases.insert("i", "info");
    aliases.insert("c", "check");
    aliases.insert("?", "help");
    aliases.insert("q", "quit");
    aliases
}

// Rustyline completion helper
struct FeroxHelper {
    commands: Vec<String>,
    modules: Arc<Mutex<Vec<String>>>,
}

impl FeroxHelper {
    fn new(modules: Arc<Mutex<Vec<String>>>) -> Self {
        let commands = vec![
            "help", "?", "modules", "list", "ls", "use", "back", "show",
            "set", "s", "options", "o", "check", "c", "run", "execute", "exploit",
            "x", "e", "info", "i", "sessions", "payloads", "clear", "cls",
            "banner", "version", "exit", "quit", "q"
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect();

        Self { commands, modules }
    }
}

impl Completer for FeroxHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let line_prefix = &line[..pos];
        let parts: Vec<&str> = line_prefix.split_whitespace().collect();

        // If we're completing the first word (command)
        if parts.is_empty() || (parts.len() == 1 && !line_prefix.ends_with(' ')) {
            let prefix = parts.first().unwrap_or(&"");
            let matches: Vec<Pair> = self
                .commands
                .iter()
                .filter(|cmd| cmd.starts_with(prefix))
                .map(|cmd| Pair {
                    display: cmd.clone(),
                    replacement: cmd.clone(),
                })
                .collect();

            let start = line_prefix.len() - prefix.len();
            return Ok((start, matches));
        }

        // If we're completing after "use" command, suggest modules
        if parts.len() >= 1 && (parts[0] == "use") {
            let prefix = if parts.len() >= 2 {
                parts[parts.len() - 1]
            } else {
                ""
            };

            // Access modules from Arc<Mutex<>> - need to block on async
            // For simplicity, we'll just return empty if we can't get lock immediately
            if let Ok(modules) = self.modules.try_lock() {
                let matches: Vec<Pair> = modules
                    .iter()
                    .filter(|module| module.starts_with(prefix))
                    .map(|module| Pair {
                        display: module.clone(),
                        replacement: module.clone(),
                    })
                    .collect();

                let start = line_prefix.len() - prefix.len();
                return Ok((start, matches));
            }
        }

        // If we're completing after "show" command
        if parts.len() >= 1 && parts[0] == "show" {
            let show_options = vec!["options", "modules"];
            let prefix = if parts.len() >= 2 {
                parts[parts.len() - 1]
            } else {
                ""
            };

            let matches: Vec<Pair> = show_options
                .into_iter()
                .filter(|opt| opt.starts_with(prefix))
                .map(|opt| Pair {
                    display: opt.to_string(),
                    replacement: opt.to_string(),
                })
                .collect();

            let start = line_prefix.len() - prefix.len();
            return Ok((start, matches));
        }

        Ok((0, vec![]))
    }
}

impl Hinter for FeroxHelper {
    type Hint = String;

    fn hint(&self, _line: &str, _pos: usize, _ctx: &rustyline::Context<'_>) -> Option<String> {
        None
    }
}

impl Highlighter for FeroxHelper {}
impl Validator for FeroxHelper {}
impl Helper for FeroxHelper {}

pub struct FeroxCli {
    registry: Arc<Mutex<ModuleRegistry>>,
    sessions: SessionManager,
    current_module: Option<String>,
    editor: Editor<FeroxHelper, rustyline::history::DefaultHistory>,
    aliases: HashMap<&'static str, &'static str>,
}

// (helpers removed; confirmation will be added later if needed)

impl FeroxCli {
    pub fn new(registry: ModuleRegistry) -> Result<Self> {
        // Get initial module list for completion
        let module_list: Vec<String> = registry.list();
        let modules_arc = Arc::new(Mutex::new(module_list));

        // Configure rustyline with tab completion
        let config = Config::builder()
            .completion_type(CompletionType::List)
            .auto_add_history(true)
            .build();

        let helper = FeroxHelper::new(modules_arc.clone());
        let mut editor = Editor::with_config(config)?;
        editor.set_helper(Some(helper));

        Ok(Self {
            registry: Arc::new(Mutex::new(registry)),
            sessions: SessionManager::new(),
            current_module: None,
            editor,
            aliases: get_aliases(),
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        Theme::banner();
        self.print_welcome().await;

        loop {
            let prompt = Theme::prompt(
                self.current_module
                    .as_ref()
                    .map(|s| s.as_str())
                    .unwrap_or(""),
            );

            match self.editor.readline(&prompt) {
                Ok(line) => {
                    let line = line.trim();
                    if line.is_empty() {
                        continue;
                    }

                    self.editor.add_history_entry(line)?;

                    if let Err(e) = self.handle_command(line).await {
                        Theme::error(&format!("Error: {}", e));
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    Theme::info("Use 'exit' or Ctrl+D to quit");
                }
                Err(ReadlineError::Eof) => {
                    println!("\n{}", "🦊 Stay ferocious! Goodbye!".bright_red().bold());
                    break;
                }
                Err(err) => {
                    Theme::error(&format!("Error: {}", err));
                    break;
                }
            }
        }

        Ok(())
    }

    async fn handle_command(&mut self, input: &str) -> Result<()> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(());
        }

        // Resolve aliases
        let raw_command = parts[0];
        let command = *self.aliases.get(raw_command).unwrap_or(&raw_command);
        let args = &parts[1..];

        match command {
            "help" | "?" => self.cmd_help_with_args(args).await,
            "modules" | "list" => self.cmd_list_modules().await,
            "use" => self.cmd_use(args).await,
            "back" => self.cmd_back().await,
            "show" => self.cmd_show(args).await,
            "set" => self.cmd_set(args).await,
            "options" => self.cmd_options().await,
            "check" => self.cmd_check().await,
            "run" | "execute" | "exploit" => self.cmd_run().await,
            "info" => self.cmd_info().await,
            "sessions" => self.cmd_sessions(args).await,
            "payloads" => self.cmd_payloads().await,
            "clear" | "cls" => self.cmd_clear(),
            "banner" => {
                Theme::banner();
                Ok(())
            }
            "version" => self.cmd_version().await,
            "exit" | "quit" | "q" => {
                println!("\n{}", "🦊 Stay ferocious! Goodbye!".bright_red().bold());
                std::process::exit(0);
            }
            _ => {
                Theme::error(&format!("Unknown command: {}", command));
                Theme::info("Type 'help' for available commands");
                Ok(())
            }
        }
    }

    async fn cmd_help(&self) -> Result<()> {
        // Check if user wants categorized help
        Theme::section("FEROX COMMANDS");
        println!();

        println!("  {}", "Core Commands:".bright_yellow().bold());
        Theme::command_help("help, ?", "Show this help message");
        Theme::command_help("help scanners", "Show available scanner modules");
        Theme::command_help("help exploits", "Show available exploit modules");
        Theme::command_help("help sessions", "Show session management help");
        Theme::command_help("modules, list, ls", "List all available modules");
        Theme::command_help("use <module>", "Select a module to use");
        Theme::command_help("back", "Deselect current module");
        println!();

        println!("  {}", "Module Commands:".bright_yellow().bold());
        Theme::command_help(
            "show <type>",
            "Show information (options, modules, sessions)",
        );
        Theme::command_help("set <option> <value>, s", "Set module option");
        Theme::command_help("options, o", "Show current module options");
        Theme::command_help("check, c", "Run non-destructive check (safe fingerprinting)");
        Theme::command_help("run, execute, x, e", "Execute current module");
        Theme::command_help("info, i", "Show current module information");
        println!();

        println!("  {}", "Session Commands:".bright_yellow().bold());
        Theme::command_help("sessions", "List all sessions");
        Theme::command_help("sessions -a", "List active sessions only");
        Theme::command_help("sessions -i <id>", "Show session details");
        Theme::command_help("sessions -k <id>", "Mark session inactive");
        Theme::command_help("sessions -r <id>", "Remove session");
        Theme::command_help("sessions -c <hours>", "Cleanup stale sessions");
        println!();

        println!("  {}", "Utility Commands:".bright_yellow().bold());
        Theme::command_help("banner", "Display Ferox banner");
        Theme::command_help("version", "Show version information");
        Theme::command_help("payloads", "List available payload blueprints");
        Theme::command_help("clear, cls", "Clear the screen");
        Theme::command_help("exit, quit, q", "Exit the framework");
        println!();

        println!("  {}", "💡 Tip:".bright_cyan().bold());
        println!(
            "    {}",
            "Use TAB for command completion. Type 'help <category>' for focused help.".bright_white()
        );
        println!();

        println!("  {}", "⚠️  Safety Notice:".bright_red().bold());
        println!(
            "    {}",
            "Always use 'check' before 'run' for exploit modules".bright_yellow()
        );
        println!(
            "    {}",
            "Exploits require explicit confirmation".bright_yellow()
        );
        println!(
            "    {}",
            "Only test systems you own or have permission to test".bright_yellow()
        );
        println!();
        Ok(())
    }

    async fn cmd_help_with_args(&self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            return self.cmd_help().await;
        }

        let category = args[0];
        match category {
            "scanners" | "scanner" => self.cmd_help_category(ModuleType::Scanner).await,
            "exploits" | "exploit" => self.cmd_help_category(ModuleType::Exploit).await,
            "auxiliary" | "aux" => self.cmd_help_category(ModuleType::Auxiliary).await,
            "post" | "postexploit" => self.cmd_help_category(ModuleType::PostExploit).await,
            "sessions" | "session" => self.cmd_help_sessions().await,
            _ => {
                Theme::error(&format!("Unknown help category: {}", category));
                Theme::info("Available categories: scanners, exploits, auxiliary, post, sessions");
                Ok(())
            }
        }
    }

    async fn cmd_help_category(&self, module_type: ModuleType) -> Result<()> {
        let registry = self.registry.lock().await;
        let modules = registry.list_by_type(module_type.clone());

        let category_name = match module_type {
            ModuleType::Scanner => "SCANNER MODULES",
            ModuleType::Exploit => "EXPLOIT MODULES",
            ModuleType::Auxiliary => "AUXILIARY MODULES",
            ModuleType::PostExploit => "POST-EXPLOITATION MODULES",
            _ => "MODULES",
        };

        Theme::section(category_name);
        println!();

        if modules.is_empty() {
            Theme::warning(&format!("No {} modules loaded", category_name.to_lowercase()));
        } else {
            let first_module = modules.first().map(|s| s.clone());
            for module_path in modules {
                if let Some(module) = registry.get(&module_path) {
                    let info = module.info();
                    println!(
                        "  {} {} - {}",
                        module_path.bright_green().bold(),
                        format!("({})", info.version).bright_yellow(),
                        info.description.bright_white()
                    );
                }
            }
            println!();
            if let Some(first) = first_module {
                Theme::info(&format!("💡 Use 'use {}' to select a module", first));
            }
        }

        println!();
        Ok(())
    }

    async fn cmd_help_sessions(&self) -> Result<()> {
        Theme::section("SESSION MANAGEMENT HELP");
        println!();

        println!("  {}", "Session Commands:".bright_yellow().bold());
        Theme::command_help("sessions", "List all sessions");
        Theme::command_help("sessions -a", "List active sessions only");
        Theme::command_help("sessions -i <id>", "Show session details (refreshes heartbeat)");
        Theme::command_help("sessions -k <id>", "Kill/mark session as inactive");
        Theme::command_help("sessions -r <id>", "Remove session from database");
        Theme::command_help("sessions -c <hours>", "Cleanup stale sessions older than N hours");
        println!();

        println!("  {}", "Session Lifecycle:".bright_cyan().bold());
        println!("    • Exploit modules can establish sessions on successful runs");
        println!("    • Sessions track access to compromised targets");
        println!("    • Use 'sessions -i <id>' to keep sessions alive via heartbeat");
        println!("    • Inactive sessions can be cleaned up with 'sessions -c <hours>'");
        println!();

        let total = self.sessions.count().await;
        let active = self.sessions.active_count().await;
        Theme::info(&format!("Current: {} total sessions ({} active)", total, active));
        println!();
        Ok(())
    }

    async fn cmd_list_modules(&self) -> Result<()> {
        let registry = self.registry.lock().await;
        let modules = registry.list();

        if modules.is_empty() {
            Theme::section("FEROX MODULES");
            println!();
            Theme::warning("No modules loaded");
            println!();
            return Ok(());
        }

        let categories: &[(ModuleType, &str)] = &[
            (ModuleType::Scanner, "Scanners"),
            (ModuleType::Exploit, "Exploits"),
            (ModuleType::Auxiliary, "Auxiliary"),
            (ModuleType::PostExploit, "Post-Exploitation"),
            (ModuleType::Payload, "Payloads"),
            (ModuleType::Encoder, "Encoders"),
            (ModuleType::Handler, "Handlers"),
        ];

        for (module_type, heading) in categories {
            let mut of_type = registry.list_by_type(module_type.clone());
            if of_type.is_empty() {
                continue;
            }

            of_type.sort();
            Theme::section(&format!("{}", heading));
            println!();

            for module_path in of_type {
                if let Some(module) = registry.get(&module_path) {
                    let info = module.info();
                    println!(
                        "  {} {} - {}",
                        module_path.bright_green().bold(),
                        format!("({})", info.version).bright_yellow(),
                        info.description.bright_white()
                    );
                }
            }
            println!();
        }

        Theme::info(&format!("🦊 Total: {} modules loaded", modules.len()));
        println!();
        Ok(())
    }

    async fn cmd_use(&mut self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            Theme::error("Usage: use <module_path>");
            return Ok(());
        }

        let module_path = args[0];
        let registry = self.registry.lock().await;

        if registry.get(module_path).is_some() {
            self.current_module = Some(module_path.to_string());
            Theme::success(&format!("🦊 Using module: {}", module_path));
        } else {
            Theme::error(&format!("Module not found: {}", module_path));
            Theme::info("Use 'modules' to see available modules");
        }

        Ok(())
    }

    async fn cmd_back(&mut self) -> Result<()> {
        self.current_module = None;
        Theme::info("Module deselected");
        Ok(())
    }

    async fn cmd_show(&self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            Theme::error("Usage: show <options|modules>");
            return Ok(());
        }

        match args[0] {
            "options" => self.cmd_options().await,
            "modules" => self.cmd_list_modules().await,
            _ => {
                Theme::error(&format!("Unknown show type: {}", args[0]));
                Ok(())
            }
        }
    }

    async fn cmd_set(&mut self, args: &[&str]) -> Result<()> {
        if args.len() < 2 {
            Theme::error("Usage: set <option> <value>");
            return Ok(());
        }

        let module_path = match &self.current_module {
            Some(path) => path.clone(),
            None => {
                Theme::error("No module selected. Use 'use <module>' first");
                return Ok(());
            }
        };

        let option = args[0];
        let value = args[1..].join(" ");

        let mut registry = self.registry.lock().await;
        if let Some(module) = registry.get_mut(&module_path) {
            module.set_option(option, &value)?;
            Theme::success(&format!(
                "{} => {}",
                option.bright_cyan(),
                value.bright_yellow()
            ));
        }

        Ok(())
    }

    async fn cmd_options(&self) -> Result<()> {
        let module_path = match &self.current_module {
            Some(path) => path,
            None => {
                Theme::error("No module selected. Use 'use <module>' first");
                return Ok(());
            }
        };

        let registry = self.registry.lock().await;
        if let Some(module) = registry.get(module_path) {
            let options = module.options();

            Theme::section("MODULE OPTIONS");
            println!();
            println!(
                "  {:<15} {:<10} {:<20} {}",
                "Name".bright_cyan().bold(),
                "Required".bright_cyan().bold(),
                "Current Value".bright_cyan().bold(),
                "Description".bright_cyan().bold()
            );
            println!("  {}", "─".repeat(85).bright_blue());

            for opt in options {
                let required = if opt.required {
                    "yes".bright_red().bold()
                } else {
                    "no".bright_green()
                };
                let value = opt.current_value.unwrap_or_else(|| "-".to_string());

                println!(
                    "  {:<15} {:<10} {:<20} {}",
                    opt.name.bright_white().bold(),
                    required,
                    value.bright_yellow(),
                    opt.description.bright_white()
                );
            }
            println!();
        }

        Ok(())
    }

    async fn cmd_check(&self) -> Result<()> {
        let module_path = match &self.current_module {
            Some(path) => path.clone(),
            None => {
                Theme::error("No module selected. Use 'use <module>' first");
                return Ok(());
            }
        };

        let registry = self.registry.lock().await;
        if let Some(module) = registry.get(&module_path) {
            // Validate first
            if let Err(e) = module.validate() {
                Theme::error(&format!("Validation failed: {}", e));
                return Ok(());
            }

            Theme::section("RUNNING SAFE CHECK");
            println!();
            Theme::info("🔍 Performing non-destructive fingerprinting...");

            let spinner = indicatif::ProgressBar::new_spinner();
            spinner.set_style(Theme::spinner_style());
            spinner.set_message("Checking target...");
            spinner.enable_steady_tick(std::time::Duration::from_millis(120));

            match module.check().await {
                Ok(result) => {
                    spinner.finish_and_clear();

                    if result.vulnerable {
                        Theme::warning(&format!(
                            "⚠️  Target appears VULNERABLE (confidence: {:.0}%)",
                            result.confidence * 100.0
                        ));
                    } else {
                        Theme::success("✓ Target does not appear vulnerable");
                    }

                    println!();
                    println!("  {}: {}", "Details".bright_cyan(), result.details);

                    if !result.fingerprint.is_empty() {
                        println!();
                        println!("  {}", "Fingerprint:".bright_cyan().bold());
                        for (key, value) in &result.fingerprint {
                            println!("    {}: {}", key.bright_yellow(), value);
                        }
                    }
                    println!();
                }
                Err(e) => {
                    spinner.finish_and_clear();
                    Theme::error(&format!("Check failed: {}", e));
                }
            }
        }

        Ok(())
    }

    async fn cmd_sessions(&self, args: &[&str]) -> Result<()> {
        match args {
            [] => {
                let total = self.sessions.count().await;
                let active = self.sessions.active_count().await;
                let sessions = self.sessions.list_all().await;

                Theme::section("FEROX SESSIONS");
                println!();
                Theme::info(&format!("Total sessions: {} ({} active)", total, active));

                if sessions.is_empty() {
                    Theme::warning("No sessions recorded yet");
                } else {
                    for session in sessions {
                        println!(
                            "  {} [{}] {} -> {}",
                            session.id,
                            if session.active { "active" } else { "inactive" },
                            session.module,
                            session.target
                        );
                    }
                }
                println!();
            }
            ["-a"] => {
                let active = self.sessions.list_active().await;
                Theme::section("ACTIVE SESSIONS");
                println!();
                if active.is_empty() {
                    Theme::warning("No active sessions");
                } else {
                    for session in active {
                        println!("  {} -> {}", session.module, session.target);
                    }
                }
                println!();
            }
            ["-i", id] => match Uuid::parse_str(id) {
                Ok(uuid) => {
                    if let Some(session) = self.sessions.get(uuid).await {
                        self.sessions.heartbeat(uuid).await?;
                        Theme::section(&format!("SESSION {}", uuid));
                        println!();
                        println!("  Module : {}", session.module);
                        println!("  Target : {}", session.target);
                        println!("  Platform: {:?}", session.platform);
                        println!("  Active : {}", session.active);
                        println!("  Established : {}", session.established_at.to_rfc2822());
                        println!("  Last Seen : {}", session.last_seen.to_rfc2822());
                        if !session.metadata.is_empty() {
                            println!();
                            println!("  Metadata:");
                            for (key, value) in session.metadata {
                                println!(
                                    "    {} => {}",
                                    key.bright_cyan(),
                                    value.to_string().bright_white()
                                );
                            }
                        }
                        println!();
                    } else {
                        Theme::error(&format!("Session not found: {}", id));
                    }
                }
                Err(_) => Theme::error("Invalid session ID format"),
            },
            ["-k", id] => {
                let uuid = Uuid::parse_str(id)?;
                self.sessions.kill(uuid).await?;
                Theme::warning(&format!("Session {} marked inactive", uuid));
            }
            ["-r", id] => {
                let uuid = Uuid::parse_str(id)?;
                self.sessions.remove(uuid).await?;
                Theme::success(&format!("Session {} removed", uuid));
            }
            ["-c", hours] => {
                let hours: i64 = hours.parse().unwrap_or(24);
                let removed = self.sessions.cleanup_stale(hours).await;
                Theme::info(&format!("Removed {} stale sessions", removed));
            }
            _ => {
                Theme::error("Usage: sessions [ -a | -i <id> | -k <id> | -r <id> | -c <hours> ]");
            }
        }

        Ok(())
    }

    async fn cmd_run(&mut self) -> Result<()> {
        let module_path = match &self.current_module {
            Some(path) => path.clone(),
            None => {
                Theme::error("No module selected. Use 'use <module>' first");
                return Ok(());
            }
        };

        let mut registry = self.registry.lock().await;
        if let Some(module) = registry.get_mut(&module_path) {
            // Validate first
            if let Err(e) = module.validate() {
                Theme::error(&format!("Validation failed: {}", e));
                Theme::info("Use 'options' to check required parameters");
                return Ok(());
            }

            Theme::section("EXECUTING MODULE");
            println!();

            let spinner = indicatif::ProgressBar::new_spinner();
            spinner.set_style(Theme::spinner_style());
            spinner.set_message("🦊 Ferox is hunting...");
            spinner.enable_steady_tick(std::time::Duration::from_millis(120));

            match module.run().await {
                Ok(result) => {
                    spinner.finish_and_clear();

                    if result.success {
                        Theme::success(&result.message);
                    } else {
                        Theme::error(&result.message);
                    }

                    // Display results
                    self.render_result(&result)?;
                }
                Err(e) => {
                    spinner.finish_and_clear();
                    Theme::error(&format!("Execution failed: {}", e));
                }
            }
        }

        Ok(())
    }

    async fn cmd_info(&self) -> Result<()> {
        let module_path = match &self.current_module {
            Some(path) => path,
            None => {
                Theme::error("No module selected. Use 'use <module>' first");
                return Ok(());
            }
        };

        let registry = self.registry.lock().await;
        if let Some(module) = registry.get(module_path) {
            let info = module.info();

            Theme::module_header(&info.name);
            println!();
            println!("  {}: {}", "Name".bright_cyan(), info.name.bright_white().bold());
            println!("  {}: {}", "Version".bright_cyan(), info.version.bright_yellow());
            println!("  {}: {}", "Author".bright_cyan(), info.author.bright_white());
            println!("  {}: {:?}", "Type".bright_cyan(), info.module_type);
            println!(
                "  {}: {}",
                "Category".bright_cyan(),
                info.category.bright_white()
            );
            println!(
                "  {}: {}",
                "Description".bright_cyan(),
                info.description.bright_white()
            );
            println!();
        }

        Ok(())
    }

    fn cmd_clear(&self) -> Result<()> {
        print!("\x1B[2J\x1B[1;1H");
        Theme::banner();
        Ok(())
    }

    async fn cmd_version(&self) -> Result<()> {
        println!();
        println!(
            "  {} {}",
            "🦊 Ferox Framework".bright_red().bold(),
            "v2.0.0".bright_yellow()
        );
        println!("  {}", "Ferocious Security Framework".bright_white());
        println!("  {}", "Built with Rust 🦀".bright_cyan());
        println!();
        Ok(())
    }

    async fn print_welcome(&self) {
        let registry = self.registry.lock().await;
        let module_count = registry.count();
        drop(registry);
        let session_count = self.sessions.count().await;
        let active_sessions = self.sessions.active_count().await;

        Theme::section("SYSTEM INITIALIZATION");
        Theme::success("Core engine initialized");
        Theme::success("Module registry loaded");
        Theme::success(&format!("{} modules available", module_count));
        Theme::info(&format!(
            "{} sessions tracked ({} active)",
            session_count, active_sessions
        ));
        Theme::status("ready", "Ferox is ready to hunt");

        println!();
        Theme::info("🦊 Type 'help' for available commands");
        Theme::info("🔥 Type 'modules' to list all modules");
        Theme::info("⚡ Fast. Fierce. Fearless.");
        println!();
    }

    fn render_result(&self, result: &ModuleResult) -> Result<()> {
        if result.data.is_empty() {
            return Ok(());
        }

        println!();
        Theme::section("RESULTS");
        println!();
        for (key, value) in &result.data {
            println!(
                "  {}: {}",
                key.bright_cyan().bold(),
                serde_json::to_string_pretty(value)?
            );
        }
        println!();
        Ok(())
    }

    async fn cmd_payloads(&self) -> Result<()> {
        Theme::section("PAYLOAD BLUEPRINTS");
        println!();

        for payload in PayloadGenerator::available_types() {
            Theme::command_help(payload, "Available placeholder payload");
        }

        println!();
        Ok(())
    }
}
