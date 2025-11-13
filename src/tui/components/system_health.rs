use crate::tui::dashboard::SystemHealth;
use crate::tui::theme::FeroxTheme;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::symbols;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Gauge, Paragraph, Sparkline};

pub fn render_system_health(
    frame: &mut Frame<'_>,
    area: Rect,
    theme: &FeroxTheme,
    system: &SystemHealth,
) {
    let block = Block::default()
        .title(theme.block_title("System Health"))
        .borders(Borders::ALL)
        .border_style(theme.styles.border)
        .style(theme.surface_style());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .split(inner);

    render_gauge_with_sparkline(
        frame,
        rows[0],
        theme,
        system.cpu_utilization,
        &system.cpu_history,
        "CPU",
        theme.styles.metric,
        theme.palette.predator_orange,
        true,
    );

    render_gauge_with_sparkline(
        frame,
        rows[1],
        theme,
        system.memory_usage,
        &system.memory_history,
        "Memory",
        theme.styles.metric,
        theme.palette.toxic_green,
        false,
    );

    render_gauge_with_sparkline(
        frame,
        rows[2],
        theme,
        system.detection_surface,
        &system.detection_history,
        "Detection",
        theme.styles.metric,
        theme.palette.signal_yellow,
        false,
    );

    let integrity_line = Line::from(vec![
        Span::styled("Integrity Score: ", theme.styles.text),
        Span::styled(
            format!("{} / 100", system.integrity_score),
            theme.styles.metric,
        ),
        Span::raw("   Threat Level: "),
        Span::styled(system.threat_level, theme.styles.warning),
    ]);
    let paragraph = Paragraph::new(integrity_line).style(theme.surface_style());
    frame.render_widget(paragraph, rows[3]);
}

fn render_gauge_with_sparkline(
    frame: &mut Frame<'_>,
    area: Rect,
    theme: &FeroxTheme,
    ratio: f64,
    history: &[u64],
    label: &str,
    label_style: Style,
    gauge_color: Color,
    signed: bool,
) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(area);

    let value = if signed {
        format!("{label} {:+.1}%", ratio * 100.0)
    } else {
        format!("{label} {:.1}%", ratio * 100.0)
    };

    let gauge = Gauge::default()
        .ratio(ratio)
        .label(Span::styled(value, label_style))
        .gauge_style(
            theme
                .surface_style()
                .fg(gauge_color)
                .bg(theme.palette.carbon_black),
        );
    frame.render_widget(gauge, chunks[0]);

    if !history.is_empty() {
        let spark = Sparkline::default()
            .data(history)
            .style(theme.styles.metric)
            .max(100)
            .bar_set(symbols::bar::NINE_LEVELS);
        frame.render_widget(spark, chunks[1]);
    }
}
