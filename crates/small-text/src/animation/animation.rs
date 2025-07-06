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
    AnimationTarget,
};
use crate::SymbolStyle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SymbolState {
    Styled(SymbolStyle),
    Initial(SymbolStyle),
}

impl Into<StepSymbolState> for SymbolState {
    fn into(self) -> StepSymbolState {
        match self {
            Self::Styled(style) => StepSymbolState::Untouched(style),
            Self::Initial(style) => StepSymbolState::Initial(style),
        }
    }
}

/// Represents the state of a symbol for the current
/// step.
#[derive(Clone, Copy)]
enum StepSymbolState {
    /// The symbol was styled in the current step.
    Styled(SymbolStyle),

    /// The symbol was not styled in the current or
    /// previous steps.
    Initial(SymbolStyle),

    /// The symbol was not styled in the current step,
    /// but was styled in the previous one.
    Untouched(SymbolStyle),
}

impl Into<SymbolState> for StepSymbolState {
    fn into(self) -> SymbolState {
        match self {
            Self::Styled(style) => SymbolState::Styled(style),
            Self::Initial(style) => SymbolState::Initial(style),
            Self::Untouched(style) => SymbolState::Styled(style),
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
    pub fn new(
        style: AnimationStyle,
        symbol_styles: HashMap<u16, SymbolStyle>,
    ) -> Self {
        let advancable_animation = AdvancableAnimation::new(
            style.steps,
            style.repeat_mode,
            style.advance_mode,
        );
        let symbol_states: HashMap<u16, SymbolState> = symbol_styles
            .iter()
            .map(|(x, style)| (*x, SymbolState::Initial(*style)))
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
        actions.sort_by(|a, b| targets_sorter(a.0, b.0));

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
        let symbol_styles: HashMap<u16, SymbolStyle> = self
            .symbol_states
            .iter()
            .filter_map(|(&x, state)| match state {
                SymbolState::Styled(style) => (x, *style).into(),
                SymbolState::Initial(style) => (x, *style).into(),
            })
            .collect();

        AnimationFrame { symbol_styles }
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

            let mut style = match step_state {
                StepSymbolState::Styled(style) => style,
                StepSymbolState::Untouched(style) => style,
                StepSymbolState::Initial(style) => style,
            };
            for action in actions.iter() {
                self.execute_action(&mut style, *action);
            }

            let new_step_state = StepSymbolState::Styled(*style);
            step_states.insert(x, new_step_state);
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
                style.modifier = style.modifier.union(modifier);
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

fn targets_sorter(a: AnimationTarget, b: AnimationTarget) -> Ordering {
    let priority = |item: &AnimationTarget| match item {
        AnimationTarget::Single(_) => 5,
        AnimationTarget::Range(_, _) => 4,
        AnimationTarget::Every(_) => 3,
        AnimationTarget::AllExceptEvery(_) => 2,
        AnimationTarget::Untouched => 1,
        AnimationTarget::UntouchedThisStep => 0,
    };
    priority(&a).cmp(&priority(&b))
}
