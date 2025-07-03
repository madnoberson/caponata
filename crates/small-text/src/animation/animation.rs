use std::{
    cmp::Ordering,
    collections::HashMap,
    time::Instant,
};

use super::{
    AdvancableAnimation,
    AnimationStep,
    AnimationStyle,
};
use crate::{
    AnimationTargetedSymbols,
    SymbolStyle,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SymbolState {
    Styled(SymbolStyle),
    Untouched,
}

#[derive(Clone, Copy)]
enum StepSymbolState {
    Styled(SymbolStyle),
    Untouched(Option<SymbolStyle>),
}

impl From<SymbolState> for StepSymbolState {
    fn from(value: SymbolState) -> Self {
        match value {
            SymbolState::Styled(style) => {
                StepSymbolState::Untouched(style.into())
            }
            SymbolState::Untouched => StepSymbolState::Untouched(None),
        }
    }
}

impl Into<SymbolState> for StepSymbolState {
    fn into(self) -> SymbolState {
        match self {
            Self::Styled(style) => SymbolState::Styled(style),
            Self::Untouched(Some(style)) => SymbolState::Styled(style),
            Self::Untouched(None) => SymbolState::Untouched,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AnimationFrame {
    pub(crate) symbol_styles: HashMap<u16, SymbolStyle>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Animation {
    advancable_animation: AdvancableAnimation,

    /// A map from virtual X coordinates of symbols to
    /// their corresponding styles.
    symbol_states: HashMap<u16, SymbolState>,

    is_paused: bool,

    /// A timestamp of the last retrieving of the unqiue
    /// step. This field is not updated if the current
    /// step is being retrieved, except the first symbol
    /// in the cycle.
    last_step_retrieved_at: Option<Instant>,
}

impl Animation {
    pub fn new(style: AnimationStyle, text_char_count: u16) -> Self {
        let advancable_animation = AdvancableAnimation::new(
            style.steps,
            style.repeat_mode,
            style.advance_mode,
        );
        let symbol_states: HashMap<u16, SymbolState> = (0..text_char_count)
            .map(|x| (x, SymbolState::Untouched))
            .collect();

        Self {
            advancable_animation,
            symbol_states,
            is_paused: false,
            last_step_retrieved_at: None,
        }
    }

    pub fn next_frame(&mut self) -> Option<AnimationFrame> {
        let now = Instant::now();

        let step = if self.last_step_retrieved_at.is_none() {
            self.last_step_retrieved_at = Some(now);
            self.advancable_animation.current_step()
        } else if self.is_paused {
            self.advancable_animation.current_step()
        } else {
            self.next_step(now)
        };

        match step {
            Some(step) => self.step_to_frame(step).into(),
            None => None,
        }
    }

    pub fn pause(&mut self) {
        self.is_paused = true;
    }

    pub fn unpause(&mut self) {
        self.is_paused = false;
    }

    pub fn advance(&mut self) {
        self.advancable_animation.advance();
    }

    fn next_step(&mut self, now: Instant) -> Option<AnimationStep> {
        let last_step_retrieved_at = self.last_step_retrieved_at?;
        let current_step = self.advancable_animation.current_step()?;

        let enough_time_passed = now.duration_since(last_step_retrieved_at)
            >= current_step.duration;
        let next_step = if enough_time_passed {
            self.advancable_animation.next_step()
        } else {
            return current_step.into();
        };

        if next_step.is_some() {
            self.last_step_retrieved_at = Some(now);
            next_step
        } else {
            current_step.into()
        }
    }

    fn step_to_frame(&mut self, step: AnimationStep) -> AnimationFrame {
        let mut symbol_styles: Vec<(AnimationTargetedSymbols, SymbolStyle)> =
            step.symbol_styles.into_iter().collect();
        symbol_styles.sort_by(|a, b| targeted_symbols_sorter(a.0, b.0));

        self.apply_styles(symbol_styles);

        let symbol_styles: HashMap<u16, SymbolStyle> = self
            .symbol_states
            .iter()
            .filter_map(|(&x, state)| match state {
                SymbolState::Styled(style) => (x, *style).into(),
                SymbolState::Untouched => None,
            })
            .collect();

        AnimationFrame { symbol_styles }
    }

    fn apply_styles(
        &mut self,
        symbol_styles: Vec<(AnimationTargetedSymbols, SymbolStyle)>,
    ) {
        let mut step_states: HashMap<u16, StepSymbolState> = self
            .symbol_states
            .clone()
            .into_iter()
            .map(|(x, state)| (x, state.into()))
            .collect();

        for (target, style) in symbol_styles {
            let step_state = StepSymbolState::Styled(style);

            match target {
                AnimationTargetedSymbols::Single(x) => {
                    step_states.insert(x, step_state);
                }
                AnimationTargetedSymbols::Range(start, end) => {
                    for x in start..=end {
                        step_states.insert(x, step_state);
                    }
                }
                AnimationTargetedSymbols::Untouched => {
                    let untouched_state_xs: Vec<u16> = step_states
                        .iter()
                        .filter(|(_, step)| {
                            matches!(step, StepSymbolState::Untouched(None))
                        })
                        .map(|(x, _)| x)
                        .copied()
                        .collect();

                    for x in untouched_state_xs {
                        step_states.insert(x, step_state);
                    }
                }
                AnimationTargetedSymbols::UntouchedThisStep => {
                    let untouched_state_xs: Vec<u16> = step_states
                        .iter()
                        .filter(|(_, step)| {
                            matches!(step, StepSymbolState::Untouched(_))
                        })
                        .map(|(x, _)| x)
                        .copied()
                        .collect();

                    for x in untouched_state_xs {
                        step_states.insert(x, step_state);
                    }
                }
            }
        }

        self.symbol_states = step_states
            .into_iter()
            .map(|(x, state)| (x, state.into()))
            .collect();
    }
}

fn targeted_symbols_sorter(
    a: AnimationTargetedSymbols,
    b: AnimationTargetedSymbols,
) -> Ordering {
    let priority = |item: &AnimationTargetedSymbols| match item {
        AnimationTargetedSymbols::Single(_) => 3,
        AnimationTargetedSymbols::Range(_, _) => 2,
        AnimationTargetedSymbols::Untouched => 1,
        AnimationTargetedSymbols::UntouchedThisStep => 0,
    };
    priority(&a).cmp(&priority(&b))
}
