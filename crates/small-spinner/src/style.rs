use std::time::Duration;

use derive_builder::Builder;
use ratatui::{
    layout::Alignment,
    style::Color,
};
use strum_macros::{
    AsRefStr,
    EnumIter,
};

/// Type of animation for [`SmallSpinnerWidget`].
///
/// Default variant is [`SmallSpinnerType::BrailleDouble`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, AsRefStr, EnumIter)]
#[strum(serialize_all = "snake_case")]
#[non_exhaustive]
pub enum SmallSpinnerType {
    /// ["|", "/", "-", "\\"]
    Ascii,

    /// ["│", "╱", "─", "╲"]
    BoxDrawing,

    /// ["↑", "↗", "→", "↘", "↓", "↙", "←", "↖"]
    Arrow,

    /// ["⇑", "⇗", "⇒", "⇘", "⇓", "⇙", "⇐", "⇖"]
    DoubleArrow,

    /// ["▝", "▗", "▖", "▘"]
    QuadrantBlock,

    /// ["▙", "▛", "▜", "▟"]
    QuadrantBlockCrack,

    /// ["▁", "▂", "▃", "▄", "▅", "▆", "▇", "█"]
    VerticalBlock,

    /// ["▏", "▎", "▍", "▌", "▋", "▊", "▉", "█"]
    HorizontalBlock,

    /// ["◢", "◥", "◤", "◣"]
    TriangleCorners,

    /// ["◳", "◲", "◱", "◰"]
    WhiteSquare,

    /// ["◷", "◶", "◵", "◴"]
    WhiteCircle,

    /// ["◑", "◒", "◐", "◓"]
    BlackCircle,

    /// ["🕛", "🕧", "🕐", "🕜", "🕑", "🕝",
    ///  "🕒", "🕞", "🕓", "🕟", "🕔", "🕠",
    ///  "🕕", "🕡", "🕖", "🕢", "🕗", "🕣",
    ///  "🕘", "🕤", "🕙", "🕥", "🕚", "🕦"]
    Clock,

    /// ["🌑", "🌒", "🌓", "🌕", "🌖"]
    MoonPhases,

    /// ["⠈", "⠐", "⠠", "⠄", "⠂", "⠁"]
    BrailleOne,

    /// ["⠘", "⠰", "⠤", "⠆", "⠃", "⠉"]
    #[default]
    BrailleDouble,

    /// ["⠷", "⠯", "⠟", "⠻", "⠽", "⠾"]
    BrailleSix,

    /// ["⠧", "⠏", "⠛", "⠹", "⠼", "⠶"]
    BrailleSixDouble,

    /// ["⣷", "⣯", "⣟", "⡿", "⢿", "⣻", "⣽", "⣾"]
    BrailleEight,

    /// ["⣧", "⣏", "⡟", "⠿", "⢻", "⣹", "⣼", "⣶"]
    BrailleEightDouble,

    /// [" ", "ᚐ", "ᚑ", "ᚒ", "ᚓ", "ᚔ"]
    OghamA,

    /// [" ", "ᚁ", "ᚂ", "ᚃ", "ᚄ", "ᚅ"]
    OghamB,

    /// [" ", "ᚆ", "ᚇ", "ᚈ", "ᚉ", "ᚊ"]
    OghamC,

    /// ["⎛", "⎜", "⎝", "⎞", "⎟", "⎠"]
    Parenthesis,

    /// ["ᔐ", "ᯇ", "ᔑ", "ᯇ"]
    Canadian,
}

/// A styling configuration for [`SmallSpinnerWidget`].
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
