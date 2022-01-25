use std::fmt::Display;

use crate::color::Color;

/// Edge options
/// 
/// # Examples
/// ```
/// use network_vis::edge_options::EdgeOptions;
/// 
/// let edge_options = EdgeOptions::RGB(255, 0, 0);
/// ```
pub enum EdgeOptions<'a> {
    Color(Color<'a>),
    Name(&'a str),
    Inherit(&'a str),
    Opacity(f32),
    Highlight(u8, u8, u8),
    Highlighta(u8, u8, u8, f32),
    HighlightName(&'a str),
}

/// Edge options for color
#[allow(non_snake_case)]
impl<'a> EdgeOptions<'a> {
    pub fn Hex(hex: &'a str) -> Self {
        EdgeOptions::Color(Color::Hex(hex))
    }
    pub fn RGB(r: u8, g: u8, b: u8) -> Self {
        EdgeOptions::Color(Color::RGB(r, g, b))
    }
    pub fn RGBa(r: u8, g: u8, b: u8, a: f32) -> Self {
        EdgeOptions::Color(Color::RGBa(r, g, b, a))
    }
}

impl<'a> Display for EdgeOptions<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            EdgeOptions::Color(color) => format!("{color}"),
            EdgeOptions::Name(name) => format!("color: \"{name}\","),
            EdgeOptions::Inherit(name) => format!("inherit: \"{name}\","),
            EdgeOptions::Opacity(a) => format!("opacity: {a},"),
            EdgeOptions::Highlight(r, g, b) => format!("hightlight: \"rgb({r}, {g}, {b}) \","),
            EdgeOptions::Highlighta(r, g, b, a) => format!("highlight: \"rgba({r}, {g}, {b}, {a}) \","),
            EdgeOptions::HighlightName(name) => format!("highlight: \"{name}\","),
        })
    }
}