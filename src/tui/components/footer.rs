use crate::tui::dashboard::C2Telemetry;
use crate::tui::theme::{FeroxTheme, ThemeMode};
use ratatui::Frame;
use ratatui::layout::{Alignment, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

pub fn render_footer(
    frame: &mut Frame<'_>,
    area: Rect,
    theme: &FeroxTheme,
    c2: &C2Telemetry,
    mode: ThemeMode,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme.styles.border)
        .style(theme.surface_style());

    let line = Line::from(vec![
        Span::styled("M", theme.styles.hotkey),
        Span::raw(" Modules  "),
        Span::styled("D", theme.styles.hotkey),
        Span::raw(" Doctor  "),
        Span::styled("C", theme.styles.hotkey),
        Span::raw(" C2  "),
        Span::styled("L", theme.styles.hotkey),
        Span::raw(" Logs  "),
        Span::styled("R", theme.styles.hotkey),
        Span::raw(" Refresh  "),
        Span::styled("1-4", theme.styles.hotkey),
        Span::raw(" Themes  •  Session: "),
        Span::styled(c2.session_id, theme.styles.metric),
        Span::raw("  •  Theme: "),
        Span::styled(mode.label(), theme.styles.metric),
        Span::raw("  •  Q/Enter/Esc to exit"),
    ]);

    let paragraph = Paragraph::new(line)
        .block(block)
        .alignment(Alignment::Center)
        .style(theme.surface_style());

    frame.render_widget(paragraph, area);
}
