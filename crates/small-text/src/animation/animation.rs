use std::{
    collections::HashMap,
    time::Instant,
};

use ratatui::style::Modifier;

use super::{
    AdvancableAnimation,
    AnimationAction,
    AnimationStep,
    AnimationStyle,
    AnimationTarget,
    animation_targets_sorter,
};
use crate::Symbol;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SymbolState {
    Styled(Symbol),
    Initial(Symbol),
}

impl Into<StepSymbolState> for SymbolState {
    fn into(self) -> StepSymbolState {
        match self {
            Self::Styled(symbol) => StepSymbolState::Untouched(symbol),
            Self::Initial(symbol) => StepSymbolState::Initial(symbol),
        }
    }
}

/// Represents the state of a symbol for the current
/// step.
#[derive(Clone, Copy)]
enum StepSymbolState {
    /// The symbol was styled in the current step.
    Styled(Symbol),

    /// The symbol was not styled in the current or
    /// previous steps.
    Initial(Symbol),

    /// The symbol was not styled in the current step,
    /// but was styled in the previous one.
    Untouched(Symbol),
}

impl Into<SymbolState> for StepSymbolState {
    fn into(self) -> SymbolState {
        match self {
            Self::Styled(symbol) => SymbolState::Styled(symbol),
            Self::Initial(symbol) => SymbolState::Initial(symbol),
            Self::Untouched(symbol) => SymbolState::Styled(symbol),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnimationFrame {
    pub symbols: HashMap<u16, Symbol>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Animation {
    advancable_animation: AdvancableAnimation,
    symbol_states: HashMap<u16, SymbolState>,
    is_paused: bool,
    last_step_retrieved_at: Option<Instant>,
}

impl Animation {
    pub fn new(style: AnimationStyle, symbols: HashMap<u16, Symbol>) -> Self {
        let advancable_animation = AdvancableAnimation::new(
            style.steps,
            style.repeat_mode,
            style.advance_mode,
        );
        let symbol_states: HashMap<u16, SymbolState> = symbols
            .iter()
            .map(|(x, symbol)| (*x, SymbolState::Initial(*symbol)))
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

        let mut actions: Vec<(AnimationTarget, Vec<AnimationAction>)> =
            step.actions.into_iter().collect();
        actions.sort_by(|a, b| animation_targets_sorter(a.0, b.0));

        for (target, actions) in actions {
            let x_coords = self.calculate_x_coords(target, &step_states);
            self.execute_actions(x_coords, &mut step_states, actions);
        }

        self.symbol_states = step_states
            .into_iter()
            .map(|(x, state)| (x, state.into()))
            .collect();
    }

    fn build_frame(&self) -> AnimationFrame {
        let symbols: HashMap<u16, Symbol> = self
            .symbol_states
            .iter()
            .filter_map(|(&x, state)| match state {
                SymbolState::Styled(symbol) => (x, *symbol).into(),
                SymbolState::Initial(symbol) => (x, *symbol).into(),
            })
            .collect();

        AnimationFrame { symbols }
    }

    fn calculate_x_coords(
        &self,
        target: AnimationTarget,
        step_states: &HashMap<u16, StepSymbolState>,
    ) -> Vec<u16> {
        let mut step_states: Vec<(u16, StepSymbolState)> = step_states
            .clone()
            .iter()
            .map(|(x, state)| (*x, *state))
            .collect();
        step_states.sort_by(|a, b| a.0.cmp(&b.0));

        match target {
            AnimationTarget::Single(x) => vec![x],
            AnimationTarget::Range(start, end) => (start..=end).collect(),
            AnimationTarget::Every(n) => step_states
                .iter()
                .map(|(x, _)| *x)
                .step_by(n as usize)
                .collect(),
            AnimationTarget::AllExceptEvery(n) => {
                step_states
                    .iter()
                    .enumerate()
                    .filter_map(|(i, (x, _))| {
                        if i as u16 % n != 0 { (*x).into() } else { None }
                    })
                    .collect()
            }
            AnimationTarget::Untouched => step_states
                .iter()
                .filter(|(_, step)| {
                    matches!(step, StepSymbolState::Initial(_))
                })
                .map(|(x, _)| x)
                .copied()
                .collect(),
            AnimationTarget::UntouchedThisStep => step_states
                .iter()
                .filter(|(_, step)| {
                    matches!(
                        step,
                        StepSymbolState::Untouched(_)
                            | StepSymbolState::Initial(_)
                    )
                })
                .map(|(x, _)| x)
                .copied()
                .collect(),
        }
    }

    fn execute_actions(
        &self,
        x_coords: Vec<u16>,
        step_states: &mut HashMap<u16, StepSymbolState>,
        actions: Vec<AnimationAction>,
    ) {
        for x in x_coords {
            let step_state = step_states.get_mut(&x).unwrap();

            let mut symbol = match step_state {
                StepSymbolState::Styled(symbol) => symbol,
                StepSymbolState::Untouched(symbol) => symbol,
                StepSymbolState::Initial(symbol) => symbol,
            };
            for action in actions.iter() {
                self.execute_action(&mut symbol, *action);
            }

            let new_step_state = StepSymbolState::Styled(*symbol);
            step_states.insert(x, new_step_state);
        }
    }

    fn execute_action(&self, symbol: &mut Symbol, action: AnimationAction) {
        match action {
            AnimationAction::UpdateCharacter(character) => {
                symbol.value = character;
            }
            AnimationAction::UpdateForegroundColor(color) => {
                symbol.foreground_color = color;
            }
            AnimationAction::UpdateBackgroundColor(color) => {
                symbol.background_color = color;
            }
            AnimationAction::AddModifier(modifier) => {
                symbol.modifier = symbol.modifier.union(modifier);
            }
            AnimationAction::RemoveModifier(modifier) => {
                symbol.modifier.remove(modifier);
            }
            AnimationAction::RemoveAllModifiers => {
                symbol.modifier = Modifier::empty();
            }
        }
    }
}
