use crate::tui::dashboard::{ActivityEntry, LogLevel};
use crate::tui::theme::FeroxTheme;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem};

pub fn render_recent_activity(
    frame: &mut Frame<'_>,
    area: Rect,
    theme: &FeroxTheme,
    entries: &[ActivityEntry],
    focused: bool,
) {
    let border_style = if focused {
        theme.styles.highlight
    } else {
        theme.styles.border
    };

    let block = Block::default()
        .title(theme.block_title("Recent Activity"))
        .borders(Borders::ALL)
        .border_style(border_style)
        .style(theme.surface_style());

    let mut items: Vec<ListItem> = entries
        .iter()
        .map(|entry| ListItem::new(activity_line(entry, theme)))
        .collect();

    items.push(ListItem::new(Line::from(vec![Span::styled(
        " (Scroll area continues)",
        theme.styles.text,
    )])));

    let list = List::new(items).block(block).style(theme.surface_style());

    frame.render_widget(list, area);
}

fn activity_line(entry: &ActivityEntry, theme: &FeroxTheme) -> Line<'static> {
    Line::from(vec![
        Span::styled(entry.timestamp, theme.styles.metric),
        Span::raw("  "),
        level_span(entry.level, theme),
        Span::raw("  "),
        Span::styled(entry.message, theme.styles.text),
    ])
}

fn level_span(level: LogLevel, theme: &FeroxTheme) -> Span<'static> {
    match level {
        LogLevel::Info => Span::styled("INFO", theme.styles.title),
        LogLevel::Warn => Span::styled("WARN", theme.styles.warning),
        LogLevel::Success => Span::styled("SUCCESS", theme.styles.success),
    }
}
