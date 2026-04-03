//! Sparklines - compact single-line charts using eighth-block characters

use crate::color::{colorize, colorize_color, Color};

const TICKS: [char; 8] = ['\u{2581}', '\u{2582}', '\u{2583}', '\u{2584}',
                           '\u{2585}', '\u{2586}', '\u{2587}', '\u{2588}'];

/// Render a sparkline from numeric values
///
/// Returns a single-line string using ▁▂▃▄▅▆▇█ characters.
/// Optionally colored with a 256-color code.
pub fn spark(values: &[f64], color: Option<u8>) -> String {
    if values.is_empty() { return String::new(); }

    let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = max - min;

    let s: String = values.iter().map(|&v| {
        let idx = if range == 0.0 {
            3 // middle for flat data
        } else {
            ((v - min) / range * 7.0).round() as usize
        };
        TICKS[idx.min(7)]
    }).collect();

    match color {
        Some(c) => colorize(&s, c),
        None => s,
    }
}

/// Render a sparkline with Color enum support
pub fn spark_color(values: &[f64], color: Option<&Color>) -> String {
    if values.is_empty() { return String::new(); }

    let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = max - min;

    let s: String = values.iter().map(|&v| {
        let idx = if range == 0.0 { 3 } else {
            ((v - min) / range * 7.0).round() as usize
        };
        TICKS[idx.min(7)]
    }).collect();

    match color {
        Some(c) => colorize_color(&s, c),
        None => s,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spark_basic() {
        let s = spark(&[1.0, 5.0, 3.0, 7.0, 2.0], None);
        assert_eq!(s.len(), 5 * 3); // 5 chars, each 3 bytes (UTF-8 block chars)
    }

    #[test]
    fn test_spark_flat() {
        let s = spark(&[5.0, 5.0, 5.0], None);
        assert!(s.contains('\u{2584}')); // middle tick
    }

    #[test]
    fn test_spark_empty() {
        assert_eq!(spark(&[], None), "");
    }
}
