use anyhow::{Context, Result, anyhow};
use chrono::{DateTime, Utc};
use clap::{Args, Subcommand};
use uuid::Uuid;

use crate::cli::theme::Theme;
use crate::core::module::Session;
use crate::core::session::SessionManager;

#[derive(Subcommand, Debug, Clone)]
pub enum SessionsCommands {
    /// List sessions (active by default)
    List(SessionsListArgs),
    /// Show a single session with metadata
    Show(SessionIdArg),
    /// Mark a session inactive
    Kill(SessionIdArg),
    /// Kill all active sessions
    KillAll,
    /// Remove a session record entirely
    Remove(SessionIdArg),
    /// Execute a command in-session
    Exec(SessionExecArgs),
    /// Print stored command history
    History(SessionIdArg),
    /// Clear history for a session
    ClearHistory(SessionIdArg),
    /// Remove stale inactive sessions older than N hours
    Cleanup(SessionsCleanupArgs),
    /// Show aggregate counts
    Stats,
}

#[derive(Args, Debug, Clone)]
pub struct SessionsListArgs {
    /// Include inactive sessions
    #[arg(short, long)]
    pub all: bool,
}

#[derive(Args, Debug, Clone)]
pub struct SessionIdArg {
    /// Session UUID (use `ferox sessions list` to discover)
    #[arg(value_parser = parse_uuid)]
    pub id: Uuid,
}

#[derive(Args, Debug, Clone)]
pub struct SessionExecArgs {
    #[arg(value_parser = parse_uuid)]
    pub id: Uuid,
    /// Command to execute (everything after the ID is treated as part of the command)
    #[arg(value_name = "command", num_args = 1.., trailing_var_arg = true)]
    pub command: Vec<String>,
}

#[derive(Args, Debug, Clone)]
pub struct SessionsCleanupArgs {
    /// Hours since last check-in
    #[arg(long, default_value = "24")]
    pub hours: i64,
}

pub struct SessionCommandHandler;

impl SessionCommandHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn describe() -> &'static str {
        "Inspect and manage session cache"
    }

    pub fn print_usage() {
        Theme::section("Sessions CLI");
        Theme::command_help("ferox sessions list", "Show active sessions");
        Theme::command_help("ferox sessions kill <uuid>", "Mark a session inactive");
        Theme::command_help(
            "ferox sessions exec <uuid> <cmd>",
            "Send a command to a session",
        );
    }

    pub async fn run(&self, manager: &SessionManager, command: SessionsCommands) -> Result<()> {
        match command {
            SessionsCommands::List(args) => {
                let sessions = if args.all {
                    manager.list_all().await
                } else {
                    manager.list_active().await
                };
                self.print_table(&sessions);
                Ok(())
            }
            SessionsCommands::Show(arg) => {
                if let Some(session) = manager.get(arg.id).await {
                    self.print_detail(&session);
                    Ok(())
                } else {
                    Err(anyhow!("Session not found: {}", arg.id))
                }
            }
            SessionsCommands::Kill(arg) => {
                manager.kill(arg.id).await?;
                Theme::success(&format!("Session {} marked inactive", arg.id));
                Ok(())
            }
            SessionsCommands::KillAll => {
                let count = manager.kill_all().await?;
                Theme::warning(&format!("Marked {count} sessions inactive"));
                Ok(())
            }
            SessionsCommands::Remove(arg) => {
                manager.remove(arg.id).await?;
                Theme::warning(&format!("Session {} removed", arg.id));
                Ok(())
            }
            SessionsCommands::Exec(args) => {
                let command = args.command.join(" ");
                let output = manager
                    .execute_command(args.id, &command)
                    .await
                    .with_context(|| "Failed to send command")?;
                Theme::section("Command Output");
                println!("{}", output);
                Ok(())
            }
            SessionsCommands::History(arg) => {
                let history = manager.get_history(arg.id).await?;
                Theme::section("Command History");
                for entry in history {
                    println!(
                        "[{}] {}\n{}\n",
                        format_timestamp(entry.executed_at),
                        entry.command,
                        entry.output
                    );
                }
                Ok(())
            }
            SessionsCommands::ClearHistory(arg) => {
                manager.clear_history(arg.id).await?;
                Theme::warning(&format!("History cleared for {}", arg.id));
                Ok(())
            }
            SessionsCommands::Cleanup(args) => {
                let removed = manager.cleanup_stale(args.hours).await;
                Theme::info(&format!("Removed {removed} stale sessions"));
                Ok(())
            }
            SessionsCommands::Stats => {
                let total = manager.count().await;
                let active = manager.active_count().await;
                Theme::section("Session Stats");
                println!(
                    "Total: {total}\nActive: {active}\nInactive: {}",
                    total - active
                );
                Ok(())
            }
        }
    }

    fn print_table(&self, sessions: &[Session]) {
        if sessions.is_empty() {
            Theme::info("No sessions found");
            return;
        }

        Theme::section("Sessions");
        println!(
            "{:<12} {:<18} {:<18} {:<8} {:<20}",
            "ID", "Module", "Target", "Active", "Last Seen"
        );
        for session in sessions {
            println!(
                "{:<12} {:<18} {:<18} {:<8} {:<20}",
                short_id(session.id),
                session.module,
                session.target,
                if session.active { "yes" } else { "no" },
                format_timestamp(session.last_seen)
            );
        }
    }

    fn print_detail(&self, session: &Session) {
        Theme::section("Session Detail");
        println!("ID: {}", session.id);
        println!("Module: {}", session.module);
        println!("Target: {}", session.target);
        println!("Platform: {:?}", session.platform);
        println!(
            "User: {}",
            session.user.clone().unwrap_or_else(|| "n/a".into())
        );
        println!("Active: {}", session.active);
        println!("Established: {}", format_timestamp(session.established_at));
        println!("Last Seen: {}", format_timestamp(session.last_seen));

        if session.metadata.is_empty() {
            Theme::info("No metadata stored");
        } else {
            Theme::section("Metadata");
            for (key, value) in &session.metadata {
                println!("- {}: {}", key, value);
            }
        }
    }
}

fn parse_uuid(value: &str) -> Result<Uuid, String> {
    Uuid::parse_str(value).map_err(|err| format!("Invalid UUID {value}: {err}"))
}

fn format_timestamp(ts: DateTime<Utc>) -> String {
    ts.format("%Y-%m-%d %H:%M:%S").to_string()
}

fn short_id(id: Uuid) -> String {
    id.to_string()[0..12].to_string()
}
