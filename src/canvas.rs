//! Canvas - grid-based character renderer with color support
//! BrailleCanvas - high-resolution sub-cell drawing using Unicode braille

use crust::style;

/// Simple character grid with per-cell color
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    cells: Vec<Vec<(String, Option<u8>, Option<u8>)>>, // (char, fg, bg)
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = (0..height)
            .map(|_| (0..width).map(|_| (" ".to_string(), None, None)).collect())
            .collect();
        Canvas { width, height, cells }
    }

    pub fn set(&mut self, x: usize, y: usize, ch: &str, fg: Option<u8>, bg: Option<u8>) {
        if x < self.width && y < self.height {
            self.cells[y][x] = (ch.to_string(), fg, bg);
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&(String, Option<u8>, Option<u8>)> {
        self.cells.get(y)?.get(x)
    }

    pub fn render(&self) -> String {
        let mut lines = Vec::new();
        for row in &self.cells {
            let mut line = String::new();
            let mut cur_fg: Option<u8> = None;
            let mut cur_bg: Option<u8> = None;
            let mut has_color = false;

            for (ch, fg, bg) in row {
                if *fg != cur_fg || *bg != cur_bg {
                    // Only emit reset + new codes when color actually changes
                    if has_color {
                        line.push_str(style::RESET);
                    }
                    let mut codes = Vec::new();
                    if let Some(f) = fg { codes.push(format!("38;5;{}", f)); }
                    if let Some(b) = bg { codes.push(format!("48;5;{}", b)); }
                    if !codes.is_empty() {
                        line.push_str(&style::sgr(&codes.join(";")));
                        has_color = true;
                    } else {
                        has_color = false;
                    }
                    cur_fg = *fg;
                    cur_bg = *bg;
                }
                line.push_str(ch);
            }
            if has_color {
                line.push_str(style::RESET);
            }
            lines.push(line);
        }
        lines.join("\n")
    }
}

/// High-resolution braille canvas (2x4 dots per character cell)
///
/// Each cell maps to a 2-column, 4-row dot grid using Unicode braille (U+2800-U+28FF).
pub struct BrailleCanvas {
    pub width: usize,  // character columns
    pub height: usize, // character rows
    dots: Vec<Vec<u8>>,           // dot bit patterns [height][width]
    colors: Vec<Vec<Option<u8>>>, // per-cell fg color
}

// Braille dot bit positions:
// Col:  0     1
// Row 0: 0x01  0x08
// Row 1: 0x02  0x10
// Row 2: 0x04  0x20
// Row 3: 0x40  0x80
const DOT_MAP: [[u8; 2]; 4] = [
    [0x01, 0x08],
    [0x02, 0x10],
    [0x04, 0x20],
    [0x40, 0x80],
];

impl BrailleCanvas {
    pub fn new(width: usize, height: usize) -> Self {
        BrailleCanvas {
            width,
            height,
            dots: vec![vec![0u8; width]; height],
            colors: vec![vec![None; width]; height],
        }
    }

    /// Pixel resolution: width*2 x height*4
    pub fn px_width(&self) -> usize { self.width * 2 }
    pub fn px_height(&self) -> usize { self.height * 4 }

    /// Set a single dot at pixel coordinates
    pub fn set_dot(&mut self, px: usize, py: usize, fg: Option<u8>) {
        let cx = px / 2;
        let cy = py / 4;
        if cx >= self.width || cy >= self.height { return; }
        let dx = px % 2;
        let dy = py % 4;
        self.dots[cy][cx] |= DOT_MAP[dy][dx];
        if fg.is_some() {
            self.colors[cy][cx] = fg;
        }
    }

    /// Render braille dots to a Canvas, then to string
    pub fn render(&self) -> String {
        let mut canvas = Canvas::new(self.width, self.height);
        for cy in 0..self.height {
            for cx in 0..self.width {
                let pattern = self.dots[cy][cx];
                let ch = char::from_u32(0x2800 + pattern as u32).unwrap_or(' ');
                canvas.set(cx, cy, &ch.to_string(), self.colors[cy][cx], None);
            }
        }
        canvas.render()
    }
}
