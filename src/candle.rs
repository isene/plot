//! Candlestick (OHLC) charts for financial data

use crate::canvas::Canvas;
use crate::color::{colorize, format_num};

const UP_COLOR: u8 = 82;   // green
const DOWN_COLOR: u8 = 196; // red

#[derive(Clone)]
pub struct OhlcBar {
    pub o: f64, pub h: f64, pub l: f64, pub c: f64,
}

pub struct Candle {
    pub width: usize,
    pub height: usize,
    data: Vec<OhlcBar>,
}

impl Candle {
    pub fn new(width: usize, height: usize) -> Self {
        Candle { width, height, data: Vec::new() }
    }

    pub fn add(&mut self, bars: &[OhlcBar]) {
        for bar in bars {
            assert!(bar.h >= bar.l, "High must be >= Low");
            self.data.push(bar.clone());
        }
    }

    pub fn render(&self) -> String {
        if self.data.is_empty() { return String::new(); }

        let data_min = self.data.iter().map(|b| b.l).fold(f64::INFINITY, f64::min);
        let data_max = self.data.iter().map(|b| b.h).fold(f64::NEG_INFINITY, f64::max);
        let data_range = if data_max == data_min { 1.0 } else { data_max - data_min };

        // Y-axis labels
        let labels: Vec<String> = (0..5).map(|i| {
            format_num(data_max - data_range * i as f64 / 4.0)
        }).collect();
        let y_label_w = labels.iter().map(|l| l.len()).max().unwrap_or(0) + 2;

        let chart_w = self.width.saturating_sub(y_label_w);
        let chart_h = self.height;
        let max_candles = chart_w / 2;
        let visible: Vec<&OhlcBar> = if self.data.len() > max_candles {
            self.data[self.data.len() - max_candles..].iter().collect()
        } else {
            self.data.iter().collect()
        };

        let mut canvas = Canvas::new(chart_w, chart_h);

        // Map price to row
        let price_to_row = |price: f64| -> usize {
            let frac = (data_max - price) / data_range;
            (frac * (chart_h - 1) as f64).round() as usize
        };

        for (i, bar) in visible.iter().enumerate() {
            let col = i * 2;
            if col >= chart_w { break; }

            let is_up = bar.c >= bar.o;
            let color = if is_up { UP_COLOR } else { DOWN_COLOR };

            let high_row = price_to_row(bar.h);
            let low_row = price_to_row(bar.l);
            let body_top = price_to_row(bar.o.max(bar.c));
            let body_bot = price_to_row(bar.o.min(bar.c));

            // Wick above body
            for row in high_row..body_top {
                canvas.set(col, row, "\u{2502}", Some(color), None);
            }
            // Body
            if body_top == body_bot {
                // Doji
                canvas.set(col, body_top, "\u{2500}", Some(color), None);
            } else {
                for row in body_top..=body_bot {
                    canvas.set(col, row, "\u{2588}", Some(color), None);
                }
            }
            // Wick below body
            for row in (body_bot + 1)..=low_row {
                canvas.set(col, row, "\u{2502}", Some(color), None);
            }
        }

        // Render with Y-axis
        let chart_text = canvas.render();
        let chart_lines: Vec<&str> = chart_text.lines().collect();

        let label_rows: Vec<usize> = vec![0, chart_h / 4, chart_h / 2, chart_h * 3 / 4, chart_h - 1];
        let mut lines = Vec::new();
        for row in 0..chart_h {
            let label_idx = label_rows.iter().position(|&r| r == row);
            let label = label_idx.map(|i| labels[i].clone()).unwrap_or_default();
            let axis = if label.is_empty() {
                format!("{:>w$}\u{2502}", "", w = y_label_w - 1)
            } else {
                format!("{:>w$}\u{2524}", label, w = y_label_w - 1)
            };
            let chart_line = chart_lines.get(row).unwrap_or(&"");
            lines.push(format!("{}{}", axis, chart_line));
        }

        lines.join("\n")
    }
}
