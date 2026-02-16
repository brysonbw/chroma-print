use crate::colors::Color;

/// A utility struct for printing colored messages to the terminal.
pub struct ChromaPrint;

impl ChromaPrint {
    /// Paints the given text with the specified color
    pub fn paint(text: &str, color: Color) -> String {
        return format!("{}{}{}", color.value(), text, Color::Reset.value());
    }

    /// Print a success message (green)
    pub fn success(text: &str) -> String {
        return Self::paint(text, Color::Green);
    }

    /// Print an info message (cyan)
    pub fn info(text: &str) -> String {
        return Self::paint(text, Color::Cyan);
    }

    /// Print a warning message (yellow)
    pub fn warn(text: &str) -> String {
        return Self::paint(text, Color::Yellow);
    }

    /// Print an error message (red)
    pub fn error(text: &str) -> String {
        return Self::paint(text, Color::Red);
    }
}
