use std::error::Error;
use std::io::{stdout, IsTerminal};
use std::time::Duration;

use crossterm::ExecutableCommand;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ferox::tui::dashboard::{FeroxDashboard, PanelFocus};
use ferox::tui::theme::ThemeMode;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    // In CI/non-interactive environments, fail fast with a friendly message instead of an EPERM.
    if !stdout.is_terminal() {
        println!("Predator dashboard requires an interactive TTY; skipping TUI launch.");
        return Ok(());
    }

    enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let mut dashboard = FeroxDashboard::new();

    run_app(&mut terminal, &mut dashboard)?;

    disable_raw_mode()?;
    terminal.backend_mut().execute(LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    dashboard: &mut FeroxDashboard,
) -> Result<(), Box<dyn Error>> {
    let tick_rate = Duration::from_millis(250);
    loop {
        terminal.draw(|frame| dashboard.render(frame))?;

        if event::poll(tick_rate)?
            && let Event::Key(key) = event::read()?
                && key.kind == KeyEventKind::Press && handle_key(dashboard, key.code) {
                    break;
                }
    }
    Ok(())
}

fn handle_key(dashboard: &mut FeroxDashboard, code: KeyCode) -> bool {
    match code {
        KeyCode::Char(ch) => match ch.to_ascii_lowercase() {
            'q' => return true,
            'm' => dashboard.focus_panel(PanelFocus::Modules),
            'd' => dashboard.focus_panel(PanelFocus::Doctor),
            'c' => dashboard.focus_panel(PanelFocus::C2),
            'l' => dashboard.focus_panel(PanelFocus::Logs),
            'r' => dashboard.refresh(),
            't' => dashboard.cycle_theme(),
            '1' => dashboard.set_theme_mode(ThemeMode::Predator),
            '2' => dashboard.set_theme_mode(ThemeMode::NightOps),
            '3' => dashboard.set_theme_mode(ThemeMode::GhostWire),
            '4' => dashboard.set_theme_mode(ThemeMode::SolarAxiom),
            _ => {}
        },
        KeyCode::Esc | KeyCode::Enter => return true,
        _ => {}
    }
    false
}
