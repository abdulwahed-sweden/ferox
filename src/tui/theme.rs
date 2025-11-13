use ratatui::style::{Color, Modifier, Style};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeMode {
    Predator,
    NightOps,
    GhostWire,
    SolarAxiom,
}

impl ThemeMode {
    pub const ALL: [ThemeMode; 4] = [
        ThemeMode::Predator,
        ThemeMode::NightOps,
        ThemeMode::GhostWire,
        ThemeMode::SolarAxiom,
    ];

    pub fn label(self) -> &'static str {
        match self {
            ThemeMode::Predator => "Predator",
            ThemeMode::NightOps => "NightOps",
            ThemeMode::GhostWire => "GhostWire",
            ThemeMode::SolarAxiom => "Solar Axiom",
        }
    }
}

#[derive(Clone, Debug)]
pub struct ThemePalette {
    pub electric_cyan: Color,
    pub plasma_purple: Color,
    pub predator_orange: Color,
    pub signal_yellow: Color,
    pub hazard_red: Color,
    pub toxic_green: Color,
    pub carbon_black: Color,
    pub night_blue: Color,
    pub soft_gray: Color,
    pub surface: Color,
}

impl ThemePalette {
    pub fn for_mode(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::Predator => Self {
                electric_cyan: Color::Rgb(0, 229, 255),
                plasma_purple: Color::Rgb(138, 0, 255),
                predator_orange: Color::Rgb(255, 122, 0),
                signal_yellow: Color::Rgb(255, 214, 10),
                hazard_red: Color::Rgb(255, 0, 51),
                toxic_green: Color::Rgb(0, 255, 156),
                carbon_black: Color::Rgb(0, 0, 0),
                night_blue: Color::Rgb(13, 27, 42),
                soft_gray: Color::Rgb(200, 200, 200),
                surface: Color::Rgb(13, 27, 42),
            },
            ThemeMode::NightOps => Self {
                electric_cyan: Color::Rgb(0, 191, 255),
                plasma_purple: Color::Rgb(64, 0, 128),
                predator_orange: Color::Rgb(255, 94, 0),
                signal_yellow: Color::Rgb(255, 199, 0),
                hazard_red: Color::Rgb(255, 71, 87),
                toxic_green: Color::Rgb(0, 255, 120),
                carbon_black: Color::Rgb(5, 7, 12),
                night_blue: Color::Rgb(10, 18, 30),
                soft_gray: Color::Rgb(210, 210, 210),
                surface: Color::Rgb(18, 30, 46),
            },
            ThemeMode::GhostWire => Self {
                electric_cyan: Color::Rgb(124, 252, 255),
                plasma_purple: Color::Rgb(176, 0, 255),
                predator_orange: Color::Rgb(255, 140, 0),
                signal_yellow: Color::Rgb(255, 240, 150),
                hazard_red: Color::Rgb(255, 105, 180),
                toxic_green: Color::Rgb(144, 238, 144),
                carbon_black: Color::Rgb(12, 0, 24),
                night_blue: Color::Rgb(24, 8, 50),
                soft_gray: Color::Rgb(230, 230, 255),
                surface: Color::Rgb(40, 12, 72),
            },
            ThemeMode::SolarAxiom => Self {
                electric_cyan: Color::Rgb(64, 224, 208),
                plasma_purple: Color::Rgb(255, 64, 129),
                predator_orange: Color::Rgb(255, 171, 0),
                signal_yellow: Color::Rgb(255, 241, 118),
                hazard_red: Color::Rgb(255, 82, 82),
                toxic_green: Color::Rgb(118, 255, 3),
                carbon_black: Color::Rgb(18, 8, 2),
                night_blue: Color::Rgb(40, 20, 8),
                soft_gray: Color::Rgb(255, 248, 225),
                surface: Color::Rgb(55, 28, 12),
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct ThemeStyles {
    pub title: Style,
    pub section_header: Style,
    pub success: Style,
    pub warning: Style,
    pub error: Style,
    pub highlight: Style,
    pub text: Style,
    pub border: Style,
    pub metric: Style,
    pub hotkey: Style,
}

impl ThemeStyles {
    pub fn new(palette: &ThemePalette) -> Self {
        let bold = Modifier::BOLD;
        Self {
            title: Style::default()
                .fg(palette.electric_cyan)
                .add_modifier(bold),
            section_header: Style::default()
                .fg(palette.plasma_purple)
                .add_modifier(bold),
            success: Style::default().fg(palette.toxic_green).add_modifier(bold),
            warning: Style::default()
                .fg(palette.signal_yellow)
                .add_modifier(bold),
            error: Style::default().fg(palette.hazard_red).add_modifier(bold),
            highlight: Style::default()
                .fg(palette.predator_orange)
                .add_modifier(bold),
            text: Style::default().fg(palette.soft_gray),
            border: Style::default().fg(palette.electric_cyan),
            metric: Style::default()
                .fg(palette.electric_cyan)
                .add_modifier(bold),
            hotkey: Style::default()
                .fg(palette.predator_orange)
                .add_modifier(bold),
        }
    }
}

#[derive(Clone, Debug)]
pub struct FeroxTheme {
    pub mode: ThemeMode,
    pub palette: ThemePalette,
    pub styles: ThemeStyles,
}

impl FeroxTheme {
    pub fn new(mode: ThemeMode) -> Self {
        let palette = ThemePalette::for_mode(mode);
        let styles = ThemeStyles::new(&palette);
        Self {
            mode,
            palette,
            styles,
        }
    }

    pub fn with_mode(mode: ThemeMode) -> Self {
        Self::new(mode)
    }

    pub fn set_mode(&mut self, mode: ThemeMode) {
        if self.mode != mode {
            *self = Self::new(mode);
        }
    }

    pub fn surface_style(&self) -> Style {
        Style::default()
            .bg(self.palette.surface)
            .fg(self.palette.soft_gray)
    }

    pub fn block_title<'a>(&self, title: &'a str) -> ratatui::text::Span<'a> {
        ratatui::text::Span::styled(title, self.styles.section_header)
    }
}

impl Default for FeroxTheme {
    fn default() -> Self {
        Self::new(ThemeMode::Predator)
    }
}
