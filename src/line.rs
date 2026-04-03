//! Line charts - high-resolution braille dot plotting with Y-axis labels

use crate::canvas::BrailleCanvas;
use crate::color::{colorize, format_num};

struct Series {
    values: Vec<f64>,
    color: u8,
    label: Option<String>,
}

pub struct Line {
    pub width: usize,
    pub height: usize,
    series: Vec<Series>,
}

impl Line {
    pub fn new(width: usize, height: usize) -> Self {
        Line { width, height, series: Vec::new() }
    }

    pub fn add(&mut self, values: &[f64], color: u8, label: Option<&str>) {
        self.series.push(Series {
            values: values.to_vec(),
            color,
            label: label.map(|s| s.to_string()),
        });
    }

    pub fn render(&self) -> String {
        if self.series.is_empty() { return String::new(); }

        // Global min/max
        let mut data_min = f64::INFINITY;
        let mut data_max = f64::NEG_INFINITY;
        for s in &self.series {
            for &v in &s.values {
                if v < data_min { data_min = v; }
                if v > data_max { data_max = v; }
            }
        }
        let data_range = if data_max == data_min { 1.0 } else { data_max - data_min };

        // Y-axis label width
        let labels = [format_num(data_max), format_num((data_max + data_min) / 2.0), format_num(data_min)];
        let y_label_w = labels.iter().map(|l| l.len()).max().unwrap_or(0) + 2; // +2 for " \u{2524}"

        // Chart area
        let chart_w = self.width.saturating_sub(y_label_w);
        let chart_h = self.height;
        let mut canvas = BrailleCanvas::new(chart_w, chart_h);
        let px_w = canvas.px_width();
        let px_h = canvas.px_height();

        // Plot each series
        for s in &self.series {
            let n = s.values.len();
            if n < 2 { continue; }

            let mut prev_x = 0usize;
            let mut prev_y = 0usize;

            for (i, &v) in s.values.iter().enumerate() {
                let px = if n > 1 { i * (px_w - 1) / (n - 1) } else { 0 };
                let py = ((data_max - v) / data_range * (px_h - 1) as f64).round() as usize;
                let py = py.min(px_h - 1);

                if i > 0 {
                    // Draw line from prev to current using Bresenham
                    bresenham(prev_x, prev_y, px, py, |x, y| {
                        canvas.set_dot(x, y, Some(s.color));
                    });
                }
                prev_x = px;
                prev_y = py;
            }
        }

        // Render canvas lines
        let chart_text = canvas.render();
        let chart_lines: Vec<&str> = chart_text.lines().collect();

        // Build output with Y-axis labels
        let mut lines = Vec::new();
        for row in 0..chart_h {
            let label = if row == 0 {
                format_num(data_max)
            } else if row == chart_h / 2 {
                format_num((data_max + data_min) / 2.0)
            } else if row == chart_h - 1 {
                format_num(data_min)
            } else {
                String::new()
            };
            let axis = if label.is_empty() {
                format!("{:>w$}\u{2502}", "", w = y_label_w - 1)
            } else {
                format!("{:>w$}\u{2524}", label, w = y_label_w - 1)
            };
            let chart_line = chart_lines.get(row).unwrap_or(&"");
            lines.push(format!("{}{}", axis, chart_line));
        }

        // Legend
        let has_labels = self.series.iter().any(|s| s.label.is_some());
        if has_labels {
            lines.push(String::new());
            let legend: Vec<String> = self.series.iter()
                .filter_map(|s| {
                    s.label.as_ref().map(|l| {
                        format!("{} {}", colorize("\u{2501}\u{2501}", s.color), l)
                    })
                })
                .collect();
            lines.push(format!("{:>w$}{}", "", legend.join("  "), w = y_label_w));
        }

        lines.join("\n")
    }
}

/// Bresenham line algorithm
fn bresenham<F: FnMut(usize, usize)>(x0: usize, y0: usize, x1: usize, y1: usize, mut plot: F) {
    let (mut x0, mut y0) = (x0 as i32, y0 as i32);
    let (x1, y1) = (x1 as i32, y1 as i32);
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        if x0 >= 0 && y0 >= 0 {
            plot(x0 as usize, y0 as usize);
        }
        if x0 == x1 && y0 == y1 { break; }
        let e2 = 2 * err;
        if e2 >= dy { err += dy; x0 += sx; }
        if e2 <= dx { err += dx; y0 += sy; }
    }
}
