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
use ratatui_small_spinner::{
    SmallSpinnerStyle,
    SmallSpinnerWidget,
};

use super::ButtonLineStyle;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LoadingLineStyle<'a> {
    text: &'a str,
    text_color: Color,
    background_color: Color,
    spinner_style: SmallSpinnerStyle,
    text_modifier: Option<Modifier>,
}

impl<'a> From<ButtonLineStyle<'a>> for LoadingLineStyle<'a> {
    fn from(value: ButtonLineStyle<'a>) -> Self {
        Self {
            text: value.text,
            text_color: value.text_color,
            background_color: value.background_color,
            spinner_style: value.spinner_style.unwrap(),
            text_modifier: value.text_modifier,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct LoadingLine<'a> {
    spinner: SmallSpinnerWidget,
    style: LoadingLineStyle<'a>,
    is_spinner_enabled: bool,
}

impl<'a> Widget for &mut LoadingLine<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = area.intersection(buf.area);
        if area.is_empty() {
            return;
        }

        // Clear cells used to render the line in case
        // the line was previously rendered with the
        // different text.
        for x in area.x..area.x + area.width as u16 {
            buf[(x, area.y)].reset();
        }

        let line_text = if self.is_spinner_enabled {
            &format!("  {}", self.style.text)
        } else {
            self.style.text
        };
        let mut line = Line::from(line_text)
            .fg(self.style.text_color)
            .bg(self.style.background_color)
            .alignment(Alignment::Center);

        line = match self.style.text_modifier {
            Some(modifier) => line.add_modifier(modifier),
            None => line,
        };
        let line_width = line.width();

        line.render(area, buf);

        if self.is_spinner_enabled {
            self.render_spinner(area, buf, line_width);
        };
    }
}

impl<'a> LoadingLine<'a> {
    pub fn new(style: impl Into<LoadingLineStyle<'a>>) -> Self {
        let style = style.into();
        let spinner = SmallSpinnerWidget::new(style.spinner_style);

        Self {
            spinner,
            style,
            is_spinner_enabled: false,
        }
    }

    pub fn enable_spinner(&mut self) {
        self.is_spinner_enabled = true;
    }

    pub fn disable_spinner(&mut self) {
        self.is_spinner_enabled = false;
    }

    fn render_spinner(
        &mut self,
        widget_area: Rect,
        buf: &mut Buffer,
        line_width: usize,
    ) {
        let enough_space_for_complete_line =
            line_width <= widget_area.width as usize;

        let spinner_area_x = if enough_space_for_complete_line {
            widget_area
                .width
                .saturating_sub(line_width as u16)
                .div_euclid(2)
                .saturating_add(widget_area.x)
        } else {
            widget_area.x
        };

        let spinner_area = Rect::new(spinner_area_x, widget_area.y, 1, 1);
        self.spinner.render(spinner_area, buf);
    }
}
