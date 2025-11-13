use crate::tui::dashboard::DoctorStatus;
use crate::tui::theme::FeroxTheme;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

pub fn render_doctor_panel(
    frame: &mut Frame<'_>,
    area: Rect,
    theme: &FeroxTheme,
    doctor: &DoctorStatus,
    focused: bool,
) {
    let border_style = if focused {
        theme.styles.highlight
    } else {
        theme.styles.border
    };

    let block = Block::default()
        .title(theme.block_title("Maintenance / Ferox Doctor"))
        .borders(Borders::ALL)
        .border_style(border_style)
        .style(theme.surface_style());

    let lines = vec![
        info_line("Doctor Status", doctor.status, theme),
        info_line("Open Findings", doctor.open_findings.to_string(), theme),
        info_line("Last Audit", doctor.last_audit, theme),
        info_line("Patch Window", doctor.patch_window, theme),
    ];

    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: true })
        .style(theme.surface_style());

    frame.render_widget(paragraph, area);
}

fn info_line<'a>(label: &'a str, value: impl Into<String>, theme: &FeroxTheme) -> Line<'a> {
    let value = value.into();
    Line::from(vec![
        Span::styled(format!("{label:<16}: "), theme.styles.text),
        Span::styled(value, theme.styles.metric),
    ])
}
