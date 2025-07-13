use std::{
    collections::HashMap,
    hash::Hash,
};

use derive_builder::Builder;

use super::{
    SymbolStyle,
    Target,
};
use crate::AnimationStyle;

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
pub struct SmallTextStyle<'a, K>
where
    K: PartialEq + Eq + Hash,
{
    #[builder(default = "\"\"")]
    pub(crate) text: &'a str,

    #[builder(default)]
    pub(crate) symbol_styles: HashMap<Target, SymbolStyle>,

    #[builder(default)]
    pub(crate) animation_styles: HashMap<K, AnimationStyle>,
}
