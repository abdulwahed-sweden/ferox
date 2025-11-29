use std::env;
use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::Subcommand;

use crate::cli::commands::{
    C2CommandHandler, C2Commands, CredsCommandHandler, CredsCommands, DoctorCommandHandler,
    LateralCommandHandler, LateralCommands, MemoryCommandHandler, MemoryCommands,
    MobileCommandHandler, MobileCommands, OpsecCommandHandler, OpsecCommands,
    PersistCommandHandler, PersistCommands, PrivEscCommandHandler, PrivEscCommands,
    SessionCommandHandler, SessionsCommands, WizardCommandHandler, WizardCommands,
};
use crate::cli::doctor::DoctorCommands;
use crate::cli::theme::Theme;
use crate::core::session::SessionManager;
use crate::tools::theme::CliThemeApplier;

/// Outcome of attempting to dispatch a CLI command
pub enum RouterDispatch {
    /// CLI command handled by router; skip interactive console
    Handled,
    /// No CLI command; fall back to interactive shell
    Fallthrough,
}

/// Unified CLI router that fronts Ferox subcommands before entering the TUI/console
pub struct CommandRouter {
    session_manager: SessionManager,
    cli_theme: CliThemeApplier,
}

impl CommandRouter {
    /// Build a router with persisted session state if possible
    pub async fn initialize(cli_theme: CliThemeApplier) -> Result<Self> {
        let session_manager = match SessionManager::with_db(Self::session_db_path()) {
            Ok(manager) => {
                manager.load_from_db().await?;
                manager
            }
            Err(err) => {
                Theme::warning(&format!(
                    "Session DB disabled ({}). Using in-memory store.",
                    err
                ));
                SessionManager::new()
            }
        };

        Ok(Self {
            session_manager,
            cli_theme,
        })
    }

    /// Print banner/usage and either handle a command or fall through to interactive console
    pub async fn dispatch(&self, command: Option<RouterCommand>) -> Result<RouterDispatch> {
        self.print_banner();

        match command {
            Some(RouterCommand::Doctor(cmd)) => {
                DoctorCommandHandler::new(&self.cli_theme).run(cmd)?;
                Ok(RouterDispatch::Handled)
            }
            Some(RouterCommand::Memory(cmd)) => {
                self.ensure_memory_toolchain();
                MemoryCommandHandler::new().run(cmd)?;
                Ok(RouterDispatch::Handled)
            }
            Some(RouterCommand::C2(cmd)) => {
                C2CommandHandler::new().run(cmd)?;
                Ok(RouterDispatch::Handled)
            }
            Some(RouterCommand::Sessions(cmd)) => {
                SessionCommandHandler::new()
                    .run(&self.session_manager, cmd)
                    .await?;
                Ok(RouterDispatch::Handled)
            }
            Some(RouterCommand::Persist(cmd)) => {
                PersistCommandHandler::new().run(cmd)?;
                Ok(RouterDispatch::Handled)
            }
            Some(RouterCommand::PrivEsc(cmd)) => {
                PrivEscCommandHandler::new().run(cmd)?;
                Ok(RouterDispatch::Handled)
            }
            Some(RouterCommand::Creds(cmd)) => {
                CredsCommandHandler::new().run(cmd)?;
                Ok(RouterDispatch::Handled)
            }
            Some(RouterCommand::Lateral(cmd)) => {
                LateralCommandHandler::new().run(cmd).await?;
                Ok(RouterDispatch::Handled)
            }
            Some(RouterCommand::Opsec(cmd)) => {
                OpsecCommandHandler::new().run(cmd).await?;
                Ok(RouterDispatch::Handled)
            }
            Some(RouterCommand::Wizard(cmd)) => {
                WizardCommandHandler::new().run(cmd).await?;
                Ok(RouterDispatch::Handled)
            }
            Some(RouterCommand::Mobile(cmd)) => {
                MobileCommandHandler::new().run(cmd).await?;
                Ok(RouterDispatch::Handled)
            }
            Some(RouterCommand::Console) => {
                self.print_usage();
                self.ensure_memory_toolchain();
                Ok(RouterDispatch::Fallthrough)
            }
            None => {
                self.print_usage();
                self.ensure_memory_toolchain();
                Ok(RouterDispatch::Fallthrough)
            }
        }
    }

    fn session_db_path() -> PathBuf {
        Path::new("ferox_sessions.db").into()
    }

    fn print_banner(&self) {
        println!("============================================================================");
        println!("                     Ferox CLI Integration Layer                              ");
        println!("  doctor | memory | c2 | sessions | persist | privesc | creds | lateral");
        println!("  opsec | wizard | mobile | console");
        println!("============================================================================");
    }

    fn print_usage(&self) {
        Theme::section("CLI Quick Start");
        Theme::command_help("ferox doctor <cmd>", DoctorCommandHandler::describe());
        Theme::command_help("ferox memory <cmd>", MemoryCommandHandler::describe());
        Theme::command_help("ferox c2 <cmd>", C2CommandHandler::describe());
        Theme::command_help("ferox sessions <cmd>", SessionCommandHandler::describe());
        Theme::command_help("ferox persist <cmd>", PersistCommandHandler::describe());
        Theme::command_help("ferox privesc <cmd>", PrivEscCommandHandler::describe());
        Theme::command_help("ferox creds <cmd>", CredsCommandHandler::describe());
        Theme::command_help("ferox lateral <cmd>", LateralCommandHandler::describe());
        Theme::command_help("ferox opsec <cmd>", OpsecCommandHandler::describe());
        Theme::command_help("ferox wizard", WizardCommandHandler::describe());
        Theme::command_help("ferox mobile <cmd>", MobileCommandHandler::describe());
        Theme::command_help("ferox console", "Launch interactive console");
    }

    fn ensure_memory_toolchain(&self) {
        Theme::section("Memory Tooling");
        self.report_tool("python3", &["python3", "python"]);
        self.report_tool("volatility3", &["volatility3", "vol.py"]);
        self.report_tool("yara", &["yara"]);
    }

    fn report_tool(&self, label: &str, candidates: &[&str]) {
        if let Some(found) = candidates.iter().find_map(|bin| self.detect_binary(bin)) {
            Theme::status("ready", &format!("{label}: {found}"));
        } else {
            let searched = candidates.join(", ");
            Theme::status("error", &format!("{label}: not found (tried {searched})"));
        }
    }

    fn detect_binary(&self, name: &str) -> Option<String> {
        let path_var = env::var_os("PATH")?;
        for dir in env::split_paths(&path_var) {
            let candidate = dir.join(name);
            if is_executable(&candidate) {
                return Some(candidate.to_string_lossy().to_string());
            }
            #[cfg(windows)]
            {
                for ext in ["exe", "bat", "cmd"] {
                    let alt = dir.join(format!("{name}.{ext}"));
                    if is_executable(&alt) {
                        return Some(alt.to_string_lossy().to_string());
                    }
                }
            }
        }
        None
    }
}

fn is_executable(path: &Path) -> bool {
    path.is_file()
}

#[derive(Subcommand, Debug, Clone)]
pub enum RouterCommand {
    /// Doctor / dependency diagnostics
    #[command(subcommand)]
    Doctor(DoctorCommands),
    /// Memory forensics helpers
    #[command(subcommand)]
    Memory(MemoryCommands),
    /// Command-and-control orchestration helpers
    #[command(subcommand)]
    C2(C2Commands),
    /// Session database helpers
    #[command(subcommand)]
    Sessions(SessionsCommands),
    /// Persistence engine commands
    #[command(subcommand)]
    Persist(PersistCommands),
    /// Privilege escalation engine commands
    #[command(subcommand, name = "privesc")]
    PrivEsc(PrivEscCommands),
    /// Credential harvesting engine commands
    #[command(subcommand)]
    Creds(CredsCommands),
    /// Lateral movement engine commands
    #[command(subcommand)]
    Lateral(LateralCommands),
    /// OPSEC engine commands
    #[command(subcommand)]
    Opsec(OpsecCommands),
    /// Attack wizard - guided penetration testing
    Wizard(WizardCommands),
    /// Mobile app security analysis (APK/IPA)
    #[command(subcommand)]
    Mobile(MobileCommands),
    /// Skip router messaging and jump into console
    Console,
}
