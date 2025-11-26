use colored::Colorize;

use crate::core::theme::{IconSet, MixedPredatorTheme, RgbColor};

/// Applies the Mixed Predator theme to CLI output helpers.
pub struct CliThemeApplier {
    theme: MixedPredatorTheme,
}

impl CliThemeApplier {
    pub fn new(theme: MixedPredatorTheme) -> Self {
        Self { theme }
    }

    pub fn icons(&self) -> &IconSet {
        &self.theme.icons
    }

    pub fn apply_colors(&self) {
        // Placeholder for advanced terminal theming. Right now we rely on formatted strings.
    }

    pub fn format_section_header(&self, text: &str) -> String {
        self.truecolor(text, self.theme.colors.accent)
            .bold()
            .to_string()
    }

    pub fn format_success(&self, text: &str) -> String {
        self.truecolor(text, self.theme.colors.success)
            .bold()
            .to_string()
    }

    pub fn format_error(&self, text: &str) -> String {
        self.truecolor(text, self.theme.colors.error)
            .bold()
            .to_string()
    }

    pub fn format_warning(&self, text: &str) -> String {
        self.truecolor(text, self.theme.colors.warning)
            .bold()
            .to_string()
    }

    pub fn format_hint(&self, text: &str) -> String {
        format!(
            "{} {}",
            self.icons().info_icon,
            self.truecolor(text, self.theme.colors.info)
        )
    }

    pub fn success_icon(&self) -> &str {
        &self.icons().success_icon
    }

    pub fn error_icon(&self) -> &str {
        &self.icons().error_icon
    }

    pub fn warning_icon(&self) -> &str {
        &self.icons().warning_icon
    }

    pub fn info_icon(&self) -> &str {
        &self.icons().info_icon
    }

    fn truecolor(&self, text: &str, color: RgbColor) -> colored::ColoredString {
        text.truecolor(color.r, color.g, color.b)
    }
}
