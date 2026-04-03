# Plot

Rust feature clone of [termchart](https://github.com/isene/termchart), a terminal charting library.

Renders sparklines, line charts (braille), candlestick (OHLC), and bar charts (horizontal + vertical) with Unicode block characters and ANSI 256-color / RGB support. Zero dependencies.

## Build

```bash
PATH="/usr/bin:$PATH" cargo build --release
```

Note: `PATH` prefix needed to avoid `~/bin/cc` (Claude Code sessions) shadowing the C compiler.

## Architecture

```
src/
  lib.rs      - Public API, re-exports
  color.rs    - Color enum (256/RGB/named), colorize, format_num
  spark.rs    - Sparklines (▁▂▃▄▅▆▇█)
  canvas.rs   - Canvas + BrailleCanvas (2x4 braille dots)
  line.rs     - Line charts (braille, Bresenham, Y-axis, legend)
  candle.rs   - Candlestick OHLC (wicks, bodies, doji)
  bar.rs      - Horizontal + vertical bars (1/8th sub-cell precision)
```

## Testing

```bash
PATH="/usr/bin:$PATH" cargo test
PATH="/usr/bin:$PATH" cargo run --release --example demo
```
