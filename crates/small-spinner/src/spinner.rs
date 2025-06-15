use std::time::Instant;

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
    SymbolCycle,
};

/// Result of checking whether the widget can proceed with
/// rendering its next or current symbol based on the time
/// elapsed since the last update.
enum RenderIntervalCheckResult {
    /// The widget is being rendered for the first time;
    /// no previous render timestamp is available;
    /// the current symbol should be rendered.
    FirstTime,

    /// Enough time has passed since the last symbol was
    /// rendered; the next symbol should now be rendered.
    Ready,

    /// Not enough time has passed since the last symbol
    /// was rendered; the current symbol should be rendered
    /// again.
    TooSoon,

    /// An error occurred while comparing timestamps.
    ComparisonError,
}

/// A widget that displays single-character animated spinner.
///
/// The [`SmallSpinnerWidget`] renders a single-character spinner
/// that cycles through a sequence of symbols based on the provided
/// [`SmallSpinnerStyle`].
///
/// # Example
///
/// ```rust
/// use std::time::Duration;
///
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
///     .with_interval(Duration::from_secs(0))
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
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SmallSpinnerWidget {
    symbol_cycle: SymbolCycle,
    style: SmallSpinnerStyle,

    /// Timestamp of the last rendering of the next
    /// symbol. This field is not updated if the
    /// current symbol being rendered, except the
    /// first symbol in the cycle.
    last_rendered_at: Option<Instant>,
}

impl Widget for &mut SmallSpinnerWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height < 1 || area.width < 1 {
            return;
        }

        let now = Instant::now();
        let interval = self.style.interval;

        let interval_check_result = match self.last_rendered_at {
            Some(timestamp) => match timestamp.checked_add(interval) {
                Some(min_timestamp) => {
                    if now >= min_timestamp {
                        RenderIntervalCheckResult::Ready
                    } else {
                        RenderIntervalCheckResult::TooSoon
                    }
                }
                None => RenderIntervalCheckResult::ComparisonError,
            },
            None => RenderIntervalCheckResult::FirstTime,
        };
        let symbol_to_render = match interval_check_result {
            RenderIntervalCheckResult::Ready => {
                self.last_rendered_at = Some(now);
                self.symbol_cycle.next_symbol()
            }
            RenderIntervalCheckResult::FirstTime => {
                self.last_rendered_at = Some(now);
                self.symbol_cycle.current_symbol()
            }
            RenderIntervalCheckResult::TooSoon => {
                self.symbol_cycle.current_symbol()
            }
            RenderIntervalCheckResult::ComparisonError => {
                self.symbol_cycle.current_symbol()
            }
        };

        let x = if area.width == 1 {
            area.x
        } else {
            match self.style.alignment {
                Alignment::Left => area.x,
                Alignment::Center => area.x + area.width / 2,
                Alignment::Right => area.x + area.width - 1,
            }
        };
        buf[(x, area.y)]
            .set_symbol(symbol_to_render)
            .set_bg(self.style.background_color)
            .set_fg(self.style.foreground_color);
    }
}

impl SmallSpinnerWidget {
    pub fn new(style: SmallSpinnerStyle) -> Self {
        Self {
            symbol_cycle: SymbolCycle::new(style.type_),
            style,
            last_rendered_at: None,
        }
    }

    /// Resets the spinner's animation to its initial state.
    ///
    /// Restarts the spinner's symbol sequence according to its
    /// current style, starting from the first symbol.
    pub fn reset(&mut self) {
        self.symbol_cycle.reset();
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

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
            .with_interval(Duration::from_secs(0))
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
            .with_interval(Duration::from_secs(0))
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
            .with_interval(Duration::from_secs(0))
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
