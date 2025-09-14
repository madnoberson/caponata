use std::{
    collections::HashMap,
    rc::Rc,
    time::Duration,
};

use caponata_common::Callable;
use derive_builder::Builder;
use ratatui::style::{
    Color,
    Modifier,
};

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
pub struct WaveAnimationStyle<'a> {
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

impl<'a> Into<AnimationStyle> for WaveAnimationStyle<'a> {
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

                    let symbol_at_head_position =
                        if let Some(symbol) = symbols.get(&x) {
                            symbol
                        } else {
                            return HashMap::new();
                        };

                    let head_symbol_foreground_color = foreground_color
                        .unwrap_or(symbol_at_head_position.foreground_color);
                    let head_symbol_background_color = background_color
                        .unwrap_or(symbol_at_head_position.background_color);
                    let head_symbol_style = SymbolStyleBuilder::default()
                        .with_foreground_color(head_symbol_foreground_color)
                        .with_background_color(head_symbol_background_color)
                        .with_modifier(symbol_at_head_position.modifier)
                        .build()
                        .unwrap();

                    let head_symbol = Symbol::new(
                        symbol_at_head_position.value,
                        head_symbol_style,
                    );
                    updated_symbols.insert(x, head_symbol);

                    let (old_head_symbol_x, old_tail_symbol_x) = if x == 0 {
                        (
                            text_char_count.saturating_sub(1),
                            text_char_count.saturating_sub(2),
                        )
                    } else {
                        (x - 1, x.saturating_sub(2))
                    };
                    let old_head_symbol = if let Some(symbol) =
                        symbols.get(&old_head_symbol_x)
                    {
                        symbol
                    } else {
                        return HashMap::new();
                    };
                    updated_symbols
                        .insert(old_head_symbol_x, *old_head_symbol);

                    let old_tail_symbol = if let Some(symbol) =
                        symbols.get(&old_tail_symbol_x)
                    {
                        symbol
                    } else {
                        return HashMap::new();
                    };
                    updated_symbols
                        .insert(old_tail_symbol_x, *old_tail_symbol);

                    if x < 2 {
                        return updated_symbols;
                    }

                    let symbol_at_tail_position =
                        if let Some(symbol) = symbols.get(&(x - 1)) {
                            symbol
                        } else {
                            return HashMap::new();
                        };

                    let tail_symbol_foreground_color = foreground_color
                        .unwrap_or(symbol_at_tail_position.foreground_color);
                    let tail_symbol_background_color = background_color
                        .unwrap_or(symbol_at_tail_position.background_color);
                    let tail_symbol_modifier =
                        symbol_at_tail_position.modifier.union(Modifier::DIM);
                    let tail_symbol_style = SymbolStyleBuilder::default()
                        .with_foreground_color(tail_symbol_foreground_color)
                        .with_background_color(tail_symbol_background_color)
                        .with_modifier(tail_symbol_modifier)
                        .build()
                        .unwrap();

                    let tail_symbol = Symbol::new(
                        symbol_at_tail_position.value,
                        tail_symbol_style,
                    );
                    updated_symbols.insert(x - 1, tail_symbol);

                    updated_symbols
                };
            let on_before_finish = Callable::new(Rc::new(on_before_finish));

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
