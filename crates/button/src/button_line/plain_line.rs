use ratatui::{
    buffer::Buffer,
    layout::{
        Alignment,
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

use super::ButtonLineStyle;

pub(crate) struct PlainLineStyle<'a> {
    text: &'a str,
    text_color: Color,
    background_color: Color,
    text_modifier: Option<Modifier>,
}

impl<'a> From<ButtonLineStyle<'a>> for PlainLineStyle<'a> {
    fn from(value: ButtonLineStyle<'a>) -> Self {
        Self {
            text: value.text,
            text_color: value.text_color,
            background_color: value.background_color,
            text_modifier: value.text_modifier,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct PlainLine<'a> {
    /// Instead of storing individual parameters for the
    /// line (e.g., 'foreground_color', 'text', etc.), we
    /// store the entire line and clone it when rendering.
    line: Line<'a>,
}

impl<'a> Widget for &PlainLine<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Clear cells used to render the line in case
        // the line was previously rendered with the
        // different text.
        for x in area.x..area.x + area.width as u16 {
            buf[(x, area.y)].reset();
        }
        self.line.clone().render(area, buf);
    }
}

impl<'a> PlainLine<'a> {
    pub fn new(style: impl Into<PlainLineStyle<'a>>) -> Self {
        let style = style.into();

        let mut line = Line::from(style.text)
            .fg(style.text_color)
            .bg(style.background_color)
            .alignment(Alignment::Center);

        line = match style.text_modifier {
            Some(modifier) => line.add_modifier(modifier),
            None => line,
        };

        Self { line }
    }
}
