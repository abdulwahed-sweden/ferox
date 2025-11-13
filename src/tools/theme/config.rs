use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::Deserialize;

use crate::core::theme::{
    ColorScheme, IconSet, LayoutSettings, MixedPredatorTheme, RgbColor, Typography,
};

#[derive(Debug, Deserialize)]
pub struct ThemeConfig {
    pub colors: ColorConfig,
    pub typography: TypographyConfig,
    pub layout: LayoutConfig,
    pub icons: IconConfig,
}

#[derive(Debug, Deserialize)]
pub struct ColorConfig {
    pub primary: RgbColor,
    pub secondary: RgbColor,
    pub accent: RgbColor,
    pub background: RgbColor,
    pub surface: RgbColor,
    pub error: RgbColor,
    pub warning: RgbColor,
    pub success: RgbColor,
    pub info: RgbColor,
}

#[derive(Debug, Deserialize)]
pub struct TypographyConfig {
    pub header_font_size: u8,
    pub body_font_size: u8,
    pub monospace_font: String,
}

#[derive(Debug, Deserialize)]
pub struct LayoutConfig {
    pub border_style: String,
    pub padding: u8,
    pub margin: u8,
}

#[derive(Debug, Deserialize)]
pub struct IconConfig {
    pub success_icon: String,
    pub error_icon: String,
    pub warning_icon: String,
    pub info_icon: String,
}

impl ThemeConfig {
    pub fn load_default() -> Result<Self> {
        let path = find_theme_file();
        let data = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read theme file at {}", path.display()))?;
        let config: ThemeConfig = toml::from_str(&data)?;
        Ok(config)
    }

    pub fn into_theme(self) -> MixedPredatorTheme {
        MixedPredatorTheme {
            colors: ColorScheme {
                primary: self.colors.primary,
                secondary: self.colors.secondary,
                accent: self.colors.accent,
                background: self.colors.background,
                surface: self.colors.surface,
                error: self.colors.error,
                warning: self.colors.warning,
                success: self.colors.success,
                info: self.colors.info,
            },
            typography: Typography {
                header_font_size: self.typography.header_font_size,
                body_font_size: self.typography.body_font_size,
                monospace_font: self.typography.monospace_font,
            },
            layout: LayoutSettings {
                border_style: self.layout.border_style,
                padding: self.layout.padding,
                margin: self.layout.margin,
            },
            icons: IconSet {
                success_icon: self.icons.success_icon,
                error_icon: self.icons.error_icon,
                warning_icon: self.icons.warning_icon,
                info_icon: self.icons.info_icon,
            },
        }
    }
}

fn find_theme_file() -> PathBuf {
    let local = Path::new("ferox_theme.toml");
    if local.exists() {
        return local.to_path_buf();
    }

    if let Some(home) = dirs::home_dir() {
        let fallback = home.join(".config/ferox/ferox_theme.toml");
        if fallback.exists() {
            return fallback;
        }
    }

    local.to_path_buf()
}
