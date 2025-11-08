use atty::Stream;
use colored::{Color, Colorize, control};
use indicatif::ProgressStyle;
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{LazyLock, RwLock};

use anyhow::Result;

#[derive(Copy, Clone)]
struct ThemeState {
    colors: bool,
    unicode: bool,
}

#[cfg(windows)]
static ANSI_VT_ENABLED: AtomicBool = AtomicBool::new(false);
#[cfg(not(windows))]
static ANSI_VT_ENABLED: AtomicBool = AtomicBool::new(true);

static THEME_STATE: LazyLock<RwLock<ThemeState>> = LazyLock::new(|| {
    let state = detect_theme_state();
    apply_color_settings(state);
    RwLock::new(state)
});

pub struct Theme;

impl Theme {
    /// Initialize theme capabilities after terminal setup.
    pub fn init() {
        let state = detect_theme_state();
        if let Ok(mut guard) = THEME_STATE.write() {
            *guard = state;
        }
        apply_color_settings(state);
    }

    /// Display the main Ferox banner.
    pub fn banner() {
        let state = Theme::state();
        if state.colors {
            let banner_template = if state.unicode {
                COLORED_BANNER
            } else {
                PLAIN_BANNER
            };
            println!("{}", banner_template.bright_red());

            let fox = themed_symbol(state, "🦊", "[+]");
            let bolt = themed_symbol(state, "⚡", "[*]");

            println!(
                "    {}   Ferox Framework {}",
                paint_symbol(fox.as_str(), Color::BrightRed, state, true),
                paint_text("v2.0.0", Color::BrightYellow, state, true)
            );
            println!(
                "    {}   Fast. Fierce. Fearless.\n",
                paint_symbol(bolt.as_str(), Color::BrightYellow, state, true)
            );
        } else {
            println!("{}", PLAIN_BANNER);
            let fox = themed_symbol(state, "🦊", "[+]");
            let bolt = themed_symbol(state, "⚡", "[*]");
            println!("{} Ferox Framework v2.0.0", fox);
            println!("{} Fast. Fierce. Fearless.\n", bolt);
        }
    }

    /// Success message.
    pub fn success(msg: &str) {
        Theme::print_marker(msg, Color::BrightGreen, "✓", "[+]");
    }

    /// Error message.
    pub fn error(msg: &str) {
        Theme::print_marker(msg, Color::BrightRed, "✗", "[-]");
    }

    /// Warning message.
    pub fn warning(msg: &str) {
        Theme::print_marker(msg, Color::BrightYellow, "⚠", "[!]");
    }

    /// Info message.
    pub fn info(msg: &str) {
        Theme::print_marker(msg, Color::BrightBlue, "ℹ", "[*]");
    }

    /// Module header.
    pub fn module_header(name: &str) {
        let state = Theme::state();
        let border_char = if state.unicode { "═" } else { "=" };
        let border = border_char.repeat(70);
        let box_icon = themed_symbol(state, "📦", "[*]");

        if state.colors {
            println!("\n{}", border.bright_blue());
            println!(
                "  {} {}",
                paint_symbol(box_icon.as_str(), Color::BrightBlue, state, false),
                paint_text(name, Color::BrightWhite, state, true)
            );
            println!("{}", border.bright_blue());
        } else {
            println!("\n{}", border);
            println!("  {} {}", box_icon, name);
            println!("{}", border);
        }
    }

    /// Section header.
    pub fn section(title: &str) {
        let state = Theme::state();
        let divider = if state.unicode {
            "──────────────────"
        } else {
            "------------------"
        };
        let formatted = format!("  {}[ {} ]{}", divider, title, divider);

        if state.colors {
            println!("\n{}", formatted.bright_cyan());
        } else {
            println!("\n{}", formatted);
        }
    }

    /// Prompt with Ferox branding.
    pub fn prompt(context: &str) -> String {
        let state = Theme::state();
        let base = if context.is_empty() {
            "ferox".to_string()
        } else {
            format!("ferox({})", context)
        };

        let prompt = format!("{}>", base);
        if state.colors {
            format!("{} ", prompt.bright_red().bold())
        } else {
            format!("{} ", prompt)
        }
    }

    /// Command help entry.
    pub fn command_help(cmd: &str, desc: &str) {
        let state = Theme::state();
        let arrow = if state.unicode { "→" } else { "->" };

        if state.colors {
            println!(
                "    {}  {}  {}",
                cmd.bright_green().bold(),
                arrow.bright_cyan(),
                desc.bright_white()
            );
        } else {
            println!("    {}  {}  {}", cmd, arrow, desc);
        }
    }

    /// Status indicator line.
    pub fn status(status: &str, message: &str) {
        let state = Theme::state();
        let (unicode, ascii, color) = match status {
            "ready" => ("🟢", "[READY]", Color::BrightGreen),
            "running" => ("🟡", "[RUN]", Color::BrightYellow),
            "error" => ("🔴", "[ERR]", Color::BrightRed),
            _ => ("⚪", "[INFO]", Color::White),
        };

        let icon = themed_symbol(state, unicode, ascii);
        let status_text = status.to_uppercase();

        if state.colors {
            println!(
                "  {}  {}: {}",
                paint_symbol(icon.as_str(), color, state, false),
                status_text.color(color).bold(),
                paint_text(message, Color::BrightWhite, state, false)
            );
        } else {
            println!("  {}  {}: {}", icon, status_text, message);
        }
    }

    /// Spinner style respecting terminal capabilities.
    pub fn spinner_style() -> ProgressStyle {
        let state = Theme::state();
        if state.colors {
            ProgressStyle::default_spinner()
                .template("{spinner:.red} {msg}")
                .unwrap()
                .tick_strings(if state.unicode {
                    &["🦊", "🔥", "⚡", "💥", "✨", "🎯", "🚀", "⚔️"]
                } else {
                    &["-", "\\", "|", "/"]
                })
        } else {
            ProgressStyle::default_spinner()
                .template("{spinner} {msg}")
                .unwrap()
                .tick_strings(if state.unicode {
                    &["*", "o", "O", "o"]
                } else {
                    &["-", "\\", "|", "/"]
                })
        }
    }

    fn print_marker(message: &str, color: Color, unicode_symbol: &str, ascii_symbol: &str) {
        let state = Theme::state();
        let symbol = themed_symbol(state, unicode_symbol, ascii_symbol);

        if state.colors {
            println!(
                "{} {}",
                paint_symbol(symbol.as_str(), color, state, true),
                paint_text(message, Color::BrightWhite, state, false)
            );
        } else {
            println!("{} {}", symbol, message);
        }
    }

    fn state() -> ThemeState {
        THEME_STATE
            .read()
            .map(|state| *state)
            .unwrap_or(ThemeState {
                colors: false,
                unicode: false,
            })
    }
}

/// Enable ANSI escape support on Windows consoles when available.
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

    if any_enabled {
        ANSI_VT_ENABLED.store(true, Ordering::Relaxed);
    } else {
        warn!("ANSI escape sequences not supported; using plain text fallback");
    }

    Ok(())
}

/// No-op on non-Windows platforms.
#[cfg(not(windows))]
pub fn enable_ansi_support() -> Result<()> {
    Ok(())
}

const COLORED_BANNER: &str = r#"
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
"#;

const PLAIN_BANNER: &str = r#"
============================================
   FEROX FRAMEWORK
   Ferocious Security Framework
============================================
"#;

fn themed_symbol(state: ThemeState, unicode_symbol: &str, ascii_symbol: &str) -> String {
    if state.unicode {
        unicode_symbol.to_string()
    } else {
        ascii_symbol.to_string()
    }
}

fn paint_symbol(symbol: &str, color: Color, state: ThemeState, bold: bool) -> String {
    if state.colors {
        if bold {
            symbol.color(color).bold().to_string()
        } else {
            symbol.color(color).to_string()
        }
    } else {
        symbol.to_string()
    }
}

fn paint_text(text: &str, color: Color, state: ThemeState, bold: bool) -> String {
    if state.colors {
        if bold {
            text.color(color).bold().to_string()
        } else {
            text.color(color).to_string()
        }
    } else {
        text.to_string()
    }
}

fn detect_theme_state() -> ThemeState {
    let ansi_enabled = ANSI_VT_ENABLED.load(Ordering::Relaxed);
    let no_color = env::var_os("NO_COLOR").is_some();
    let stdout_is_tty = atty::is(Stream::Stdout);
    let colors = !no_color && stdout_is_tty && ansi_enabled;
    let unicode = detect_unicode(ansi_enabled);

    ThemeState { colors, unicode }
}

fn detect_unicode(ansi_enabled: bool) -> bool {
    if env::var_os("NO_EMOJI").is_some() {
        return false;
    }

    if let Ok(term) = env::var("TERM") {
        if term.eq_ignore_ascii_case("dumb") {
            return false;
        }
    }

    if cfg!(windows) {
        if env::var_os("WT_SESSION").is_some()
            || env::var_os("ConEmuANSI").is_some()
            || env::var("TERM_PROGRAM")
                .map(|tp| tp.to_ascii_lowercase().contains("vscode"))
                .unwrap_or(false)
        {
            return true;
        }

        return ansi_enabled;
    }

    true
}

fn apply_color_settings(state: ThemeState) {
    if state.colors {
        control::unset_override();
        #[cfg(windows)]
        let _ = control::set_virtual_terminal(true);
    } else {
        control::set_override(false);
    }
}
