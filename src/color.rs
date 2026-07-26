//! ANSI color utilities

use crust::style;

/// Color specification: 256-color code, RGB hex, or named color
#[derive(Clone, Copy)]
pub enum Color {
    Index(u8),
    Rgb(u8, u8, u8),
}

impl Color {
    /// Parse from various formats: u8, &str name, or "#RRGGBB" hex
    pub fn from_str(s: &str) -> Option<Color> {
        // Hex: #RRGGBB or RRGGBB
        let hex = s.trim_start_matches('#');
        if hex.len() == 6 && hex.chars().all(|c| c.is_ascii_hexdigit()) {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            return Some(Color::Rgb(r, g, b));
        }
        // Named colors (matching termchart)
        match s.to_lowercase().as_str() {
            "red" => Some(Color::Index(196)),
            "green" => Some(Color::Index(82)),
            "blue" => Some(Color::Index(33)),
            "yellow" => Some(Color::Index(226)),
            "cyan" => Some(Color::Index(51)),
            "magenta" => Some(Color::Index(201)),
            "white" => Some(Color::Index(255)),
            "gray" | "grey" => Some(Color::Index(245)),
            "orange" => Some(Color::Index(208)),
            _ => None,
        }
    }

    /// ANSI foreground escape code (without reset)
    pub fn fg_code(&self) -> String {
        match self {
            Color::Index(c) => style::set_fg(*c),
            Color::Rgb(r, g, b) => style::set_fg_rgb(*r, *g, *b),
        }
    }

    /// ANSI background escape code (without reset)
    pub fn bg_code(&self) -> String {
        match self {
            Color::Index(c) => style::set_bg(*c),
            Color::Rgb(r, g, b) => style::set_bg_rgb(*r, *g, *b),
        }
    }
}

impl From<u8> for Color {
    fn from(c: u8) -> Self { Color::Index(c) }
}

/// Wrap text in ANSI foreground color (256-color index)
pub fn colorize(text: &str, color: u8) -> String {
    style::styled(text, Some(color), None, "")
}

/// Wrap text in ANSI foreground color (Color enum)
pub fn colorize_color(text: &str, color: &Color) -> String {
    format!("{}{}{}", color.fg_code(), text, style::RESET)
}

/// Format a number for axis labels with adaptive precision
pub fn format_num(v: f64) -> String {
    if v.abs() >= 1000.0 {
        format!("{:.0}", v)
    } else if v.abs() >= 100.0 {
        format!("{:.1}", v)
    } else if v.abs() >= 1.0 {
        format!("{:.1}", v)
    } else {
        format!("{:.2}", v)
    }
}
