/// Defines the `Color` enum and its associated ANSI escape codes for terminal coloring
pub enum Color {
    Green,
    Yellow,
    Red,
    Cyan,
    Reset,
}

impl Color {
    /// Returns the ANSI escape code for the color
    pub fn value(&self) -> &'static str {
        return match self {
            Color::Green => "\x1b[32m",
            Color::Yellow => "\x1b[33m",
            Color::Red => "\x1b[31m",
            Color::Cyan => "\x1b[36m",
            Color::Reset => "\x1b[0m",
        };
    }
}
