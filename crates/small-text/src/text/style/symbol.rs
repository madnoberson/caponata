use std::hash::Hash;

use derive_builder::Builder;
use ratatui::style::{
    Color,
    Modifier,
};

/// A styling configuration for a single symbol.
///
/// # Example
///
/// ```rust
/// use ratatui::style::{Color, Modifier};
/// use ratatui_small_text::SymbolStyleBuilder;
///
/// let symbol_style = SymbolStyleBuilder::default()
///     .with_foreground_color(Color::White)
///     .with_background_color(Color::Red)
///     .with_modifier(Modifier::BOLD)
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Builder)]
#[builder(setter(prefix = "with", into))]
pub struct SymbolStyle {
    #[builder(default)]
    pub(crate) foreground_color: Color,

    #[builder(default)]
    pub(crate) background_color: Color,

    #[builder(default)]
    pub(crate) modifier: Modifier,
}

impl SymbolStyle {
    pub fn new(
        foreground_color: Color,
        background_color: Color,
        modifier: Modifier,
    ) -> Self {
        Self {
            foreground_color,
            background_color,
            modifier,
        }
    }
}
