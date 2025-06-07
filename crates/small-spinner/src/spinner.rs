use derivative::Derivative;
use ratatui::{
    buffer::Buffer,
    layout::{
        Alignment,
        Rect,
    },
    widgets::Widget,
};

use super::{
    SmallSpinnerStyle,
    symbol_cycle_by_spinner_type,
};

/// A widget that displays single-character animated spinner.
///
/// The [`SmallSpinnerWidget`] renders a single-character spinner
/// that cycles through a sequence of symbols based on the provided
/// [`SmallSpinnerStyle`].
///
/// # Important
///
/// The `Clone` implementation creates a new spinner with the same
/// style but resets the symbol cycle to the beginning, not preserving
/// the current animation state.
///
/// # Example
///
/// ```rust
/// use ratatui::{
///     buffer::Buffer,
///     layout::{Alignment, Position, Rect},
///     style::Color,
///     widgets::Widget,
/// };
/// use ratatui_small_spinner::{
///     SmallSpinnerType,
///     SmallSpinnerStyleBuilder,
///     SmallSpinnerWidget,
/// };
///
/// let spinner_style = SmallSpinnerStyleBuilder::default()
///     .with_type(SmallSpinnerType::BrailleDouble)
///     .with_alignment(Alignment::Right)
///     .with_foreground_color(Color::White)
///     .with_background_color(Color::Black)
///     .build()
///     .unwrap();
/// let mut spinner = SmallSpinnerWidget::new(spinner_style);
///
/// let area = Rect::new(0, 0, 5, 1);
/// let mut buf = Buffer::empty(area);
/// let spinner_cell_position = Position::new(4, 0);
///
/// spinner.render(area, &mut buf);
/// let spinner_cell = buf.cell(spinner_cell_position).unwrap();
/// assert_eq!(spinner_cell.symbol(), "⠘");
///
/// spinner.render(area, &mut buf);
/// let spinner_cell = buf.cell(spinner_cell_position).unwrap();
/// assert_eq!(spinner_cell.symbol(), "⠰");
///
/// spinner.render(area, &mut buf);
/// let spinner_cell = buf.cell(spinner_cell_position).unwrap();
/// assert_eq!(spinner_cell.symbol(), "⠤");
///
/// spinner.render(area, &mut buf);
/// let spinner_cell = buf.cell(spinner_cell_position).unwrap();
/// assert_eq!(spinner_cell.symbol(), "⠆");
///
/// spinner.render(area, &mut buf);
/// let spinner_cell = buf.cell(spinner_cell_position).unwrap();
/// assert_eq!(spinner_cell.symbol(), "⠃");
///
/// spinner.render(area, &mut buf);
/// let spinner_cell = buf.cell(spinner_cell_position).unwrap();
/// assert_eq!(spinner_cell.symbol(), "⠉");
///
/// // Iteration starts with the beginning.
/// spinner.render(area, &mut buf);
/// let spinner_cell = buf.cell(spinner_cell_position).unwrap();
/// assert_eq!(spinner_cell.symbol(), "⠘");
/// ```
#[derive(Derivative)]
#[derivative(Debug, PartialEq, Eq)]
pub struct SmallSpinnerWidget {
    #[derivative(Debug = "ignore", PartialEq = "ignore")]
    symbol_cycle: Box<dyn Iterator<Item = &'static str>>,
    style: SmallSpinnerStyle,
}

impl Default for SmallSpinnerWidget {
    fn default() -> Self {
        let style = SmallSpinnerStyle::default();
        Self::new(style)
    }
}

impl Clone for SmallSpinnerWidget {
    fn clone(&self) -> Self {
        Self::new(self.style)
    }
}

impl Widget for &mut SmallSpinnerWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height < 1 || area.width < 1 {
            return;
        }

        let x = if area.width == 1 {
            area.x
        } else {
            match self.style.alignment {
                Alignment::Left => area.x,
                Alignment::Center => area.x + area.width / 2,
                Alignment::Right => area.x + area.width - 1,
            }
        };
        let next_symbol = self.symbol_cycle.next().unwrap();

        buf[(x, area.y)]
            .set_symbol(next_symbol)
            .set_bg(self.style.background_color)
            .set_fg(self.style.foreground_color);
    }
}

impl SmallSpinnerWidget {
    pub fn new(style: SmallSpinnerStyle) -> Self {
        let symbol_cycle = symbol_cycle_by_spinner_type(style.type_);
        let symbol_cycle = Box::new(symbol_cycle);

        Self {
            symbol_cycle,
            style,
        }
    }

    /// Resets the spinner's animation to its initial state.
    ///
    /// Restarts the spinner's symbol sequence according to its current
    /// style, starting from the first symbol.
    pub fn reset(&mut self) {
        let symbol_cycle = symbol_cycle_by_spinner_type(self.style.type_);
        self.symbol_cycle = Box::new(symbol_cycle);
    }
}

#[cfg(test)]
mod tests {
    use ratatui::{
        buffer::Buffer,
        layout::{
            Alignment,
            Position,
            Rect,
        },
        widgets::Widget,
    };

    use super::SmallSpinnerWidget;
    use crate::{
        SmallSpinnerStyleBuilder,
        SmallSpinnerType,
    };

    #[test]
    fn left_aligned_spinner() {
        let spinner_style = SmallSpinnerStyleBuilder::default()
            .with_type(SmallSpinnerType::BrailleDouble)
            .with_alignment(Alignment::Left)
            .build()
            .unwrap();
        let mut spinner = SmallSpinnerWidget::new(spinner_style);

        let area = Rect::new(0, 0, 6, 1);
        let mut buf = Buffer::empty(area);
        let spinner_cell_position = Position::new(0, 0);

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
    }

    #[test]
    fn center_aligned_spinner() {
        let spinner_style = SmallSpinnerStyleBuilder::default()
            .with_type(SmallSpinnerType::BrailleDouble)
            .with_alignment(Alignment::Center)
            .build()
            .unwrap();
        let mut spinner = SmallSpinnerWidget::new(spinner_style);

        let area = Rect::new(0, 0, 7, 1);
        let mut buf = Buffer::empty(area);
        let spinner_cell_position = Position::new(3, 0);

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
    }

    #[test]
    fn right_aligned_spinner() {
        let spinner_style = SmallSpinnerStyleBuilder::default()
            .with_type(SmallSpinnerType::BrailleDouble)
            .with_alignment(Alignment::Right)
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
    }
}
