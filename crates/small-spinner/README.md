# Ratatui Small Spinner

A simple Ratatui widget for displaying a single-character animated spinner.

## Run Example

```bash
cargo run --example showcase --features examples
```

## Usage

Create and render a spinner with a custom style:

```rust
use std::time::Duration;

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Position, Rect},
    style::Color,
    widgets::Widget,
};
use ratatui_small_spinner::{
    SmallSpinnerStyleBuilder,
    SmallSpinnerType,
    SmallSpinnerWidget,
};

let spinner_style = SmallSpinnerStyleBuilder::default()
    .with_type(SmallSpinnerType::BrailleDouble)
    .with_interval(Duration::from_millis(0))
    .with_alignment(Alignment::Right)
    .with_foreground_color(Color::White)
    .with_background_color(Color::Black)
    .build()
    .unwrap();
let mut spinner = SmallSpinnerWidget::new(spinner_style);
```

On each `render` call, the spinner moves to the next character in its sequence. Increasing the interval slows down the update rate, creating smoother animation.
