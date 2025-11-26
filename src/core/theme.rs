use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorScheme {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Typography {
    pub header_font_size: u8,
    pub body_font_size: u8,
    pub monospace_font: String,
}

impl Default for Typography {
    fn default() -> Self {
        Self::new()
    }
}

impl Typography {
    pub fn new() -> Self {
        Self {
            header_font_size: 18,
            body_font_size: 12,
            monospace_font: "Fira Code".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutSettings {
    pub border_style: String,
    pub padding: u8,
    pub margin: u8,
}

impl Default for LayoutSettings {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutSettings {
    pub fn new() -> Self {
        Self {
            border_style: "rounded".into(),
            padding: 1,
            margin: 1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconSet {
    pub success_icon: String,
    pub error_icon: String,
    pub warning_icon: String,
    pub info_icon: String,
}

impl IconSet {
    pub fn predator_icons() -> Self {
        Self {
            success_icon: "✅".into(),
            error_icon: "❌".into(),
            warning_icon: "⚠️".into(),
            info_icon: "ℹ️".into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MixedPredatorTheme {
    pub colors: ColorScheme,
    pub typography: Typography,
    pub layout: LayoutSettings,
    pub icons: IconSet,
}

impl Default for MixedPredatorTheme {
    fn default() -> Self {
        Self::new()
    }
}

impl MixedPredatorTheme {
    pub fn new() -> Self {
        Self {
            colors: ColorScheme {
                primary: RgbColor {
                    r: 34,
                    g: 34,
                    b: 34,
                },
                secondary: RgbColor {
                    r: 76,
                    g: 175,
                    b: 80,
                },
                accent: RgbColor {
                    r: 255,
                    g: 87,
                    b: 34,
                },
                background: RgbColor {
                    r: 13,
                    g: 17,
                    b: 23,
                },
                surface: RgbColor {
                    r: 22,
                    g: 27,
                    b: 34,
                },
                error: RgbColor {
                    r: 244,
                    g: 67,
                    b: 54,
                },
                warning: RgbColor {
                    r: 255,
                    g: 152,
                    b: 0,
                },
                success: RgbColor {
                    r: 76,
                    g: 175,
                    b: 80,
                },
                info: RgbColor {
                    r: 33,
                    g: 150,
                    b: 243,
                },
            },
            typography: Typography::new(),
            layout: LayoutSettings::new(),
            icons: IconSet::predator_icons(),
        }
    }

    pub fn apply_to_ui(&self) {
        // Currently noop but reserved for future advanced UI theming hooks.
    }
}
