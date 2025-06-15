use crossterm::event::{
    Event,
    MouseButton,
    MouseEventKind,
};
use ratatui::{
    buffer::Buffer,
    layout::{
        Position,
        Rect,
    },
    widgets::Widget,
};

use super::{
    ButtonEvent,
    ButtonStatus,
    ButtonStyle,
    StyledButton,
};

/// A widget that displays button that can update its state
/// by being pressed, hovered or disabled.
///
/// # Important
///
/// If the provided area's height is greater than 3, the button
/// will be rendered on the second line of the area; otherwise,
/// it will be rendered on the first line.
///
/// # Example
///
/// ```rust
/// use ratatui::{
///     buffer::Buffer,
///     style::{Color, Modifier, Style},
///     layout::{Alignment, Rect},
///     widgets::Widget,
/// };
/// use ratatui_button::{
///     ButtonThickness,
///     ButtonStateStyleBuilder,
///     ButtonStyleBuilder,
///     ButtonWidget,
/// };
/// use ratatui_small_spinner::{
///     SmallSpinnerType,
///     SmallSpinnerStyleBuilder,
/// };
///
/// let spinner_style = SmallSpinnerStyleBuilder::default()
///     .with_type(SmallSpinnerType::BrailleDouble)
///     .with_foreground_color(Color::White)
///     .with_background_color(Color::Red)
///     .build()
///     .unwrap();
/// let normal_button_style = ButtonStateStyleBuilder::default()
///     .with_text("Normal state")
///     .with_text_color(Color::White)
///     .with_background_color(Color::Red)
///     .build()
///     .unwrap();
/// let pressed_button_style = ButtonStateStyleBuilder::default()
///     .with_text("Pressed")
///     .with_text_color(Color::White)
///     .with_background_color(Color::Red)
///     .with_text_modifier(Modifier::BOLD)
///     .with_spinner_style(spinner_style)
///     .with_thickness(ButtonThickness::OneEightBlock)
///     .build()
///     .unwrap();
/// let button_style = ButtonStyleBuilder::default()
///     .with_normal_style(normal_button_style)
///     .with_pressed_style(pressed_button_style)
///     .build()
///     .unwrap();
///
/// let mut button = ButtonWidget::new(button_style);
///
/// let area = Rect::new(0, 0, 16, 3);
/// let mut buf = Buffer::empty(area);
///
/// button.render(area, &mut buf);
///
/// let mut expected_buf = Buffer::with_lines(vec![
///     "                ",
///     "  Normal state  ",
///     "                ",
/// ]);
/// expected_buf.set_style(
///     Rect::new(0, 1, 16, 1),
///     Style::default().bg(Color::Red).fg(Color::White),
/// );
/// expected_buf.set_style(
///     Rect::new(0, 2, 16, 1),
///     Style::default().bg(Color::Reset).fg(Color::Reset),
/// );
/// assert_eq!(buf, expected_buf);
///
/// button.press();
/// button.enable_spinner();
/// button.render(area, &mut buf);
///
/// let mut expected_buf = Buffer::with_lines(vec![
///     "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
///     "   ⠘ Pressed    ",
///     "▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔",
/// ]);
/// expected_buf.set_style(
///     Rect::new(0, 0, 16, 1),
///     Style::default().bg(Color::Reset).fg(Color::Red),
/// );
/// expected_buf.set_style(
///     Rect::new(0, 1, 16, 1),
///     Style::default()
///         .bg(Color::Red)
///         .fg(Color::White)
///         .add_modifier(Modifier::BOLD),
/// );
/// expected_buf.set_style(
///     Rect::new(0, 2, 16, 1),
///     Style::default().bg(Color::Reset).fg(Color::Red),
/// );
/// assert_eq!(buf, expected_buf);
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ButtonWidget<'a> {
    normal_button: StyledButton<'a>,
    hovered_button: StyledButton<'a>,
    pressed_button: StyledButton<'a>,
    disabled_button: StyledButton<'a>,
    status: ButtonStatus,
}

impl<'a> Widget for &mut ButtonWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.status {
            ButtonStatus::Normal => self.normal_button.render(area, buf),
            ButtonStatus::Hovered => self.hovered_button.render(area, buf),
            ButtonStatus::Pressed => self.pressed_button.render(area, buf),
            ButtonStatus::Disabled => self.disabled_button.render(area, buf),
        }
    }
}

impl<'a> ButtonWidget<'a> {
    pub fn new(style: ButtonStyle<'a>) -> Self {
        Self {
            normal_button: StyledButton::new(style.normal_style),
            hovered_button: StyledButton::new(style.hovered_style),
            pressed_button: StyledButton::new(style.pressed_style),
            disabled_button: StyledButton::new(style.disabled_style),
            status: ButtonStatus::Normal,
        }
    }

    pub fn status(&self) -> ButtonStatus {
        self.status
    }

    fn contains(&self, area: Rect, position: Position) -> bool {
        match self.status {
            ButtonStatus::Normal => {
                self.normal_button.contains(area, position)
            }
            ButtonStatus::Hovered => {
                self.hovered_button.contains(area, position)
            }
            ButtonStatus::Pressed => {
                self.pressed_button.contains(area, position)
            }
            ButtonStatus::Disabled => {
                self.disabled_button.contains(area, position)
            }
        }
    }

    /// Sets the button status to [`ButtonStatus::Pressed`] if it
    /// is not currently disabled. Does nothing if the button is
    /// disabled.
    pub fn press(&mut self) {
        if self.status != ButtonStatus::Disabled {
            self.status = ButtonStatus::Pressed;
        }
    }

    /// Resets the button status to [`ButtonStatus::Normal`] if it
    /// is currently pressed. Does nothing if the button is not
    /// pressed.
    pub fn unpress(&mut self) {
        if self.status == ButtonStatus::Pressed {
            self.status = ButtonStatus::Normal;
        }
    }

    /// Sets the button status to [`ButtonStatus::Disabled`] if it
    /// is not currently disabled. Does nothing if the button is
    /// disabled.
    pub fn disable(&mut self) {
        if self.status != ButtonStatus::Disabled {
            self.status = ButtonStatus::Disabled
        }
    }

    /// Sets the button status to [`ButtonStatus::Normal`] if it
    /// is currently disabled. Does nothing if the button is
    /// not disabled.
    pub fn enable(&mut self) {
        if self.status == ButtonStatus::Disabled {
            self.status = ButtonStatus::Normal;
        }
    }

    /// Enables spinner if the button supports spinner; otherwise
    /// does nothing. Spinner will be enabled for all the button
    /// states.
    pub fn enable_spinner(&mut self) {
        self.normal_button.enable_spinner();
        self.hovered_button.enable_spinner();
        self.pressed_button.enable_spinner();
        self.disabled_button.enable_spinner();
    }

    /// Disables spinner if the button supports spinner; otherwise
    /// does nothing. Spinner will be disabled for all the button
    /// states.
    pub fn disable_spinner(&mut self) {
        self.normal_button.disable_spinner();
        self.hovered_button.disable_spinner();
        self.pressed_button.disable_spinner();
        self.disabled_button.disable_spinner();
    }

    pub fn on_crossterm_event(
        &mut self,
        event: Event,
        widget_area: Rect,
    ) -> Option<ButtonEvent> {
        if let Event::Mouse(mouse_event) = event {
            let mouse_position = Position {
                x: mouse_event.column,
                y: mouse_event.row,
            };
            match mouse_event.kind {
                MouseEventKind::Down(mouse_button) => self.on_mouse_down(
                    mouse_position,
                    mouse_button,
                    widget_area,
                ),
                MouseEventKind::Moved => {
                    self.on_mouse_moved(mouse_position, widget_area)
                }
                _ => None,
            }
        } else {
            None
        }
    }

    fn on_mouse_down(
        &self,
        mouse_position: Position,
        mouse_button: MouseButton,
        widget_area: Rect,
    ) -> Option<ButtonEvent> {
        if mouse_button == MouseButton::Left
            && self.status != ButtonStatus::Disabled
            && self.contains(widget_area, mouse_position)
        {
            Some(ButtonEvent::Clicked)
        } else {
            None
        }
    }

    fn on_mouse_moved(
        &mut self,
        mouse_position: Position,
        widget_area: Rect,
    ) -> Option<ButtonEvent> {
        match (self.status, self.contains(widget_area, mouse_position)) {
            (ButtonStatus::Hovered, false) => {
                self.status = ButtonStatus::Normal;
                Some(ButtonEvent::Unhovered)
            }
            (ButtonStatus::Hovered, true) => Some(ButtonEvent::Hovered(true)),
            (ButtonStatus::Normal, true) => {
                self.status = ButtonStatus::Hovered;
                Some(ButtonEvent::Hovered(false))
            }
            (_, true) => Some(ButtonEvent::Hovered(false)),
            (_, false) => None,
        }
    }
}
