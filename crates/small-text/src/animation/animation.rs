use std::{
    cmp::Ordering,
    collections::HashMap,
    time::Instant,
};

use ratatui::style::Modifier;

use super::{
    AdvancableAnimation,
    AnimationAction,
    AnimationStep,
    AnimationStyle,
    AnimationTargetedSymbols,
};
use crate::SymbolStyle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SymbolState {
    Styled(SymbolStyle),
    Untouched,
}

impl Into<StepSymbolState> for SymbolState {
    fn into(self) -> StepSymbolState {
        match self {
            Self::Styled(style) => StepSymbolState::Untouched(style.into()),
            Self::Untouched => StepSymbolState::Untouched(None),
        }
    }
}

/// Represents the state of a symbol for the current
/// step.
///
/// # Variants:
///
/// - `Styled(SymbolStyle)`: The symbol was styled in
///    the current step.
///
/// - `Untouched(Option<SymbolStyle>)`: The symbol was
///    not styled in the current step.
///
///    - `None`: The symbol has never been styled in
///       current or any previous steps.
///
///    - `Some(SymbolStyle)`: The symbol was styled in
///       a previous step, and this is its style.
#[derive(Clone, Copy)]
enum StepSymbolState {
    Styled(SymbolStyle),
    Untouched(Option<SymbolStyle>),
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
    /// A map from virtual X coordinates of symbols to
    /// their corresponding styles.
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

        let step = if self.is_paused {
            self.advancable_animation.current_step()
        } else if self.last_step_retrieved_at.is_none() {
            self.last_step_retrieved_at = Some(now);
            self.advancable_animation.current_step()
        } else {
            self.next_step(now)
        };

        if let Some(step) = step {
            self.process_step(step);
            self.build_frame().into()
        } else {
            None
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

    fn process_step(&mut self, step: AnimationStep) {
        let mut step_states: HashMap<u16, StepSymbolState> = self
            .symbol_states
            .clone()
            .into_iter()
            .map(|(x, state)| (x, state.into()))
            .collect();
        let mut actions: Vec<(
            AnimationTargetedSymbols,
            Vec<AnimationAction>,
        )> = step.actions.into_iter().collect();
        actions.sort_by(|a, b| targeted_symbols_sorter(a.0, b.0));

        for (target, actions) in actions {
            let xs = self.xs_by_targeted_symbols(target, &step_states);
            self.execute_actions(xs, &mut step_states, actions);
        }

        self.symbol_states = step_states
            .into_iter()
            .map(|(x, state)| (x, state.into()))
            .collect();
    }

    fn build_frame(&self) -> AnimationFrame {
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

    fn xs_by_targeted_symbols(
        &self,
        target: AnimationTargetedSymbols,
        step_states: &HashMap<u16, StepSymbolState>,
    ) -> Vec<u16> {
        match target {
            AnimationTargetedSymbols::Single(x) => vec![x],
            AnimationTargetedSymbols::Range(start, end) => {
                (start..=end).collect()
            }
            AnimationTargetedSymbols::Untouched => step_states
                .iter()
                .filter(|(_, step)| {
                    matches!(step, StepSymbolState::Untouched(None))
                })
                .map(|(x, _)| x)
                .copied()
                .collect(),
            AnimationTargetedSymbols::UntouchedThisStep => step_states
                .iter()
                .filter(|(_, step)| {
                    matches!(step, StepSymbolState::Untouched(_))
                })
                .map(|(x, _)| x)
                .copied()
                .collect(),
        }
    }

    fn execute_actions(
        &self,
        xs: Vec<u16>,
        step_states: &mut HashMap<u16, StepSymbolState>,
        actions: Vec<AnimationAction>,
    ) {
        for x in xs {
            let step_state = if let Some(state) = step_states.get(&x) {
                let mut style = match state {
                    StepSymbolState::Styled(style) => *style,
                    StepSymbolState::Untouched(Some(style)) => *style,
                    StepSymbolState::Untouched(None) => SymbolStyle::default(),
                };
                for action in actions.iter() {
                    self.execute_action(&mut style, *action);
                }

                StepSymbolState::Styled(style)
            } else {
                let mut style = SymbolStyle::default();
                for action in actions.iter() {
                    self.execute_action(&mut style, *action);
                }

                StepSymbolState::Styled(style)
            };

            step_states.insert(x, step_state);
        }
    }

    fn execute_action(
        &self,
        style: &mut SymbolStyle,
        action: AnimationAction,
    ) {
        match action {
            AnimationAction::UpdateForegroundColor(color) => {
                style.foreground_color = color;
            }
            AnimationAction::UpdateBackgroundColor(color) => {
                style.background_color = color;
            }
            AnimationAction::AddModifier(modifier) => {
                style.modifier |= modifier;
            }
            AnimationAction::RemoveModifier(modifier) => {
                style.modifier.remove(modifier);
            }
            AnimationAction::RemoveAllModifiers => {
                style.modifier = Modifier::empty();
            }
        }
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
