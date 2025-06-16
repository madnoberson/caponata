use ratatui::{
    buffer::Buffer,
    layout::{
        Position,
        Rect,
    },
    style::{
        Color,
        Modifier,
    },
    widgets::Widget,
};
use ratatui_small_spinner::SmallSpinnerStyle;

use crate::{
    ButtonLine,
    ButtonStateStyle,
};

pub(crate) struct ThinButtonStyle<'a> {
    pub text: &'a str,
    pub text_color: Color,
    pub background_color: Color,
    pub text_modifier: Option<Modifier>,
    pub spinner_style: Option<SmallSpinnerStyle>,
}

impl<'a> From<ButtonStateStyle<'a>> for ThinButtonStyle<'a> {
    fn from(value: ButtonStateStyle<'a>) -> Self {
        Self {
            text: value.text,
            text_color: value.text_color,
            background_color: value.background_color,
            text_modifier: value.text_modifier,
            spinner_style: value.spinner_style,
        }
    }
}

/// A minimal button widget rendered using a single
/// horizontal line.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct ThinButton<'a> {
    line: ButtonLine<'a>,
}

impl<'a> Widget for &ThinButton<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height >= 3 {
            let area = Rect::new(area.x, area.y + 1, area.width, area.height);
            self.line.clone().render(area, buf);
        } else {
            self.line.clone().render(area, buf);
        }
    }
}

impl<'a> ThinButton<'a> {
    pub fn new(style: impl Into<ThinButtonStyle<'a>>) -> Self {
        let style = style.into();
        let line = ButtonLine::new(style);

        Self { line }
    }

    /// Returns boolean flag indicating whether widget contains
    /// provided position. Widget's area is calculated based on
    /// provided area.
    pub fn contains(&self, area: Rect, position: Position) -> bool {
        if area.height >= 3 {
            Rect::new(area.x, area.y + 1, area.width, 1).contains(position)
        } else {
            Rect::new(area.x, area.y, area.width, 1).contains(position)
        }
    }

    /// Enables spinner if the button supports spinner; otherwise
    /// does nothing.
    pub fn enable_spinner(&mut self) {
        self.line.enable_spinner();
    }

    /// Disables spinner if the button supports spinner; otherwise
    /// does nothing.
    pub fn disable_spinner(&mut self) {
        self.line.disable_spinner();
    }
}
