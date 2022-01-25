use std::fmt::Display;

pub enum Color<'a> {
    RGB(u8, u8, u8),
    RGBa(u8, u8, u8, f32),
    Hex(&'a str),
}

impl<'a> Display for Color<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Color::RGB(r, g, b) => format!("color: \"rgb({r}, {g}, {b}) \","),
            Color::RGBa(r, g, b, a) => format!("color: \"rgba({r}, {g}, {b}, {a})\","),
            Color::Hex(hex) => format!("color: \"{hex}\","),
        })
    }
}