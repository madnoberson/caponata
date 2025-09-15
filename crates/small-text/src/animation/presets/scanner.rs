use std::{
    collections::HashMap,
    sync::Arc,
    time::Duration,
};

use caponata_common::Callable;
use derive_builder::Builder;
use ratatui::style::Color;

use crate::{
    AnimationAdvanceMode,
    AnimationRepeatMode,
    AnimationStep,
    AnimationStepBuilder,
    AnimationStyle,
    AnimationStyleBuilder,
    SmallTextStyle,
    StepSymbolState,
    Symbol,
    SymbolStyleBuilder,
    create_symbols,
};

#[derive(Debug, Clone, PartialEq, Eq, Builder)]
#[builder(setter(prefix = "with", into, strip_option))]
pub struct ScannerAnimationStyle<'a> {
    text_style: &'a SmallTextStyle<'a>,

    #[builder(default)]
    duration: Duration,

    #[builder(default)]
    foreground_color: Option<Color>,

    #[builder(default)]
    background_color: Option<Color>,

    #[builder(default)]
    advance_mode: AnimationAdvanceMode,

    #[builder(default)]
    repeat_mode: AnimationRepeatMode,
}

impl<'a> Into<AnimationStyle> for ScannerAnimationStyle<'a> {
    fn into(self) -> AnimationStyle {
        let mut steps: Vec<AnimationStep> = Vec::new();

        let foreground_color = self.foreground_color;
        let background_color = self.background_color;

        let text_symbols = create_symbols(
            self.text_style.text,
            self.text_style.symbol_styles.clone(),
        );
        let text_char_count = self.text_style.text.chars().count() as u16;

        for x in 0..text_char_count {
            let symbols = text_symbols.clone();

            let on_before_finish =
                move |(step_states,): (HashMap<u16, StepSymbolState>,)| {
                    if step_states.is_empty() {
                        return HashMap::new();
                    }
                    let mut updated_symbols = HashMap::new();

                    let current_symbol = if let Some(symbol) = symbols.get(&x)
                    {
                        symbol
                    } else {
                        return HashMap::new();
                    };

                    let scanned_symbol_foreground_color = foreground_color
                        .unwrap_or(current_symbol.foreground_color);
                    let scanned_symbol_background_color = background_color
                        .unwrap_or(current_symbol.background_color);
                    let scanned_symbol_style = SymbolStyleBuilder::default()
                        .with_foreground_color(scanned_symbol_foreground_color)
                        .with_background_color(scanned_symbol_background_color)
                        .with_modifier(current_symbol.modifier)
                        .build()
                        .unwrap();

                    let scanned_symbol = Symbol::new(
                        current_symbol.value,
                        scanned_symbol_style,
                    );
                    updated_symbols.insert(x, scanned_symbol);

                    if x == 0 {
                        return updated_symbols;
                    }

                    let old_scanned_symbol_x = x.saturating_sub(1);

                    let old_scanned_symbol = if let Some(symbol) =
                        symbols.get(&old_scanned_symbol_x)
                    {
                        symbol
                    } else {
                        return HashMap::new();
                    };
                    updated_symbols
                        .insert(old_scanned_symbol_x, *old_scanned_symbol);

                    updated_symbols
                };

            let on_before_finish = Arc::new(on_before_finish);
            let on_before_finish = Callable::new(on_before_finish);

            let step = AnimationStepBuilder::default()
                .with_duration(self.duration)
                .with_before_finish_callback(on_before_finish)
                .build();
            steps.push(step);
        }

        for x in (1..text_char_count.saturating_sub(1)).rev() {
            let symbols = text_symbols.clone();

            let on_before_finish =
                move |(step_states,): (HashMap<u16, StepSymbolState>,)| {
                    if step_states.is_empty() {
                        return HashMap::new();
                    }
                    let mut updated_symbols = HashMap::new();

                    let current_symbol = if let Some(symbol) = symbols.get(&x)
                    {
                        symbol
                    } else {
                        return HashMap::new();
                    };

                    let scanned_symbol_foreground_color = foreground_color
                        .unwrap_or(current_symbol.foreground_color);
                    let scanned_symbol_background_color = background_color
                        .unwrap_or(current_symbol.background_color);
                    let scanned_symbol_style = SymbolStyleBuilder::default()
                        .with_foreground_color(scanned_symbol_foreground_color)
                        .with_background_color(scanned_symbol_background_color)
                        .with_modifier(current_symbol.modifier)
                        .build()
                        .unwrap();

                    let scanned_symbol = Symbol::new(
                        current_symbol.value,
                        scanned_symbol_style,
                    );
                    updated_symbols.insert(x, scanned_symbol);

                    if x == text_char_count.saturating_sub(1) {
                        return updated_symbols;
                    }

                    let old_scanned_symbol_x = x.saturating_add(1);

                    let old_scanned_symbol = if let Some(symbol) =
                        symbols.get(&old_scanned_symbol_x)
                    {
                        symbol
                    } else {
                        return HashMap::new();
                    };
                    updated_symbols
                        .insert(old_scanned_symbol_x, *old_scanned_symbol);

                    updated_symbols
                };

            let on_before_finish = Arc::new(on_before_finish);
            let on_before_finish = Callable::new(on_before_finish);

            let step = AnimationStepBuilder::default()
                .with_duration(self.duration)
                .with_before_finish_callback(on_before_finish)
                .build();
            steps.push(step);
        }

        AnimationStyleBuilder::default()
            .with_advance_mode(self.advance_mode)
            .with_repeat_mode(self.repeat_mode)
            .with_steps(steps)
            .build()
            .unwrap()
    }
}
