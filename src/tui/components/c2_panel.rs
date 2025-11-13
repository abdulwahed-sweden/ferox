use crate::tui::dashboard::C2Telemetry;
use crate::tui::theme::FeroxTheme;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

pub fn render_c2_panel(
    frame: &mut Frame<'_>,
    area: Rect,
    theme: &FeroxTheme,
    c2: &C2Telemetry,
    focused: bool,
) {
    let border_style = if focused {
        theme.styles.highlight
    } else {
        theme.styles.border
    };

    let block = Block::default()
        .title(theme.block_title("C2 / Operations"))
        .borders(Borders::ALL)
        .border_style(border_style)
        .style(theme.surface_style());

    let lines = vec![
        Line::from(vec![
            Span::styled("Active Sessions: ", theme.styles.text),
            Span::styled(c2.active_sessions.to_string(), theme.styles.metric),
            Span::raw("  |  Live Beacons: "),
            Span::styled(c2.live_beacons.to_string(), theme.styles.metric),
        ]),
        Line::from(vec![
            Span::styled("Last Sync: ", theme.styles.text),
            Span::styled(c2.last_sync, theme.styles.metric),
            Span::raw("  |  Exfiltration: "),
            Span::styled(
                format!("{:.1} MB/s", c2.exfiltration_rate_mbps),
                theme.styles.metric,
            ),
        ]),
        Line::from(vec![
            Span::styled("OPSEC Directive: ", theme.styles.text),
            Span::styled(c2.opsec_directive, theme.styles.warning),
        ]),
    ];

    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: true })
        .style(theme.surface_style());

    frame.render_widget(paragraph, area);
}
