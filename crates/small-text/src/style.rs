use std::{
    collections::HashMap,
    hash::Hash,
};

use derive_builder::Builder;
use ratatui::style::{
    Color,
    Modifier,
};

use crate::AnimationStyle;

/// A styling configuration for a single symbol.
///
/// # Example
///
/// ```rust
/// use ratatui::style::{Color, Modifier};
/// use ratatui_small_text::SymbolStyleBuilder;
///
/// let symbol_style = SymbolStyleBuilder::default()
///     .with_foreground_color(Color::Reset)
///     .with_background_color(Color::Red)
///     .with_modifier(Modifier::BOLD)
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Builder)]
#[builder(setter(prefix = "with", into, strip_option))]
pub struct SymbolStyle {
    #[builder(default)]
    pub(crate) foreground_color: Option<Color>,

    #[builder(default)]
    pub(crate) background_color: Option<Color>,

    #[builder(default)]
    pub(crate) modifier: Option<Modifier>,
}

/// Represents the selection of symbol positions to
/// which styles should be applied to [`SmallTextWidget`]
/// when animation is disabled.
///
/// Default variant is [`TargetedSymbols::Untouched`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TargetedSymbols {
    /// A specific position of a single symbol. This
    /// is a virtual X coordinate representing the
    /// offset from the beginning of the text.
    Single(u16),

    /// A range of symbol positions (inclusive).
    /// The first value is the start, and the second
    /// is the end of the range. These are virtual
    /// X coordinates representing the offset from
    /// the beginning of the text.
    Range(u16, u16),

    /// Positions of symbols that were not affected
    /// by styling.
    #[default]
    Untouched,
}

/// A styling configuration for [`SmallTextWidget`].
///
/// # Example
///
/// ```rust
/// use std::collections::HashMap;
///
/// use ratatui_small_text::{
///     TargetedSymbols,
///     SymbolStyle,
///     AnimationStyle,
///     SmallTextStyleBuilder,
/// };
///
/// let symbol_styles = HashMap::from([
///     (TargetedSymbols::Untouched, SymbolStyle::default()),
/// ]);
/// let animation_styles = HashMap::from([
///     (1, AnimationStyle::default()),
/// ]);
/// let text_style = SmallTextStyleBuilder::default()
///     .with_text("Text example")
///     .with_symbol_styles(symbol_styles)
///     .with_animation_styles(animation_styles)
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq, Builder)]
#[builder(setter(prefix = "with", into))]
pub struct SmallTextStyle<'a, K>
where
    K: PartialEq + Eq + Hash,
{
    #[builder(default = "\"\"")]
    pub(crate) text: &'a str,

    #[builder(default)]
    pub(crate) symbol_styles: HashMap<TargetedSymbols, SymbolStyle>,

    #[builder(default)]
    pub(crate) animation_styles: HashMap<K, AnimationStyle>,
}
