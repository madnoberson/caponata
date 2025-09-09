use std::{
    collections::HashMap,
    time::Instant,
};

use ratatui::style::Modifier;

use super::{
    AdvancableAnimation,
    AnimationAction,
    AnimationEvent,
    AnimationStep,
    AnimationStyle,
    AnimationTarget,
    animation_target_sorter,
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
pub enum StepSymbolState {
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

/// Provides a high-level API of working with animations
/// for [`SmallTextWidget`] with full control over
/// behavior.
///
/// If you don't need manual control of the animation
/// mechanisms, consider using [`AnimatedSmallTextWidget`],
/// which combines [`SmallTextWidget`] and [`Animation`]
/// into a single struct.
///
/// # Example
///
/// ```rust
/// use std::{
///     time::Duration,
///     collections::HashMap,
/// };
///
/// use ratatui::style::{Color, Modifier};
/// use ratatui_small_text::{
///     Symbol,
///     AnimationTarget,
///     AnimationAdvanceMode,
///     AnimationRepeatMode,
///     AnimationStepBuilder,
///     AnimationStyleBuilder,
///     Animation,
/// };
///
/// let first_step = AnimationStepBuilder::default()
///     .with_duration(Duration::from_millis(100))
///     .for_target(AnimationTarget::Range(0, 2))
///     .update_foreground_color(Color::White)
///     .update_background_color(Color::Green)
///     .add_modifier(Modifier::BOLD)
///     .then()
///     .for_target(AnimationTarget::UntouchedThisStep)
///     .update_foreground_color(Color::Gray)
///     .update_background_color(Color::Blue)
///     .remove_all_modifiers()
///     .then()
///     .build();
/// let second_step = AnimationStepBuilder::default()
///     .with_duration(Duration::from_millis(100))
///     .for_target(AnimationTarget::Range(0, 2))
///     .update_foreground_color(Color::Gray)
///     .update_background_color(Color::Blue)
///     .add_modifier(Modifier::BOLD)
///     .then()
///     .for_target(AnimationTarget::UntouchedThisStep)
///     .update_foreground_color(Color::White)
///     .update_background_color(Color::Green)
///     .remove_all_modifiers()
///     .then()
///     .build();
/// let animation_style = AnimationStyleBuilder::default()
///     .with_advance_mode(AnimationAdvanceMode::Auto)
///     .with_repeat_mode(AnimationRepeatMode::Finite(1))
///     .with_steps(vec![first_step, second_step])
///     .build()
///     .unwrap();
///
/// let symbols = HashMap::from([
///     (0, Symbol::default()),
///     (1, Symbol::default()),
///     (2, Symbol::default()),
/// ]);
/// let mut animation = Animation::new(animation_style, symbols);
///
/// // Returns next frame of the animation.
/// let first_frame = animation.next_frame().unwrap();
///
/// // Returns a new event (`AnimationEvent::FrameGenerated`)
/// animation.take_last_event();
///
/// // Pause the animation.
/// animation.pause();
///
/// // Returns the same frame as before because animation
/// // is paused.
/// let second_frame = animation.next_frame().unwrap();
/// assert_eq!(first_frame, second_frame);
///
/// // Resume the animation.
/// animation.unpause();
///
/// // Returns a new frame since animation resumed.
/// let third_frame = animation.next_frame().unwrap();
/// assert_ne!(second_frame, third_frame);
///
/// // Returns None when animation reaches the end.
/// let fourth_frame = animation.next_frame();
/// assert_eq!(fourth_frame, None);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Animation {
    advancable_animation: AdvancableAnimation,
    symbol_states: HashMap<u16, SymbolState>,
    is_paused: bool,
    last_step_retrieved_at: Option<Instant>,
    last_event: Option<AnimationEvent>,
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
            last_event: None,
        }
    }

    pub fn take_last_event(&mut self) -> Option<AnimationEvent> {
        self.last_event.take()
    }

    pub fn next_frame(&mut self) -> Option<AnimationFrame> {
        let now = Instant::now();

        let step = if self.is_paused {
            self.advancable_animation.current_step()
        } else if self.last_step_retrieved_at.is_none() {
            self.last_step_retrieved_at = Some(now);
            self.advancable_animation.current_step()
        } else {
            let last_step_retrieved_at = self.last_step_retrieved_at?;
            self.last_step_retrieved_at = Some(now);
            self.last_event = Some(AnimationEvent::FrameGenerated);
            self.next_step(now, last_step_retrieved_at)
        };

        if let Some(step) = step {
            self.process_step(step);
            self.make_frame().into()
        } else {
            self.last_event = Some(AnimationEvent::Ended);
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

    fn next_step(
        &mut self,
        now: Instant,
        last_step_retrieved_at: Instant,
    ) -> Option<AnimationStep> {
        let current_step = self.advancable_animation.current_step()?;

        let enough_time_passed = now.duration_since(last_step_retrieved_at)
            >= current_step.duration;
        let next_step = if enough_time_passed {
            self.advancable_animation.next_step()
        } else {
            return current_step.into();
        };

        if next_step.is_some() {
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
        actions
            .sort_by(|a, b| animation_target_sorter(a.0.clone(), b.0.clone()));

        for (target, actions) in actions {
            let x_coords = self.resolve_target(target, &step_states);
            self.execute_actions(x_coords, &mut step_states, actions);
        }

        self.symbol_states = step_states
            .into_iter()
            .map(|(x, state)| (x, state.into()))
            .collect();
    }

    fn make_frame(&self) -> AnimationFrame {
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

    fn resolve_target(
        &self,
        target: AnimationTarget,
        step_states: &HashMap<u16, StepSymbolState>,
    ) -> Vec<u16> {
        let mut step_states_as_vec: Vec<(u16, StepSymbolState)> = step_states
            .clone()
            .iter()
            .map(|(x, state)| (*x, *state))
            .collect();
        step_states_as_vec.sort_by(|a, b| a.0.cmp(&b.0));

        match target {
            AnimationTarget::Single(x) => vec![x],
            AnimationTarget::Range(start, end) => (start..=end).collect(),
            AnimationTarget::Custom(callable) => {
                callable.call((step_states.clone(),)).collect()
            }
            AnimationTarget::Every(n) => step_states
                .iter()
                .map(|(x, _)| *x)
                .step_by(n as usize)
                .collect(),
            AnimationTarget::EveryFrom(n, offset) => step_states
                .iter()
                .map(|(x, _)| *x)
                .skip(offset as usize)
                .step_by(n as usize)
                .collect(),
            AnimationTarget::ExceptEvery(n) => step_states
                .iter()
                .enumerate()
                .filter(|(i, _)| *i as u16 % n != 0)
                .map(|(_, (x, _))| (*x).into())
                .collect(),
            AnimationTarget::ExceptEveryFrom(n, offset) => step_states
                .iter()
                .enumerate()
                .skip(offset as usize)
                .filter(|(i, _)| *i as u16 % n + offset != 0)
                .map(|(_, (x, _))| (*x).into())
                .collect(),
            AnimationTarget::Untouched => step_states_as_vec
                .iter()
                .filter(|(_, state)| is_symbol_untouched(*state))
                .map(|(x, _)| x)
                .copied()
                .collect(),
            AnimationTarget::UntouchedThisStep => step_states_as_vec
                .iter()
                .filter(|(_, state)| is_symbol_untouched_this_step(*state))
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

fn is_symbol_untouched(state: StepSymbolState) -> bool {
    matches!(state, StepSymbolState::Untouched(_))
}

fn is_symbol_untouched_this_step(state: StepSymbolState) -> bool {
    matches!(
        state,
        StepSymbolState::Initial(_) | StepSymbolState::Untouched(_)
    )
}
