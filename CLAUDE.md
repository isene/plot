# Plot

Rust feature clone of [termchart](https://github.com/isene/termchart), a terminal charting library.

Renders bar charts, line charts, and sparklines in the terminal with ANSI colors. Used by Tock, Kastrup, and Rush for data visualization.

## Build

```bash
PATH="/usr/bin:$PATH" cargo build --release
```

Note: `PATH` prefix needed to avoid `~/bin/cc` (Claude Code sessions) shadowing the C compiler.
