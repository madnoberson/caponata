# Ratatui Small Spinner 

A simple Ratatui widget for displaying a single-character animated spinner.

## Usage

Create and render a spinner with a custom style:

```rust
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
    .with_alignment(Alignment::Right)
    .with_foreground_color(Color::White)
    .with_background_color(Color::Black)
    .build()
    .unwrap();
let mut spinner = SmallSpinnerWidget::new(spinner_style);

let area = Rect::new(0, 0, 5, 1);
let mut buf = Buffer::empty(area);
let spinner_cell_position = Position::new(4, 0);

spinner.render(area, &mut buf);
let spinner_cell = buf.cell(spinner_cell_position).unwrap();
assert_eq!(spinner_cell.symbol(), "⠘");

spinner.render(area, &mut buf);
let spinner_cell = buf.cell(spinner_cell_position).unwrap();
assert_eq!(spinner_cell.symbol(), "⠰");

spinner.render(area, &mut buf);
let spinner_cell = buf.cell(spinner_cell_position).unwrap();
assert_eq!(spinner_cell.symbol(), "⠤");

spinner.render(area, &mut buf);
let spinner_cell = buf.cell(spinner_cell_position).unwrap();
assert_eq!(spinner_cell.symbol(), "⠆");

spinner.render(area, &mut buf);
let spinner_cell = buf.cell(spinner_cell_position).unwrap();
assert_eq!(spinner_cell.symbol(), "⠃");

spinner.render(area, &mut buf);
let spinner_cell = buf.cell(spinner_cell_position).unwrap();
assert_eq!(spinner_cell.symbol(), "⠉");

spinner.render(area, &mut buf);
let spinner_cell = buf.cell(spinner_cell_position).unwrap();
assert_eq!(spinner_cell.symbol(), "⠘");
```
