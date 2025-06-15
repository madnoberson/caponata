use ratatui::{
    buffer::Buffer,
    layout::{
        Position,
        Rect,
    },
    widgets::Widget,
};

use super::{
    ThickButton,
    ThinButton,
};
use crate::ButtonStateStyle;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum StyledButton<'a> {
    Thick(ThickButton<'a>),
    Thin(ThinButton<'a>),
}

impl<'a> Default for StyledButton<'a> {
    fn default() -> Self {
        Self::Thin(ThinButton::default())
    }
}

impl<'a> Widget for &mut StyledButton<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            StyledButton::Thick(button) => button.render(area, buf),
            StyledButton::Thin(button) => button.render(area, buf),
        };
    }
}

impl<'a> StyledButton<'a> {
    pub fn new(style: ButtonStateStyle<'a>) -> Self {
        match style.thickness {
            Some(_) => Self::Thick(ThickButton::new(style)),
            None => Self::Thin(ThinButton::new(style)),
        }
    }

    /// Returns boolean flag indicating whether widget contains
    /// provided position. Widget's area is calculated based on
    /// provided area.
    pub fn contains(&self, area: Rect, position: Position) -> bool {
        match self {
            StyledButton::Thick(button) => button.contains(area, position),
            StyledButton::Thin(button) => button.contains(area, position),
        }
    }

    /// Enables spinner if the button supports spinner; otherwise
    /// does nothing.
    pub fn enable_spinner(&mut self) {
        match self {
            StyledButton::Thick(button) => button.enable_spinner(),
            StyledButton::Thin(button) => button.enable_spinner(),
        }
    }

    /// Disables spinner if the button supports spinner; otherwise
    /// does nothing.
    pub fn disable_spinner(&mut self) {
        match self {
            StyledButton::Thick(button) => button.disable_spinner(),
            StyledButton::Thin(button) => button.disable_spinner(),
        }
    }
}
