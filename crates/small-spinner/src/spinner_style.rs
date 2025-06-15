use std::time::Duration;

use derive_builder::Builder;
use ratatui::{
    layout::Alignment,
    style::Color,
};

use super::SmallSpinnerType;

/// Styling configuration for [`SmallSpinnerWidget`].
///
/// # Example
///
/// ```rust
/// use std::time::Duration;
///
/// use ratatui::{
///     style::Color,
///     layout::Alignment,
/// };
/// use ratatui_small_spinner::{
///     SmallSpinnerType,
///     SmallSpinnerStyleBuilder,
/// };
///
/// let style = SmallSpinnerStyleBuilder::default()
///     .with_type(SmallSpinnerType::BrailleDouble)
///     .with_interval(Duration::from_millis(100))
///     .with_alignment(Alignment::Center)
///     .with_foreground_color(Color::White)
///     .with_background_color(Color::Black)
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Builder)]
#[builder(setter(prefix = "with", into))]
pub struct SmallSpinnerStyle {
    #[builder(default, setter(name = "with_type"))]
    pub(crate) type_: SmallSpinnerType,

    #[builder(default)]
    pub(crate) interval: Duration,

    #[builder(default)]
    pub(crate) alignment: Alignment,

    #[builder(default)]
    pub(crate) foreground_color: Color,

    #[builder(default)]
    pub(crate) background_color: Color,
}
