use std::iter::repeat;

use ratatui::{
    buffer::Buffer,
    layout::{
        Position,
        Rect,
    },
    style::{
        Color,
        Modifier,
        Stylize,
    },
    text::Line,
    widgets::Widget,
};
use ratatui_small_spinner::SmallSpinnerStyle;

use crate::{
    ButtonLine,
    ButtonStateStyle,
    ButtonThickness,
};

#[derive(Clone, Copy)]
pub(crate) struct ThickButtonStyle<'a> {
    pub text: &'a str,
    pub text_color: Color,
    pub background_color: Color,
    pub thickness: ButtonThickness,
    pub text_modifier: Option<Modifier>,
    pub spinner_style: Option<SmallSpinnerStyle>,
}

impl<'a> From<ButtonStateStyle<'a>> for ThickButtonStyle<'a> {
    fn from(value: ButtonStateStyle<'a>) -> Self {
        Self {
            text: value.text,
            text_color: value.text_color,
            background_color: value.background_color,
            thickness: value.thickness.unwrap(),
            text_modifier: value.text_modifier,
            spinner_style: value.spinner_style,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ThickButton<'a> {
    /// Symbol used to render the top line of
    /// the button. We don't store the line itself,
    /// because we don't know the width of the button,
    /// so we don't know how many of these symbols
    /// we should include in the line.
    top_line_symbol: &'a str,

    middle_line: ButtonLine<'a>,

    /// Symbol used to render the bottom line of
    /// the button. We don't store the line itself,
    /// because we don't know the width of the button,
    /// so we don't know how many of these symbols
    /// we should include in the line.
    bottom_line_symbol: &'a str,

    background_color: Color,
}

impl<'a> Widget for &mut ThickButton<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let top_line_text: String = repeat(self.top_line_symbol)
            .take(area.width as usize)
            .collect();
        let top_line_area = Rect::new(area.x, area.y, area.width, 1);

        Line::from(top_line_text)
            .fg(self.background_color)
            .render(top_line_area, buf);

        let middle_line_area = Rect::new(area.x, area.y + 1, area.width, 1);
        self.middle_line.render(middle_line_area, buf);

        let bottom_line_text: String = repeat(self.bottom_line_symbol)
            .take(area.width as usize)
            .collect();
        let bottom_line_area = Rect::new(area.x, area.y + 2, area.width, 1);

        Line::from(bottom_line_text)
            .fg(self.background_color)
            .render(bottom_line_area, buf);
    }
}

impl<'a> ThickButton<'a> {
    pub fn new(style: impl Into<ThickButtonStyle<'a>>) -> Self {
        let style = style.into();

        let (top_line_symbol, bottom_line_symbol) = match style.thickness {
            ButtonThickness::OneEightBlock => ("▁", "▔"),
            ButtonThickness::HalfBlock => ("▄", "▀"),
        };
        let middle_line = ButtonLine::new(style);

        Self {
            top_line_symbol,
            middle_line,
            bottom_line_symbol,
            background_color: style.background_color,
        }
    }

    /// Returns boolean flag indicating whether widget contains
    /// provided position. Widget's area is calculated based on
    /// provided area.
    pub fn contains(&self, area: Rect, position: Position) -> bool {
        Rect::new(area.x, area.y, area.width, area.height.min(3))
            .contains(position)
    }

    /// Enables spinner if the button supports spinner; otherwise
    /// does nothing.
    pub fn enable_spinner(&mut self) {
        self.middle_line.enable_spinner();
    }

    /// Disables spinner if the button supports spinner; otherwise
    /// does nothing.
    pub fn disable_spinner(&mut self) {
        self.middle_line.disable_spinner();
    }
}
