//! # plot - Terminal charts with Unicode and ANSI colors
//!
//! Rust clone of [termchart](https://github.com/isene/termchart).
//! Renders sparklines, line charts, candlestick charts, and bar charts
//! directly in the terminal using Unicode block/braille characters.
//!
//! All chart types produce plain strings with ANSI color codes.
//! No dependencies.
//!
//! ## Quick Start
//! ```
//! use plot::{spark, Bar, Orientation, Line, Candle};
//!
//! // Sparkline
//! let s = spark(&[1.0, 3.0, 7.0, 2.0, 5.0, 8.0, 4.0], None);
//! println!("{}", s);
//!
//! // Bar chart
//! let mut bar = Bar::new(50, Orientation::Horizontal);
//! bar.add("Rust", 95.0, Some(208));
//! bar.add("Ruby", 80.0, Some(196));
//! println!("{}", bar.render());
//! ```

pub mod color;
pub mod spark;
pub mod bar;
pub mod canvas;
pub mod line;
pub mod candle;

pub use spark::{spark, spark_color};
pub use color::Color;
pub use bar::{Bar, Orientation};
pub use canvas::{Canvas, BrailleCanvas};
pub use line::Line;
pub use candle::Candle;
