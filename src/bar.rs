//! Bar charts - horizontal and vertical with sub-cell precision

use crate::color::colorize;

/// Horizontal bar sub-cell blocks (0/8 to 8/8 of cell width)
const H_BLOCKS: [&str; 9] = [" ", "\u{258F}", "\u{258E}", "\u{258D}", "\u{258C}",
                              "\u{258B}", "\u{258A}", "\u{2589}", "\u{2588}"];

/// Vertical bar sub-cell blocks (0/8 to 8/8 of cell height)
const V_BLOCKS: [&str; 9] = [" ", "\u{2581}", "\u{2582}", "\u{2583}", "\u{2584}",
                              "\u{2585}", "\u{2586}", "\u{2587}", "\u{2588}"];

#[derive(Clone, Copy, PartialEq)]
pub enum Orientation { Horizontal, Vertical }

struct BarItem {
    label: String,
    value: f64,
    color: Option<u8>,
}

pub struct Bar {
    width: usize,
    height: usize,
    orientation: Orientation,
    items: Vec<BarItem>,
}

impl Bar {
    pub fn new(width: usize, orientation: Orientation) -> Self {
        Bar { width, height: 15, orientation, items: Vec::new() }
    }

    pub fn set_height(&mut self, h: usize) { self.height = h; }

    pub fn add(&mut self, label: &str, value: f64, color: Option<u8>) {
        self.items.push(BarItem { label: label.to_string(), value, color });
    }

    pub fn render(&self) -> String {
        if self.items.is_empty() { return String::new(); }
        match self.orientation {
            Orientation::Horizontal => self.render_horizontal(),
            Orientation::Vertical => self.render_vertical(),
        }
    }

    fn render_horizontal(&self) -> String {
        let max_val = self.items.iter().map(|i| i.value).fold(0.0_f64, f64::max);
        if max_val == 0.0 { return String::new(); }

        let label_w = self.items.iter().map(|i| i.label.len()).max().unwrap_or(0);
        let val_w = self.items.iter()
            .map(|i| format_val(i.value).len())
            .max().unwrap_or(0);
        let bar_w = self.width.saturating_sub(label_w + val_w + 4);

        let mut lines = Vec::new();
        for item in &self.items {
            let frac = item.value / max_val;
            let total_eighths = (frac * bar_w as f64 * 8.0).round() as usize;
            let full = total_eighths / 8;
            let eighth = total_eighths % 8;

            let mut bar_str = "\u{2588}".repeat(full);
            if eighth > 0 && full < bar_w {
                bar_str.push_str(H_BLOCKS[eighth]);
            }
            // Pad to bar_w
            let vis_len = full + if eighth > 0 { 1 } else { 0 };
            if vis_len < bar_w {
                bar_str.push_str(&" ".repeat(bar_w - vis_len));
            }

            let colored_bar = match item.color {
                Some(c) => colorize(&bar_str, c),
                None => bar_str,
            };
            let val_str = format_val(item.value);
            lines.push(format!("{:>label_w$} \u{2502}{} {:>val_w$}",
                item.label, colored_bar, val_str,
                label_w = label_w, val_w = val_w));
        }
        lines.join("\n")
    }

    fn render_vertical(&self) -> String {
        let max_val = self.items.iter().map(|i| i.value).fold(0.0_f64, f64::max);
        if max_val == 0.0 { return String::new(); }

        let col_w = self.items.iter().map(|i| i.label.len().max(3)).max().unwrap_or(3);
        let h = self.height;

        // Build columns
        let columns: Vec<Vec<String>> = self.items.iter().map(|item| {
            let frac = item.value / max_val;
            let total_eighths = (frac * h as f64 * 8.0).round() as usize;
            let full_rows = total_eighths / 8;
            let eighth = total_eighths % 8;

            let mut col = Vec::new();
            for row in 0..h {
                let from_bottom = h - 1 - row;
                let cell = if from_bottom < full_rows {
                    "\u{2588}".to_string()
                } else if from_bottom == full_rows && eighth > 0 {
                    V_BLOCKS[eighth].to_string()
                } else {
                    " ".to_string()
                };
                let colored = match item.color {
                    Some(c) if cell != " " => colorize(&cell, c),
                    _ => cell,
                };
                col.push(format!("{:^col_w$}", colored, col_w = col_w));
            }
            col
        }).collect();

        let mut lines = Vec::new();
        for row in 0..h {
            let line: Vec<&str> = columns.iter().map(|c| c[row].as_str()).collect();
            lines.push(line.join(" "));
        }
        // Label row
        let labels: Vec<String> = self.items.iter()
            .map(|i| format!("{:^col_w$}", i.label, col_w = col_w))
            .collect();
        lines.push(labels.join(" "));
        lines.join("\n")
    }
}

fn format_val(v: f64) -> String {
    if v == (v as i64) as f64 {
        format!("{}", v as i64)
    } else {
        format!("{:.2}", v)
    }
}
