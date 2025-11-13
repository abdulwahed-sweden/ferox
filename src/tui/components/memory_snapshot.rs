use crate::tui::dashboard::MemorySnapshot;
use crate::tui::theme::FeroxTheme;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

pub fn render_memory_snapshot(
    frame: &mut Frame<'_>,
    area: Rect,
    theme: &FeroxTheme,
    memory: &MemorySnapshot,
) {
    let block = Block::default()
        .title(theme.block_title("Memory Forensics Snapshot"))
        .borders(Borders::ALL)
        .border_style(theme.styles.border)
        .style(theme.surface_style());

    let lines = vec![
        metric_line("YARA Hits", memory.yara_hits, theme),
        metric_line(
            "Volatility Profiles",
            memory.volatility_profiles as u16,
            theme,
        ),
        metric_line("Credential Artifacts", memory.credential_artifacts, theme),
        metric_line("Active Dump Jobs", memory.active_dump_jobs as u16, theme),
    ];

    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: true })
        .style(theme.surface_style());

    frame.render_widget(paragraph, area);
}

fn metric_line<'a>(label: &'a str, value: u16, theme: &FeroxTheme) -> Line<'a> {
    Line::from(vec![
        Span::styled(format!("{label:<20}: "), theme.styles.text),
        Span::styled(value.to_string(), theme.styles.metric),
    ])
}
