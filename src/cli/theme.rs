//! Ferox theming (modernized)
//! Provides a unified, capability-aware interface for colored output, Unicode symbols,
//! and plain fallbacks. Maintains backward compatibility with previous Theme API.

use anyhow::Result;
use is_terminal::IsTerminal;
use indicatif::ProgressStyle;
use owo_colors::{AnsiColors, OwoColorize};
use std::env;
use std::sync::{OnceLock, RwLock};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum DisplayProfile {
    Rich,
    Minimal,
    Plain,
    Compact,
}

#[derive(Debug, Clone)]
pub struct Symbols {
    pub fox: &'static str,
    pub ok: &'static str,
    pub warn: &'static str,
    pub err: &'static str,
    pub arrow: &'static str,
    pub line: &'static str,
}

impl Symbols {
    fn for_profile(p: DisplayProfile) -> Self {
        match p {
            DisplayProfile::Rich => Self {
                fox: "🦊",
                ok: "✓",
                warn: "⚠",
                err: "✗",
                arrow: "→",
                line: "─",
            },
            DisplayProfile::Minimal => Self {
                fox: "[fox]",
                ok: "[+]",
                warn: "[!]",
                err: "[x]",
                arrow: "->",
                line: "-",
            },
            DisplayProfile::Plain | DisplayProfile::Compact => Self {
                fox: "",
                ok: "+",
                warn: "!",
                err: "x",
                arrow: "->",
                line: "-",
            },
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct ThemeState {
    profile: DisplayProfile,
    use_color: bool,
    use_unicode: bool,
    is_tty: bool,
    symbols: Symbols,
}

static THEME: OnceLock<RwLock<ThemeState>> = OnceLock::new();

pub struct Theme;

impl Theme {
    pub fn init() {
        let s = Self::detect();
        let _ = THEME.set(RwLock::new(s));
    }
    #[allow(dead_code)]
    pub fn refresh() {
        if let Some(lock) = THEME.get() {
            *lock.write().unwrap() = Self::detect();
        }
    }
    fn detect() -> ThemeState {
        #[cfg(windows)]
        let _ = enable_ansi_support();
        let no_color = env::var_os("NO_COLOR").is_some();
        let no_emoji = env::var_os("NO_EMOJI").is_some();
        let is_ci = env::var_os("CI").is_some();
        let is_tty = std::io::stdout().is_terminal();
        let utf8 = env::var("LANG")
            .unwrap_or_default()
            .to_ascii_lowercase()
            .contains("utf-8");
        let use_color = !no_color && is_tty;
        let use_unicode = !no_emoji && utf8 && !is_ci;
        let profile = if !is_tty || no_color || is_ci {
            DisplayProfile::Plain
        } else if use_color && use_unicode {
            DisplayProfile::Rich
        } else if use_color {
            DisplayProfile::Minimal
        } else {
            DisplayProfile::Plain
        };
        ThemeState {
            profile,
            use_color,
            use_unicode,
            is_tty,
            symbols: Symbols::for_profile(profile),
        }
    }
    fn state() -> ThemeState {
        THEME
            .get()
            .expect("Theme::init not called")
            .read()
            .unwrap()
            .clone()
    }

    pub fn banner() {
        let s = Self::state();
        match s.profile {
            DisplayProfile::Rich => {
                println!(
                    "{} {}",
                    s.symbols.fox,
                    "Ferox Framework".color(AnsiColors::BrightCyan)
                );
                println!("{}", "====================".color(AnsiColors::BrightBlack));
            }
            DisplayProfile::Minimal => {
                println!("[Ferox] Framework");
                println!(
                    "{}",
                    if s.use_color {
                        "================"
                            .color(AnsiColors::BrightBlack)
                            .to_string()
                    } else {
                        "================".to_string()
                    }
                );
            }
            _ => println!("Ferox Framework"),
        }
    }

    pub fn success(msg: &str) {
        Self::print_marker(msg, AnsiColors::Green, "✓", "[+]");
    }
    pub fn error(msg: &str) {
        Self::print_marker(msg, AnsiColors::Red, "✗", "[-]");
    }
    pub fn warning(msg: &str) {
        Self::print_marker(msg, AnsiColors::Yellow, "⚠", "[!]");
    }
    pub fn info(msg: &str) {
        Self::print_marker(msg, AnsiColors::Blue, "ℹ", "[*]");
    }

    pub fn module_header(name: &str) {
        let st = Self::state();
        let line = if st.use_unicode { "═" } else { "=" };
        let border = line.repeat(70);
        if st.use_color {
            println!(
                "\n{}\n  {} {}\n{}",
                border.color(AnsiColors::Blue),
                st.symbols.fox.color(AnsiColors::Blue),
                name.color(AnsiColors::White).bold(),
                border.color(AnsiColors::Blue)
            );
        } else {
            println!("\n{}\n  {} {}\n{}", border, st.symbols.fox, name, border);
        }
    }

    pub fn section(title: &str) {
        let st = Self::state();
        let line = st.symbols.line.repeat(18);
        let text = format!("{}[ {} ]{}", line, title, line);
        if st.use_color {
            println!("\n{}", text.color(AnsiColors::Cyan));
        } else {
            println!("\n{}", text);
        }
    }

    pub fn prompt(context: &str) -> String {
        let st = Self::state();
        let base = if context.is_empty() {
            "ferox".to_string()
        } else {
            format!("ferox({})", context)
        };
        let p = format!("{}>", base);
        if st.use_color {
            p.color(AnsiColors::Red).bold().to_string() + " "
        } else {
            p + " "
        }
    }

    pub fn command_help(cmd: &str, desc: &str) {
        let st = Self::state();
        let arrow = st.symbols.arrow;
        if st.use_color {
            println!(
                "    {}  {}  {}",
                cmd.color(AnsiColors::Green).bold(),
                arrow.color(AnsiColors::Cyan),
                desc.color(AnsiColors::White)
            );
        } else {
            println!("    {}  {}  {}", cmd, arrow, desc);
        }
    }

    pub fn status(kind: &str, message: &str) {
        let st = Self::state();
        let (sym, color) = match kind {
            "ready" => (st.symbols.ok, AnsiColors::Green),
            "running" => (st.symbols.warn, AnsiColors::Yellow),
            "error" => (st.symbols.err, AnsiColors::Red),
            _ => (st.symbols.arrow, AnsiColors::Blue),
        };
        if st.use_color {
            println!(
                "  {}  {}: {}",
                sym.color(color),
                kind.to_uppercase().color(color).bold(),
                message.color(AnsiColors::White)
            );
        } else {
            println!("  {}  {}: {}", sym, kind.to_uppercase(), message);
        }
    }

    pub fn spinner_style() -> ProgressStyle {
        let st = Self::state();
        if st.use_color {
            ProgressStyle::default_spinner()
                .template("{spinner:.red} {msg}")
                .unwrap()
                .tick_strings(if st.use_unicode {
                    &["🦊", "🔥", "⚡", "💥", "✨", "🎯", "🚀", "⚔️"]
                } else {
                    &["-", "\\", "|", "/"]
                })
        } else {
            ProgressStyle::default_spinner()
                .template("{spinner} {msg}")
                .unwrap()
                .tick_strings(&["-", "\\", "|", "/"])
        }
    }

    fn print_marker(message: &str, color: AnsiColors, unicode: &str, ascii: &str) {
        let st = Self::state();
        let sym = if st.use_unicode { unicode } else { ascii };
        if st.use_color {
            println!(
                "{} {}",
                sym.color(color).bold(),
                message.color(AnsiColors::White)
            );
        } else {
            println!("{} {}", sym, message);
        }
    }
}

#[cfg(windows)]
pub fn enable_ansi_support() -> Result<()> {
    use tracing::warn;
    use winapi::shared::minwindef::DWORD;
    use winapi::um::consoleapi::{GetConsoleMode, SetConsoleMode};
    use winapi::um::handleapi::INVALID_HANDLE_VALUE;
    use winapi::um::processenv::GetStdHandle;
    use winapi::um::winbase::{STD_ERROR_HANDLE, STD_OUTPUT_HANDLE};
    use winapi::um::wincon::ENABLE_VIRTUAL_TERMINAL_PROCESSING;

    let handles = [STD_OUTPUT_HANDLE, STD_ERROR_HANDLE];
    let mut any_enabled = false;
    unsafe {
        for std_handle in handles {
            let handle = GetStdHandle(std_handle);
            if handle.is_null() || handle == INVALID_HANDLE_VALUE {
                continue;
            }
            let mut mode: DWORD = 0;
            if GetConsoleMode(handle, &mut mode) == 0 {
                continue;
            }
            if mode & ENABLE_VIRTUAL_TERMINAL_PROCESSING == 0 {
                if SetConsoleMode(handle, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING) == 0 {
                    continue;
                }
            }
            any_enabled = true;
        }
    }
    if !any_enabled {
        warn!("ANSI escape sequences not supported; falling back to plain output");
    }
    Ok(())
}
#[cfg(not(windows))]
pub fn enable_ansi_support() -> Result<()> {
    Ok(())
}
