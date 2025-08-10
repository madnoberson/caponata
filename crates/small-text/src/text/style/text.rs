use std::collections::HashMap;

use ratatui::style::{
    Color,
    Modifier,
};

use super::{
    SymbolStyle,
    Target,
};

/// A styling configuration for [`SmallTextWidget`].
///
/// # Example
///
/// ```rust
/// use ratatui::style::{Color, Modifier};
/// use ratatui_small_text::{
///     Target,
///     SymbolStyleBuilder,
///     SmallTextStyleBuilder,
///     SmallTextWidget,
/// };
///
/// let symbol_style = SymbolStyleBuilder::default()
///     .with_background_color(Color::Gray)
///     .with_foreground_color(Color::Blue)
///     .with_modifier(Modifier::BOLD)
///     .build()
///     .unwrap();
/// let text_style = SmallTextStyleBuilder::default()
///     .with_text("Text example")
///     .for_target(Target::Every(2))
///     .set_background_color(Color::White)
///     .set_foreground_color(Color::Red)
///     .set_modifier(Modifier::UNDERLINED)
///     .then()
///     .for_target(Target::Untouched)
///     .set_style(symbol_style)
///     .then()
///     .build();
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq)]
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

/// A builder for constructing [`SmallTextStyle`].
///
/// # Example
///
/// ```rust
/// use ratatui::style::{Color, Modifier};
/// use ratatui_small_text::{
///     Target,
///     SymbolStyleBuilder,
///     SmallTextStyleBuilder,
///     SmallTextWidget,
/// };
///
/// let symbol_style = SymbolStyleBuilder::default()
///     .with_background_color(Color::Gray)
///     .with_foreground_color(Color::Blue)
///     .with_modifier(Modifier::BOLD)
///     .build()
///     .unwrap();
/// let text_style = SmallTextStyleBuilder::default()
///     .with_text("Text example")
///     .for_target(Target::Every(2))
///     .set_background_color(Color::White)
///     .set_foreground_color(Color::Red)
///     .set_modifier(Modifier::UNDERLINED)
///     .then()
///     .for_target(Target::Untouched)
///     .set_style(symbol_style)
///     .then()
///     .build();
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SmallTextStyleBuilder<'a> {
    text: Option<&'a str>,
    symbol_styles: HashMap<Target, SymbolStyle>,
}

impl<'a> SmallTextStyleBuilder<'a> {
    pub fn with_text(mut self, text: &'a str) -> Self {
        self.text = Some(text);
        self
    }

    pub fn for_target(self, target: Target) -> SymbolStyleAssembler<'a> {
        SymbolStyleAssembler {
            target,
            text_style_builder: self,
            background_color: None,
            foreground_color: None,
            modifier: None,
        }
    }

    pub fn build(self) -> SmallTextStyle<'a> {
        SmallTextStyle {
            text: self.text.unwrap_or_default(),
            symbol_styles: self.symbol_styles,
        }
    }
}

pub struct SymbolStyleAssembler<'a> {
    target: Target,
    text_style_builder: SmallTextStyleBuilder<'a>,
    background_color: Option<Color>,
    foreground_color: Option<Color>,
    modifier: Option<Modifier>,
}

impl<'a> SymbolStyleAssembler<'a> {
    pub fn set_background_color(mut self, color: Color) -> Self {
        self.background_color = Some(color);
        self
    }

    pub fn set_foreground_color(mut self, color: Color) -> Self {
        self.foreground_color = Some(color);
        self
    }

    pub fn set_modifier(mut self, modifier: Modifier) -> Self {
        self.modifier = Some(modifier);
        self
    }

    pub fn set_style(mut self, style: SymbolStyle) -> Self {
        self.background_color = Some(style.background_color);
        self.foreground_color = Some(style.foreground_color);
        self.modifier = Some(style.modifier);
        self
    }

    pub fn then(mut self) -> SmallTextStyleBuilder<'a> {
        let symbol_style = SymbolStyle::new(
            self.foreground_color.unwrap_or_default(),
            self.background_color.unwrap_or_default(),
            self.modifier.unwrap_or_default(),
        );
        self.text_style_builder
            .symbol_styles
            .insert(self.target, symbol_style);

        self.text_style_builder
    }
}
