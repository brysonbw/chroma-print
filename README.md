# ChromaPrint

A lightweight utility for styled terminal printing using ANSI escape codes.

[![Crates.io](https://img.shields.io/crates/v/chroma-print?style=flat-square)](https://crates.io/crates/chroma-print)
[![Crates.io](https://img.shields.io/crates/d/chroma-print?style=flat-square)](https://crates.io/crates/chroma-print)

## Install

```bash
cargo add chroma-print
```

## Usage

```rust
use chroma_print::{ChromaPrint, print_error, print_info, print_success, print_warn};
fn main() {
    // Using the provided macros for convenient styled printing:
    print_success!("This is a success message!");
    print_info!("This is an info message!");
    print_warn!("This is a warning message!");
    print_error!("This is an error message!");

    // Alternatively, you can use the ChromaPrint struct directly:
    println!("{}", ChromaPrint::success("Success!"));
    println!("{}", ChromaPrint::info("Info!"));
    println!("{}", ChromaPrint::warn("Warning!"));
    eprintln!("{}", ChromaPrint::error("Error!"));
}
```
