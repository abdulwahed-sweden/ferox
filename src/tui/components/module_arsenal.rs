use crate::tui::dashboard::{DashboardSnapshot, ModuleRecord};
use crate::tui::theme::FeroxTheme;
use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::widgets::{Block, Borders, Cell, Row, Table};

pub fn render_module_arsenal(
    frame: &mut Frame<'_>,
    area: Rect,
    theme: &FeroxTheme,
    snapshot: &DashboardSnapshot,
    focused: bool,
) {
    let border_style = if focused {
        theme.styles.highlight
    } else {
        theme.styles.border
    };

    let block = Block::default()
        .title(theme.block_title("Module Arsenal"))
        .borders(Borders::ALL)
        .border_style(border_style)
        .style(theme.surface_style());

    let rows: Vec<Row<'static>> = snapshot
        .modules
        .iter()
        .map(|record| module_row(record, theme))
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(20),
            Constraint::Length(12),
            Constraint::Min(20),
        ],
    )
    .block(block)
    .header(
        Row::new(vec![
            Cell::from("Domain"),
            Cell::from("Operational"),
            Cell::from("Highlights"),
        ])
        .style(theme.styles.section_header)
        .bottom_margin(1),
    )
    .column_spacing(2);

    frame.render_widget(table, area);
}

fn module_row(record: &ModuleRecord, theme: &FeroxTheme) -> Row<'static> {
    Row::new(vec![
        Cell::from(record.domain),
        Cell::from(record.operational.to_string()).style(theme.styles.metric),
        Cell::from(record.highlights),
    ])
}
