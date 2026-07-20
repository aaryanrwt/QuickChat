use ratatui::style::Color;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Theme {
    pub primary_accent: ColorConfig,
    pub error_text: ColorConfig,
    pub code_block_bg: ColorConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ColorConfig {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ColorConfig {
    pub fn to_ratatui_color(&self) -> Color {
        Color::Rgb(self.r, self.g, self.b)
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            primary_accent: ColorConfig {
                r: 88,
                g: 101,
                b: 242,
            }, // Discord-like blurple
            error_text: ColorConfig {
                r: 237,
                g: 66,
                b: 69,
            },
            code_block_bg: ColorConfig {
                r: 43,
                g: 45,
                b: 49,
            },
        }
    }
}

pub fn parse_theme(toml_str: &str) -> Result<Theme, toml::de::Error> {
    toml::from_str(toml_str)
}
