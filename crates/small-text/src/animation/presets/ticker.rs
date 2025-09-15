use std::{
    collections::HashMap,
    sync::Arc,
    time::Duration,
};

use caponata_common::Callable;
use derive_builder::Builder;

use crate::{
    AnimationAdvanceMode,
    AnimationRepeatMode,
    AnimationStepBuilder,
    AnimationStyle,
    AnimationStyleBuilder,
    StepSymbolState,
    Symbol,
};

/// Direction of the ticker animation movement.
///
/// Default variant is [`TickerAnimationDirection::Forward`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TickerAnimationDirection {
    #[default]
    Forward,
    Backward,
}

/// A styling configuration for the ticker animation.
///
/// # Example
///
/// ```rust
/// use std::time::Duration;
///
/// use caponata_small_text::{
///     AnimationStyle,
///     AnimationAdvanceMode,
///     AnimationRepeatMode,
///     TickerAnimationDirection,
///     TickerAnimationStyleBuilder,
/// };
///
/// let animation_style: AnimationStyle =
///     TickerAnimationStyleBuilder::default()
///         .with_direction(TickerDirection::Forward)
///         .with_duration(Duration::from_millis(100))
///         .with_advance_mode(AnimationAdvanceMode::Auto)
///         .with_repeat_mode(AnimationRepeatMode::Infinite)
///         .build()
///         .unwrap()
///         .into();
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Builder)]
#[builder(setter(prefix = "with", into))]
pub struct TickerAnimationStyle {
    #[builder(default)]
    direction: TickerAnimationDirection,

    #[builder(default)]
    duration: Duration,

    #[builder(default)]
    advance_mode: AnimationAdvanceMode,

    #[builder(default)]
    repeat_mode: AnimationRepeatMode,
}

impl Into<AnimationStyle> for TickerAnimationStyle {
    fn into(self) -> AnimationStyle {
        let on_before_finish =
            move |(step_states,): (HashMap<u16, StepSymbolState>,)| {
                if step_states.is_empty() {
                    return HashMap::new();
                }

                let mut symbols: Vec<(u16, Symbol)> = step_states
                    .into_iter()
                    .map(|(x, state)| (x, state.symbol()))
                    .collect();
                symbols.sort_by(|a, b| a.0.cmp(&b.0));

                if self.direction == TickerAnimationDirection::Forward {
                    let last_symbol_index = symbols.iter().count() - 1;
                    let last_symbol = symbols.remove(last_symbol_index);
                    symbols.insert(0, last_symbol);
                } else {
                    let first_symbol = symbols.remove(0);
                    symbols.push(first_symbol);
                }

                let mut updated_symbols: HashMap<u16, Symbol> = HashMap::new();
                for (new_x, (_, symbol)) in symbols.iter().enumerate() {
                    updated_symbols.insert(new_x as u16, *symbol);
                }

                updated_symbols
            };

        let on_before_finish = Arc::new(on_before_finish);
        let on_before_finish = Callable::new(on_before_finish);

        let step = AnimationStepBuilder::default()
            .with_duration(self.duration)
            .with_before_finish_callback(on_before_finish)
            .build();

        return AnimationStyleBuilder::default()
            .with_advance_mode(self.advance_mode)
            .with_repeat_mode(self.repeat_mode)
            .with_steps(vec![step])
            .build()
            .unwrap();
    }
}
