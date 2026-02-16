//! > **A lightweight utility for styled terminal printing using ANSI escape codes.**
//!
//! ## Install
//! ```console
//! $ cargo add chroma-print
//! ```
//! ## Usage
//! ```rust
//! use chroma_print::{ChromaPrint, print_error, print_info, print_success, print_warn};
//! fn main() {
//!     // Using the provided macros for convenient styled printing:
//!     print_success!("This is a success message!");
//!     print_info!("This is an info message!");
//!     print_warn!("This is a warning message!");
//!     print_error!("This is an error message!");
//!     
//!     // Alternatively, you can use the ChromaPrint struct directly:
//!     println!("{}", ChromaPrint::success("Success!"));
//!     println!("{}", ChromaPrint::info("Info!"));
//!     println!("{}", ChromaPrint::warn("Warning!"));
//!     eprintln!("{}", ChromaPrint::error("Error!"));
//! }   
//! ```

pub mod colors;
pub mod printer;

pub use colors::Color;
pub use printer::ChromaPrint;

// Macros
#[macro_export]
/// Macro for printing a success message (green)
macro_rules! print_success {
    ($($arg:tt)*) => {
        println!("{}", $crate::ChromaPrint::success(&format!($($arg)*)))
    };
}

#[macro_export]
/// Macro for printing an info message (cyan)
macro_rules! print_info {
    ($($arg:tt)*) => {
        println!("{}", $crate::ChromaPrint::info(&format!($($arg)*)))
    };
}

#[macro_export]
/// Macro for printing a warning message (yellow)
macro_rules! print_warn {
    ($($arg:tt)*) => {
        println!("{}", $crate::ChromaPrint::warn(&format!($($arg)*)))
    };
}

#[macro_export]
/// Macro for printing an error message (red)
macro_rules! print_error {
    ($($arg:tt)*) => {
        eprintln!("{}", $crate::ChromaPrint::error(&format!($($arg)*)))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESET: &str = "\x1b[0m";

    #[test]
    fn test_color_values() {
        assert_eq!(Color::Green.value(), "\x1b[32m");
        assert_eq!(Color::Yellow.value(), "\x1b[33m");
        assert_eq!(Color::Cyan.value(), "\x1b[36m");
        assert_eq!(Color::Red.value(), "\x1b[31m");
        assert_eq!(Color::Reset.value(), "\x1b[0m");
    }

    #[test]
    fn test_success_formatting() {
        let msg: &str = "success";
        assert_eq!(
            ChromaPrint::success(msg),
            format!("\x1b[32msuccess{}", RESET)
        );
    }

    #[test]
    fn test_error_formatting() {
        let msg: &str = "error";
        assert_eq!(ChromaPrint::error(msg), format!("\x1b[31merror{}", RESET));
    }

    #[test]
    fn test_info_formatting() {
        let msg = "info";
        assert_eq!(ChromaPrint::info(msg), format!("\x1b[36minfo{}", RESET));
    }

    #[test]
    fn test_warn_formatting() {
        let msg: &str = "warning";
        assert_eq!(ChromaPrint::warn(msg), format!("\x1b[33mwarning{}", RESET));
    }
}
