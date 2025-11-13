use crate::tui::dashboard::DashboardSnapshot;
use crate::tui::theme::{FeroxTheme, ThemeMode};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

pub fn render_header(
    frame: &mut Frame<'_>,
    area: Rect,
    theme: &FeroxTheme,
    snapshot: &DashboardSnapshot,
    mode: ThemeMode,
) {
    let block = Block::default()
        .title(Span::styled(
            "Ferox Predator Command Surface",
            theme.styles.title,
        ))
        .borders(Borders::ALL)
        .border_style(theme.styles.border)
        .style(theme.surface_style());

    let info_line = Line::from(vec![
        Span::styled("Version 2.0.0", theme.styles.metric),
        Span::raw(" • Modules Online: "),
        Span::styled(snapshot.module_total().to_string(), theme.styles.metric),
        Span::raw(" • Integrity: "),
        Span::styled(
            format!("{}%", snapshot.system.integrity_score),
            theme.styles.metric,
        ),
        Span::raw(" • Theme: "),
        Span::styled(mode.label(), theme.styles.metric),
    ]);

    let paragraph = Paragraph::new(vec![info_line])
        .block(block)
        .style(theme.surface_style());

    frame.render_widget(paragraph, area);
}
