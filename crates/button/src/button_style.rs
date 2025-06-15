use derive_builder::Builder;
use ratatui::style::{
    Color,
    Modifier,
};
use ratatui_small_spinner::SmallSpinnerStyle;

use super::ButtonThickness;

/// Styling configuration for a [`ButtonWidget`].
///
/// # Example
///
/// ```rust
/// use ratatui::layout::Alignment;
/// use ratatui_button::{ButtonStateStyle, ButtonStyleBuilder};
///
/// let button_style = ButtonStyleBuilder::default()
///     .with_normal_style(ButtonStateStyle::default())
///     .with_pressed_style(ButtonStateStyle::default())
///     .with_hovered_style(ButtonStateStyle::default())
///     .with_disabled_style(ButtonStateStyle::default())
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Builder)]
#[builder(setter(prefix = "with", into))]
pub struct ButtonStyle<'a> {
    /// Style applied when used when a [`ButtonWidget`]
    /// is not pressed, hovered or disabled.
    #[builder(default)]
    pub(crate) normal_style: ButtonStateStyle<'a>,

    /// Style applied when a [`ButtonWidget`] is hovered.
    /// This state has lower priority than both
    /// 'pressed' and 'disabled'.
    #[builder(default)]
    pub(crate) hovered_style: ButtonStateStyle<'a>,

    /// Style applied when a [`ButtonWidget`] is pressed.
    /// This state has higher priority than 'hovered',
    /// but lower than 'disabled'.
    #[builder(default)]
    pub(crate) pressed_style: ButtonStateStyle<'a>,

    /// Style applied when a [`ButtonWidget`] is disabled.
    /// This state has the highest priority over both
    /// 'pressed' and 'hovered'.
    #[builder(default)]
    pub(crate) disabled_style: ButtonStateStyle<'a>,
}

/// Styling configuration for a specific state of a [`ButtonWidget`].
///
/// # Example
///
/// ```rust
/// use ratatui::style::{Color, Modifier};
/// use ratatui_small_spinner::SmallSpinnerStyle;
/// use ratatui_button::{ButtonThickness, ButtonStateStyleBuilder};
///
/// let button_state_style = ButtonStateStyleBuilder::default()
///     .with_text("Submit")
///     .with_text_color(Color::White)
///     .with_background_color(Color::Green)
///     .with_text_modifier(Modifier::BOLD)
///     .with_spinner_style(SmallSpinnerStyle::default())
///     .with_thickness(ButtonThickness::OneEightBlock)
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Builder)]
#[builder(setter(prefix = "with", into, strip_option))]
pub struct ButtonStateStyle<'a> {
    #[builder(default = "\"\"")]
    pub(crate) text: &'a str,

    #[builder(default)]
    pub(crate) text_color: Color,

    #[builder(default)]
    pub(crate) background_color: Color,

    #[builder(default)]
    pub(crate) text_modifier: Option<Modifier>,

    #[builder(default)]
    pub(crate) spinner_style: Option<SmallSpinnerStyle>,

    #[builder(default)]
    pub(crate) thickness: Option<ButtonThickness>,
}
