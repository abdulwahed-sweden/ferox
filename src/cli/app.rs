#[cfg(feature = "memory-forensics")]
use crate::cli::memory::MemoryCli;
use crate::cli::theme::Theme;
use crate::core::audit;
use crate::core::module::{ModuleRegistry, ModuleResult, ModuleType};
use crate::core::payload::PayloadGenerator;
#[cfg(feature = "pdf-export")]
use crate::core::reporter::PdfReporter;
use crate::core::reporter::{HtmlReporter, JsonReporter, ReportData, Reporter};
use crate::core::result_store::ResultStore;
use crate::core::session::SessionManager;
use crate::handlers::{
    FileOperationsHandler, HandlerRegistry, HandlerType, LocalShellHandler, RemoteShellHandler,
    ShellType,
};
use anyhow::Result;
use colored::Colorize;
use rustyline::Helper;
use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{CompletionType, Config, Editor};
use std::collections::HashMap;
use std::io::{self, Write};
use std::path::Path;
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
            "help", "?", "modules", "list", "ls", "use", "back", "show", "set", "s", "options",
            "o", "check", "c", "run", "execute", "exploit", "x", "e", "info", "i", "sessions",
            "payloads", "export", "clear", "cls", "banner", "version", "exit", "quit", "q",
            // Handler commands
            "handlers", "shell", "exec", "upload", "download", "listen", "connect", "sysinfo", "ps",
            "kill", "pwd", "cd", "cat", "rm", "mkdir",
            // Memory forensics (feature gated at runtime)
            "memory",
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
        if !parts.is_empty() && (parts[0] == "use") {
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
        if !parts.is_empty() && parts[0] == "show" {
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
    result_store: Arc<Mutex<ResultStore>>,
    handlers: Arc<Mutex<HandlerRegistry>>,
    current_module: Option<String>,
    current_handler: Option<Uuid>,
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
            result_store: Arc::new(Mutex::new(ResultStore::default())),
            handlers: Arc::new(Mutex::new(HandlerRegistry::new())),
            current_module: None,
            current_handler: None,
            editor,
            aliases: get_aliases(),
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        Theme::banner();
        self.print_welcome().await;

        loop {
            let prompt = Theme::prompt(self.current_module.as_deref().unwrap_or(""));

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
            "export" => self.cmd_export(args).await,
            // Handler commands
            "handlers" => self.cmd_handlers(args).await,
            "shell" => self.cmd_shell(args).await,
            "exec" => self.cmd_exec(args).await,
            "upload" => self.cmd_upload(args).await,
            "download" => self.cmd_download(args).await,
            "listen" => self.cmd_listen(args).await,
            "connect" => self.cmd_connect(args).await,
            "sysinfo" => self.cmd_sysinfo().await,
            "ps" => self.cmd_ps().await,
            "kill" => self.cmd_kill(args).await,
            "pwd" => self.cmd_pwd().await,
            "cd" => self.cmd_cd(args).await,
            "cat" => self.cmd_cat(args).await,
            "rm" => self.cmd_rm(args).await,
            "mkdir" => self.cmd_mkdir(args).await,
            "memory" => self.cmd_memory(args).await,
            // Utility commands
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
        Theme::command_help(
            "check, c",
            "Run non-destructive check (safe fingerprinting)",
        );
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

        println!("  {}", "Handler Commands:".bright_yellow().bold());
        Theme::command_help("handlers", "List all registered handlers");
        Theme::command_help("handlers -s", "Show handler statistics");
        Theme::command_help("handlers -k <id>", "Remove handler by ID");
        Theme::command_help("handlers -t <type>", "List handlers by type");
        println!();

        println!("  {}", "Shell & Execution:".bright_yellow().bold());
        Theme::command_help("shell", "Create new local shell handler");
        Theme::command_help("shell -i <id>", "Select existing shell handler");
        Theme::command_help("exec <command>", "Execute command in current shell");
        Theme::command_help("sysinfo", "Display system information");
        Theme::command_help("ps", "List running processes");
        Theme::command_help("kill <pid>", "Terminate process by PID");
        println!();

        println!("  {}", "File Operations:".bright_yellow().bold());
        Theme::command_help("upload <src> <dst>", "Upload file to target");
        Theme::command_help("download <src> <dst>", "Download file from target");
        Theme::command_help("pwd", "Print current working directory");
        Theme::command_help("cd <path>", "Change working directory");
        Theme::command_help("cat <file>", "Display file contents");
        Theme::command_help("rm <file>", "Delete file");
        Theme::command_help("mkdir <dir>", "Create directory");
        println!();

        println!("  {}", "Remote Shell:".bright_yellow().bold());
        Theme::command_help("listen <port>", "Start reverse shell listener");
        Theme::command_help("connect <host> <port>", "Create bind shell connection");
        println!();

        #[cfg(feature = "memory-forensics")]
        {
            println!("  {}", "Memory Forensics:".bright_yellow().bold());
            Theme::command_help("memory analyze <dump>", "Run full memory analysis pipeline");
            Theme::command_help("memory pslist <dump>", "List heuristic process inventory");
            Theme::command_help(
                "memory malfind <dump>",
                "Search for injection and malware strings",
            );
            Theme::command_help("memory netscan <dump>", "Extract network indicators");
            Theme::command_help("memory mitre <dump>", "Map findings to MITRE ATT&CK");
            println!();
        }

        println!("  {}", "Report & Export Commands:".bright_yellow().bold());
        Theme::command_help("export <format> <file>", "Export results (json, html, pdf)");
        Theme::command_help("export results", "Show stored results summary");
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
            "Use TAB for command completion. Type 'help <category>' for focused help."
                .bright_white()
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
            Theme::warning(&format!(
                "No {} modules loaded",
                category_name.to_lowercase()
            ));
        } else {
            let first_module = modules.first().cloned();
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
        Theme::command_help(
            "sessions -i <id>",
            "Show session details (refreshes heartbeat)",
        );
        Theme::command_help("sessions -k <id>", "Kill/mark session as inactive");
        Theme::command_help("sessions -r <id>", "Remove session from database");
        Theme::command_help(
            "sessions -c <hours>",
            "Cleanup stale sessions older than N hours",
        );
        println!();

        println!("  {}", "Session Lifecycle:".bright_cyan().bold());
        println!("    • Exploit modules can establish sessions on successful runs");
        println!("    • Sessions track access to compromised targets");
        println!("    • Use 'sessions -i <id>' to keep sessions alive via heartbeat");
        println!("    • Inactive sessions can be cleaned up with 'sessions -c <hours>'");
        println!();

        let total = self.sessions.count().await;
        let active = self.sessions.active_count().await;
        Theme::info(&format!(
            "Current: {} total sessions ({} active)",
            total, active
        ));
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
            Theme::section(heading);
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

            // CRITICAL: Enforce confirmation for dangerous modules
            if module.requires_confirmation() {
                let info = module.info();
                println!();
                Theme::warning("⚠️  This module performs potentially destructive operations!");
                Theme::info(&format!("Module: {}", info.name));
                Theme::info(&format!("Category: {}", info.category));
                Theme::info(&format!("Description: {}", info.description));
                println!();
                Theme::warning("⚠️  AUTHORIZED USE ONLY - Explicit permission required");
                Theme::warning("⚠️  Use only in authorized testing environments");
                println!();

                print!("Continue? [y/N]: ");
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;

                let confirmed = input.trim().eq_ignore_ascii_case("y");

                // Log to audit file (append-only)
                let user = std::env::var("USER")
                    .or_else(|_| std::env::var("USERNAME"))
                    .unwrap_or_else(|_| "unknown".to_string());

                if let Err(e) =
                    audit::append_confirmation(&info.name, &info.category, &user, confirmed)
                {
                    Theme::warning(&format!("Failed to write audit log: {}", e));
                }

                if !confirmed {
                    Theme::warning("Module execution cancelled by user");
                    println!();
                    return Ok(());
                }

                Theme::success("Confirmation received - proceeding with execution");
                println!();
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

                    // Store result for later export
                    let module_info = module.info();
                    let mut store = self.result_store.lock().await;
                    let result_id = store.add(module_info, result.clone());
                    drop(store);

                    // Display results
                    self.render_result(&result)?;

                    // Notify user about stored result
                    Theme::info(&format!(
                        "Result stored (ID: {}). Use 'export' to save reports.",
                        result_id
                    ));
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
            println!(
                "  {}: {}",
                "Name".bright_cyan(),
                info.name.bright_white().bold()
            );
            println!(
                "  {}: {}",
                "Version".bright_cyan(),
                info.version.bright_yellow()
            );
            println!(
                "  {}: {}",
                "Author".bright_cyan(),
                info.author.bright_white()
            );
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

    async fn cmd_export(&self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            Theme::error("Usage: export <format> <filename>");
            Theme::info("Formats: json, html, pdf");
            Theme::info("Example: export json results.json");
            Theme::info("Example: export html report.html");
            Theme::info("Example: export pdf report.pdf");
            Theme::info("Or: export results (to view stored results)");
            return Ok(());
        }

        // Special command to view stored results
        if args[0] == "results" {
            return self.cmd_show_stored_results().await;
        }

        if args.len() < 2 {
            Theme::error("Please specify both format and filename");
            Theme::info("Usage: export <format> <filename>");
            return Ok(());
        }

        let format = args[0].to_lowercase();
        let filename = args[1];

        // Validate format
        if !["json", "html", "pdf"].contains(&format.as_str()) {
            Theme::error(&format!("Unknown format: {}", format));
            Theme::info("Supported formats: json, html, pdf");
            return Ok(());
        }

        // Get all stored results
        let store = self.result_store.lock().await;
        let results = store.get_all();

        if results.is_empty() {
            Theme::warning("No results to export. Run some modules first!");
            Theme::info("Use 'run' to execute modules and generate results");
            return Ok(());
        }

        Theme::info(&format!(
            "Exporting {} results to {} format...",
            results.len(),
            format
        ));

        // Get all sessions
        let sessions = self.sessions.list_all().await;

        // Create report data
        let stored_results: Vec<_> = results.into_iter().cloned().collect();
        let report_data = ReportData::new(stored_results, sessions);

        // Export based on format
        let result = match format.as_str() {
            "json" => {
                let reporter = JsonReporter;
                reporter.export(&report_data, Path::new(filename))
            }
            "html" => {
                let reporter = HtmlReporter;
                reporter.export(&report_data, Path::new(filename))
            }
            "pdf" => {
                #[cfg(feature = "pdf-export")]
                {
                    let reporter = PdfReporter;
                    reporter.export(&report_data, Path::new(filename))
                }
                #[cfg(not(feature = "pdf-export"))]
                {
                    Theme::error("PDF export not available. Rebuild with --features pdf-export");
                    return Ok(());
                }
            }
            _ => unreachable!(),
        };

        match result {
            Ok(_) => {
                Theme::success(&format!("✓ Report exported successfully to: {}", filename));
                Theme::info(&format!(
                    "Total results: {}",
                    report_data.summary.total_results
                ));
                Theme::info(&format!(
                    "Successful: {}",
                    report_data.summary.successful_results
                ));
                Theme::info(&format!("Failed: {}", report_data.summary.failed_results));
            }
            Err(e) => {
                Theme::error(&format!("Export failed: {}", e));
            }
        }

        Ok(())
    }

    async fn cmd_show_stored_results(&self) -> Result<()> {
        let store = self.result_store.lock().await;
        let results = store.get_all();

        Theme::section("STORED RESULTS");
        println!();

        if results.is_empty() {
            Theme::warning("No results stored yet");
            Theme::info("Run some modules to generate results");
            println!();
            return Ok(());
        }

        Theme::info(&format!("Total stored results: {}", results.len()));
        println!();

        println!(
            "  {:<8} {:<30} {:<10} {}",
            "Status".bright_cyan().bold(),
            "Module".bright_cyan().bold(),
            "Time".bright_cyan().bold(),
            "Message".bright_cyan().bold()
        );
        println!("  {}", "─".repeat(90).bright_blue());

        for result in results.iter().rev().take(20) {
            let status = if result.result.success {
                "SUCCESS".bright_green()
            } else {
                "FAILED".bright_red()
            };

            let module = format!(
                "{}/{}",
                result.module_info.category, result.module_info.name
            );
            let time = result.result.timestamp.format("%H:%M:%S").to_string();
            let message = if result.result.message.len() > 40 {
                format!("{}...", &result.result.message[..37])
            } else {
                result.result.message.clone()
            };

            println!(
                "  {:<8} {:<30} {:<10} {}",
                status,
                module.bright_white(),
                time.bright_yellow(),
                message
            );
        }

        println!();

        let successful = store.get_successful().len();
        let failed = store.get_failed().len();

        Theme::info(&format!("Successful: {} | Failed: {}", successful, failed));
        println!();
        Theme::info("💡 Use 'export <format> <file>' to export these results");
        println!();

        Ok(())
    }

    // ========== HANDLER COMMANDS ==========

    async fn cmd_handlers(&mut self, args: &[&str]) -> Result<()> {
        let handlers = self.handlers.lock().await;

        match args {
            [] => {
                let stats = handlers.get_stats().await;
                Theme::section("REGISTERED HANDLERS");
                println!();
                Theme::info(&format!("Total handlers: {}", stats.total));
                println!("  Local shells: {}", stats.local_shells);
                println!("  Remote shells: {}", stats.remote_shells);
                println!("  File operations: {}", stats.file_operations);
                println!();

                let local_ids = handlers.list_handlers(HandlerType::LocalShell).await;
                if !local_ids.is_empty() {
                    println!("  {}", "Local Shell Handlers:".bright_yellow());
                    for id in local_ids {
                        println!("    {}", id.to_string().bright_cyan());
                    }
                }

                let remote_ids = handlers.list_handlers(HandlerType::RemoteShell).await;
                if !remote_ids.is_empty() {
                    println!("  {}", "Remote Shell Handlers:".bright_yellow());
                    for id in remote_ids {
                        println!("    {}", id.to_string().bright_cyan());
                    }
                }

                let file_ids = handlers.list_handlers(HandlerType::FileOperations).await;
                if !file_ids.is_empty() {
                    println!("  {}", "File Operations Handlers:".bright_yellow());
                    for id in file_ids {
                        println!("    {}", id.to_string().bright_cyan());
                    }
                }
                println!();
            }
            ["-s"] => {
                let stats = handlers.get_stats().await;
                Theme::section("HANDLER STATISTICS");
                println!();
                println!("  Total handlers: {}", stats.total);
                println!("  Local shells: {}", stats.local_shells);
                println!("  Remote shells: {}", stats.remote_shells);
                println!("  File operations: {}", stats.file_operations);
                println!();
            }
            ["-k", id_str] => {
                if let Ok(id) = Uuid::parse_str(id_str) {
                    for handler_type in &[
                        HandlerType::LocalShell,
                        HandlerType::RemoteShell,
                        HandlerType::FileOperations,
                    ] {
                        if handlers.remove_handler(id, *handler_type).await {
                            Theme::success(&format!("Handler {} removed", id));
                            if self.current_handler == Some(id) {
                                drop(handlers);
                                return self.cmd_back().await;
                            }
                            return Ok(());
                        }
                    }
                    Theme::error(&format!("Handler {} not found", id));
                } else {
                    Theme::error("Invalid UUID format");
                }
            }
            ["-t", handler_type] => {
                let htype = match *handler_type {
                    "local" => HandlerType::LocalShell,
                    "remote" => HandlerType::RemoteShell,
                    "file" => HandlerType::FileOperations,
                    _ => {
                        Theme::error("Invalid handler type. Use: local, remote, file");
                        return Ok(());
                    }
                };

                let ids = handlers.list_handlers(htype).await;
                Theme::section(&format!("{:?} HANDLERS", htype));
                println!();
                if ids.is_empty() {
                    Theme::warning("No handlers of this type");
                } else {
                    for id in ids {
                        println!("  {}", id.to_string().bright_cyan());
                    }
                }
                println!();
            }
            _ => {
                Theme::error("Usage: handlers [-s|-k <id>|-t <type>]");
            }
        }
        Ok(())
    }

    async fn cmd_shell(&mut self, args: &[&str]) -> Result<()> {
        match args {
            [] => {
                let handler = LocalShellHandler::new();
                let handlers = self.handlers.lock().await;
                let id = handlers.register_local_shell(handler).await;
                drop(handlers);

                self.current_handler = Some(id);
                Theme::success(&format!("Local shell handler created: {}", id));
                Theme::info("Use 'exec <command>' to execute commands");
                Theme::info("Use 'back' to deselect handler");
            }
            ["-i", id_str] => {
                if let Ok(id) = Uuid::parse_str(id_str) {
                    let handlers = self.handlers.lock().await;

                    if handlers.has_handler(id, HandlerType::LocalShell).await
                        || handlers.has_handler(id, HandlerType::RemoteShell).await
                    {
                        drop(handlers);
                        self.current_handler = Some(id);
                        Theme::success(&format!("Handler {} selected", id));
                    } else {
                        Theme::error(&format!("Handler {} not found", id));
                    }
                } else {
                    Theme::error("Invalid UUID format");
                }
            }
            _ => {
                Theme::error("Usage: shell [-i <handler_id>]");
            }
        }
        Ok(())
    }

    async fn cmd_exec(&self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            Theme::error("Usage: exec <command>");
            return Ok(());
        }

        let handler_id = match self.current_handler {
            Some(id) => id,
            None => {
                Theme::error("No handler selected. Use 'shell' to create one");
                return Ok(());
            }
        };

        let command = args.join(" ");
        let handlers = self.handlers.lock().await;

        if let Some(result) = handlers.execute_local_command(handler_id, &command).await {
            match result {
                Ok(output) => {
                    if !output.stdout.is_empty() {
                        print!("{}", output.stdout);
                    }
                    if !output.stderr.is_empty() {
                        Theme::error(&output.stderr);
                    }
                    if !output.success {
                        Theme::warning(&format!("Command exited with code: {}", output.exit_code));
                    }
                }
                Err(e) => {
                    Theme::error(&format!("Execution failed: {}", e));
                }
            }
        } else {
            Theme::error("Handler not found or not a local shell");
        }

        Ok(())
    }

    async fn cmd_sysinfo(&self) -> Result<()> {
        let mut handler = LocalShellHandler::new();
        let info = handler.get_system_info();

        Theme::section("SYSTEM INFORMATION");
        println!();
        println!("  Hostname: {}", info.hostname.bright_cyan());
        println!("  OS: {} {}", info.os_name, info.os_version);
        println!("  Kernel: {}", info.kernel_version);
        println!("  CPUs: {}", info.cpu_count);
        println!(
            "  Memory: {} MB / {} MB used",
            info.used_memory / 1024 / 1024,
            info.total_memory / 1024 / 1024
        );
        println!("  Swap: {} MB", info.total_swap / 1024 / 1024);
        println!();

        Ok(())
    }

    async fn cmd_ps(&self) -> Result<()> {
        let mut handler = LocalShellHandler::new();
        let processes = handler.list_processes();

        Theme::section("RUNNING PROCESSES");
        println!();
        println!(
            "  {:<8} {:<30} {:<10} {:<10}",
            "PID".bright_yellow(),
            "NAME".bright_yellow(),
            "CPU%".bright_yellow(),
            "MEMORY".bright_yellow()
        );
        println!("  {}", "-".repeat(70));

        let mut sorted = processes;
        sorted.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());

        for (i, proc) in sorted.iter().take(20).enumerate() {
            let mem_mb = proc.memory / 1024 / 1024;
            println!(
                "  {:<8} {:<30} {:<10.2} {:<10} MB",
                proc.pid,
                &proc.name[..proc.name.len().min(30)],
                proc.cpu_usage,
                mem_mb
            );

            if i >= 19 {
                println!("\n  ... showing top 20 processes");
                break;
            }
        }
        println!();

        Ok(())
    }

    async fn cmd_kill(&self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            Theme::error("Usage: kill <pid>");
            return Ok(());
        }

        if let Ok(pid) = args[0].parse::<i32>() {
            let handler = LocalShellHandler::new();
            match handler.kill_process(pid) {
                Ok(_) => Theme::success(&format!("Process {} killed", pid)),
                Err(e) => Theme::error(&format!("Failed to kill process: {}", e)),
            }
        } else {
            Theme::error("Invalid PID");
        }

        Ok(())
    }

    async fn cmd_listen(&mut self, args: &[&str]) -> Result<()> {
        let (host, port) = match args {
            [port_str] => ("0.0.0.0".to_string(), port_str.parse::<u16>()?),
            [host, port_str] => (host.to_string(), port_str.parse::<u16>()?),
            _ => {
                Theme::error("Usage: listen [host] <port>");
                return Ok(());
            }
        };

        Theme::info(&format!(
            "Starting reverse shell listener on {}:{}",
            host, port
        ));

        let handler = RemoteShellHandler::new(ShellType::Reverse, host.clone(), port);

        match handler.start().await {
            Ok(_) => {
                let handlers = self.handlers.lock().await;
                let id = handlers.register_remote_shell(handler).await;
                drop(handlers);

                self.current_handler = Some(id);
                Theme::success(&format!("Connection received! Handler ID: {}", id));
                Theme::info("Use 'exec <command>' to interact with shell");
            }
            Err(e) => {
                Theme::error(&format!("Failed to start listener: {}", e));
            }
        }

        Ok(())
    }

    async fn cmd_connect(&mut self, args: &[&str]) -> Result<()> {
        if args.len() != 2 {
            Theme::error("Usage: connect <host> <port>");
            return Ok(());
        }

        let host = args[0].to_string();
        let port = args[1].parse::<u16>()?;

        Theme::info(&format!("Connecting to bind shell at {}:{}", host, port));

        let handler = RemoteShellHandler::new(ShellType::Bind, host, port);

        match handler.start().await {
            Ok(_) => {
                let handlers = self.handlers.lock().await;
                let id = handlers.register_remote_shell(handler).await;
                drop(handlers);

                self.current_handler = Some(id);
                Theme::success(&format!("Connected! Handler ID: {}", id));
                Theme::info("Use 'exec <command>' to interact with shell");
            }
            Err(e) => {
                Theme::error(&format!("Connection failed: {}", e));
            }
        }

        Ok(())
    }

    async fn cmd_upload(&self, args: &[&str]) -> Result<()> {
        if args.len() != 2 {
            Theme::error("Usage: upload <local_path> <remote_path>");
            return Ok(());
        }

        let handler = FileOperationsHandler::new();
        let local_path = args[0];
        let remote_path = args[1];

        Theme::info(&format!("Uploading {} -> {}", local_path, remote_path));

        let spinner = indicatif::ProgressBar::new_spinner();
        spinner.set_style(Theme::spinner_style());
        spinner.set_message("Uploading...");
        spinner.enable_steady_tick(std::time::Duration::from_millis(120));

        match handler.upload(local_path, remote_path).await {
            Ok(result) => {
                spinner.finish_and_clear();
                Theme::success(&format!("Uploaded {} bytes", result.bytes_transferred));
            }
            Err(e) => {
                spinner.finish_and_clear();
                Theme::error(&format!("Upload failed: {}", e));
            }
        }

        Ok(())
    }

    async fn cmd_download(&self, args: &[&str]) -> Result<()> {
        if args.len() != 2 {
            Theme::error("Usage: download <remote_path> <local_path>");
            return Ok(());
        }

        let handler = FileOperationsHandler::new();
        let remote_path = args[0];
        let local_path = args[1];

        Theme::info(&format!("Downloading {} -> {}", remote_path, local_path));

        let spinner = indicatif::ProgressBar::new_spinner();
        spinner.set_style(Theme::spinner_style());
        spinner.set_message("Downloading...");
        spinner.enable_steady_tick(std::time::Duration::from_millis(120));

        match handler.download(remote_path, local_path).await {
            Ok(result) => {
                spinner.finish_and_clear();
                Theme::success(&format!("Downloaded {} bytes", result.bytes_transferred));
            }
            Err(e) => {
                spinner.finish_and_clear();
                Theme::error(&format!("Download failed: {}", e));
            }
        }

        Ok(())
    }

    async fn cmd_pwd(&self) -> Result<()> {
        let handler = LocalShellHandler::new();
        match handler.get_cwd() {
            Ok(cwd) => println!("{}", cwd),
            Err(e) => Theme::error(&format!("Failed to get working directory: {}", e)),
        }
        Ok(())
    }

    async fn cmd_cd(&self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            Theme::error("Usage: cd <path>");
            return Ok(());
        }

        let handler = LocalShellHandler::new();
        match handler.change_directory(args[0]) {
            Ok(_) => Theme::success(&format!("Changed directory to {}", args[0])),
            Err(e) => Theme::error(&format!("Failed to change directory: {}", e)),
        }
        Ok(())
    }

    async fn cmd_cat(&self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            Theme::error("Usage: cat <file>");
            return Ok(());
        }

        let handler = FileOperationsHandler::new();
        match handler.read_file_string(args[0]).await {
            Ok(contents) => print!("{}", contents),
            Err(e) => Theme::error(&format!("Failed to read file: {}", e)),
        }
        Ok(())
    }

    async fn cmd_rm(&self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            Theme::error("Usage: rm <file>");
            return Ok(());
        }

        let handler = FileOperationsHandler::new();
        match handler.delete_file(args[0]).await {
            Ok(_) => Theme::success(&format!("Deleted {}", args[0])),
            Err(e) => Theme::error(&format!("Failed to delete file: {}", e)),
        }
        Ok(())
    }

    async fn cmd_mkdir(&self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            Theme::error("Usage: mkdir <directory>");
            return Ok(());
        }

        let handler = FileOperationsHandler::new();
        match handler.create_directory(args[0]).await {
            Ok(_) => Theme::success(&format!("Created directory {}", args[0])),
            Err(e) => Theme::error(&format!("Failed to create directory: {}", e)),
        }
        Ok(())
    }

    #[cfg(feature = "memory-forensics")]
    async fn cmd_memory(&mut self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            Theme::info("Usage: memory <subcommand> <dump> [options]");
            return Ok(());
        }

        MemoryCli::handle(args)
    }

    #[cfg(not(feature = "memory-forensics"))]
    async fn cmd_memory(&mut self, _args: &[&str]) -> Result<()> {
        Theme::warning(
            "Ferox built without memory analysis support. Rebuild with `--features memory-forensics`.",
        );
        Ok(())
    }
}
