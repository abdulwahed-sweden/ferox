use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, Subcommand};

use crate::cli::memory::{MemoryCli, MemoryCommand};
use crate::cli::theme::Theme;

#[derive(Subcommand, Debug, Clone)]
pub enum MemoryCommands {
    /// Full triage pipeline with optional export
    Analyze(MemoryAnalyzeArgs),
    /// Process inventory
    Pslist(MemoryDumpArgs),
    /// Process tree view
    Pstree(MemoryDumpArgs),
    /// Injection and malware scan
    Malfind(MemoryDumpArgs),
    /// Network connection extraction
    Netscan(MemoryDumpArgs),
    /// Dump credential artifacts
    Hashdump(MemoryDumpArgs),
    /// Registry hive listing
    Hivelist(MemoryDumpArgs),
    /// Print registry key/value pairs
    Printkey(MemoryPrintKeyArgs),
    /// Execute YARA rules over dump
    Yarascan(MemoryYaraArgs),
    /// Map findings to MITRE ATT&CK
    Mitre(MemoryMitreArgs),
}

#[derive(Args, Debug, Clone)]
pub struct MemoryDumpArgs {
    /// Path to raw memory image (e.g. WinPMEM)
    pub dump: PathBuf,
}

#[derive(Args, Debug, Clone)]
pub struct MemoryAnalyzeArgs {
    /// Path to raw memory image
    pub dump: PathBuf,
    /// Optional JSON export path
    #[arg(long)]
    pub output: Option<PathBuf>,
    /// Emit JSON to stdout instead of rich text
    #[arg(short, long)]
    pub json: bool,
}

#[derive(Args, Debug, Clone)]
pub struct MemoryPrintKeyArgs {
    /// Path to raw memory image
    pub dump: PathBuf,
    /// Registry path (e.g. HKLM\\Software\\Microsoft)
    #[arg(long)]
    pub key: String,
}

#[derive(Args, Debug, Clone)]
pub struct MemoryYaraArgs {
    /// Path to raw memory image
    pub dump: PathBuf,
    /// Path to YARA ruleset
    #[arg(long)]
    pub rules: PathBuf,
}

#[derive(Args, Debug, Clone)]
pub struct MemoryMitreArgs {
    /// Path to raw memory image
    pub dump: PathBuf,
    /// Optional JSON export path for ATT&CK techniques
    #[arg(long)]
    pub output: Option<PathBuf>,
}

pub struct MemoryCommandHandler;

impl MemoryCommandHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn describe() -> &'static str {
        "Volatility-backed memory forensics helpers"
    }

    pub fn print_usage() {
        Theme::section("Memory Forensics CLI");
        Theme::command_help(
            "ferox memory analyze <dump>",
            "Run the full analysis pipeline",
        );
        Theme::command_help("ferox memory pslist <dump>", "Process inventory (quick)");
        Theme::command_help(
            "ferox memory malfind <dump>",
            "Scan for injections/YARA hits",
        );
        Theme::command_help("ferox memory mitre <dump>", "Export ATT&CK mapping");
    }

    pub fn run(&self, command: MemoryCommands) -> Result<()> {
        Theme::section("Memory Forensics");
        MemoryCli::run_command(command.into())
    }
}

impl From<MemoryCommands> for MemoryCommand {
    fn from(cmd: MemoryCommands) -> Self {
        match cmd {
            MemoryCommands::Analyze(args) => MemoryCommand::Analyze {
                dump: args.dump,
                output: args.output,
                json: args.json,
            },
            MemoryCommands::Pslist(args) => MemoryCommand::PsList { dump: args.dump },
            MemoryCommands::Pstree(args) => MemoryCommand::PsTree { dump: args.dump },
            MemoryCommands::Malfind(args) => MemoryCommand::Malfind { dump: args.dump },
            MemoryCommands::Netscan(args) => MemoryCommand::NetScan { dump: args.dump },
            MemoryCommands::Hashdump(args) => MemoryCommand::HashDump { dump: args.dump },
            MemoryCommands::Hivelist(args) => MemoryCommand::Hivelist { dump: args.dump },
            MemoryCommands::Printkey(args) => MemoryCommand::PrintKey {
                dump: args.dump,
                key: args.key,
            },
            MemoryCommands::Yarascan(args) => MemoryCommand::YaraScan {
                dump: args.dump,
                rules: args.rules,
            },
            MemoryCommands::Mitre(args) => MemoryCommand::Mitre {
                dump: args.dump,
                output: args.output,
            },
        }
    }
}
