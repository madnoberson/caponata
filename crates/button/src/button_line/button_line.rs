use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{
        Color,
        Modifier,
    },
    widgets::Widget,
};
use ratatui_small_spinner::SmallSpinnerStyle;

use super::{
    LoadingLine,
    PlainLine,
};
use crate::{
    ThickButtonStyle,
    ThinButtonStyle,
};

pub(crate) struct ButtonLineStyle<'a> {
    pub text: &'a str,
    pub text_color: Color,
    pub background_color: Color,
    pub text_modifier: Option<Modifier>,
    pub spinner_style: Option<SmallSpinnerStyle>,
}

impl<'a> From<ThickButtonStyle<'a>> for ButtonLineStyle<'a> {
    fn from(value: ThickButtonStyle<'a>) -> Self {
        Self {
            text: value.text,
            text_color: value.text_color,
            background_color: value.background_color,
            text_modifier: value.text_modifier,
            spinner_style: value.spinner_style,
        }
    }
}

impl<'a> From<ThinButtonStyle<'a>> for ButtonLineStyle<'a> {
    fn from(value: ThinButtonStyle<'a>) -> Self {
        Self {
            text: value.text,
            text_color: value.text_color,
            background_color: value.background_color,
            text_modifier: value.text_modifier,
            spinner_style: value.spinner_style,
        }
    }
}

/// A single-line button content abstraction that may
/// include a loading spinner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ButtonLine<'a> {
    Plain(PlainLine<'a>),
    Loading(LoadingLine<'a>),
}

impl<'a> Default for ButtonLine<'a> {
    fn default() -> Self {
        Self::Plain(PlainLine::default())
    }
}

impl<'a> Widget for &mut ButtonLine<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            ButtonLine::Plain(line) => line.render(area, buf),
            ButtonLine::Loading(line) => line.render(area, buf),
        };
    }
}

impl<'a> ButtonLine<'a> {
    pub fn new(style: impl Into<ButtonLineStyle<'a>>) -> Self {
        let style = style.into();

        match style.spinner_style {
            Some(_) => ButtonLine::Loading(LoadingLine::new(style)),
            None => ButtonLine::Plain(PlainLine::new(style)),
        }
    }

    /// Enables spinner if the line supports spinner; otherwise
    /// does nothing.
    pub fn enable_spinner(&mut self) {
        if let ButtonLine::Loading(line) = self {
            line.enable_spinner();
        }
    }

    /// Disables spinner if the line supports spinner; otherwise
    /// does nothing.
    pub fn disable_spinner(&mut self) {
        if let ButtonLine::Loading(line) = self {
            line.disable_spinner();
        }
    }
}
