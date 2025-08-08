use std::collections::HashMap;

use derive_builder::Builder;

use super::{
    SymbolStyle,
    Target,
};

/// A styling configuration for [`SmallTextWidget`].
///
/// # Example
///
/// ```rust
/// use std::collections::HashMap;
///
/// use ratatui_small_text::{
///     Target,
///     SymbolStyle,
///     AnimationStyle,
///     SmallTextStyleBuilder,
/// };
///
/// let symbol_styles = HashMap::from([
///     (Target::Untouched, SymbolStyle::default()),
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
pub struct SmallTextStyle<'a> {
    pub(crate) text: &'a str,
    pub(crate) symbol_styles: HashMap<Target, SymbolStyle>,
}

impl<'a> SmallTextStyle<'a> {
    pub fn new(
        text: &'a str,
        symbol_styles: HashMap<Target, SymbolStyle>,
    ) -> Self {
        Self {
            text,
            symbol_styles,
        }
    }
}
