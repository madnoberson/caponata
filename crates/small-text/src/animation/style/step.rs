use std::{
    collections::HashMap,
    time::Duration,
};

use ratatui::style::{
    Color,
    Modifier,
};

use super::{
    AnimationAction,
    AnimationTarget,
};

/// A single step in the animation for [`SmallTextWidget`].
///
/// # Example
///
/// ```rust
/// use std::time::Duration;
///
/// use ratatui::style::{Color, Modifier};
/// use ratatui_small_text::{
///     AnimationTarget,
///     AnimationAction,
///     AnimationStep,
///     AnimationStepBuilder,
/// };
///
/// let animation_step = AnimationStepBuilder::default()
///     .with_duration(Duration::from_millis(100))
///     .for_target(AnimationTarget::Single(0))
///     .update_foreground_color(Color::Gray)
///     .update_background_color(Color::Red)
///     .add_modifier(Modifier::UNDERLINED)
///     .then()
///     .for_target(AnimationTarget::Every(2))
///     .update_foreground_color(Color::White)
///     .update_background_color(Color::Green)
///     .add_modifier(Modifier::BOLD)
///     .remove_modifier(Modifier::UNDERLINED)
///     .then()
///     .for_target(AnimationTarget::UntouchedThisStep)
///     .update_foreground_color(Color::Gray)
///     .update_background_color(Color::Blue)
///     .remove_all_modifiers()
///     .then()
///     .build();
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct AnimationStep {
    /// A map from the selections of the symbol positions
    /// to the actions that applied to them.
    pub(crate) actions: HashMap<AnimationTarget, Vec<AnimationAction>>,

    /// The duration of this animation step. Once this
    /// time elapses, the animation advances to the next
    /// step.
    pub(crate) duration: Duration,
}

impl AnimationStep {
    pub fn new(
        actions: HashMap<AnimationTarget, Vec<AnimationAction>>,
        duration: Duration,
    ) -> Self {
        Self { actions, duration }
    }
}

/// A builder for constructing an [`AnimationStep`].
///
/// # Example
///
/// ```rust
/// use std::time::Duration;
///
/// use ratatui::style::Color;
/// use ratatui_small_text::{
///     AnimationTarget,
///     AnimationAction,
///     AnimationStepBuilder,
/// };
///
/// let animation_step = AnimationStepBuilder::default()
///     .with_duration(Duration::from_millis(100))
///     .for_target(AnimationTarget::Single(0))
///     .update_foreground_color(Color::Gray)
///     .update_background_color(Color::Red)
///     .add_modifier(Modifier::UNDERLINED)
///     .then()
///     .for_target(AnimationTarget::Every(2))
///     .update_foreground_color(Color::White)
///     .update_background_color(Color::Green)
///     .add_modifier(Modifier::BOLD)
///     .remove_modifier(Modifier::UNDERLINED)
///     .then()
///     .for_target(AnimationTarget::UntouchedThisStep)
///     .update_foreground_color(Color::Gray)
///     .update_background_color(Color::Blue)
///     .remove_all_modifiers()
///     .then()
///     .build();
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct AnimationStepBuilder {
    duration: Option<Duration>,
    actions: HashMap<AnimationTarget, Vec<AnimationAction>>,
}

impl AnimationStepBuilder {
    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);
        self
    }

    pub fn for_target(
        self,
        target: AnimationTarget,
    ) -> AnimationActionAccumulator {
        AnimationActionAccumulator {
            target,
            actions: Vec::new(),
            step_builder: self,
        }
    }

    pub fn build(self) -> AnimationStep {
        AnimationStep {
            actions: self.actions,
            duration: self.duration.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnimationActionAccumulator {
    target: AnimationTarget,
    actions: Vec<AnimationAction>,
    step_builder: AnimationStepBuilder,
}

impl AnimationActionAccumulator {
    pub fn update_character(self, character: char) -> Self {
        let action = AnimationAction::UpdateCharacter(character);
        self.do_action(action)
    }

    pub fn update_foreground_color(self, color: Color) -> Self {
        let action = AnimationAction::UpdateForegroundColor(color);
        self.do_action(action)
    }

    pub fn update_background_color(self, color: Color) -> Self {
        let action = AnimationAction::UpdateBackgroundColor(color);
        self.do_action(action)
    }

    pub fn add_modifier(self, modifier: Modifier) -> Self {
        let action = AnimationAction::AddModifier(modifier);
        self.do_action(action)
    }

    pub fn remove_modifier(self, modifier: Modifier) -> Self {
        let action = AnimationAction::RemoveModifier(modifier);
        self.do_action(action)
    }

    pub fn remove_all_modifiers(self) -> Self {
        let action = AnimationAction::RemoveAllModifiers;
        self.do_action(action)
    }

    pub fn do_action(mut self, action: AnimationAction) -> Self {
        self.actions.push(action);
        self
    }

    pub fn then(mut self) -> AnimationStepBuilder {
        self.step_builder
            .actions
            .extend([(self.target, self.actions)]);
        self.step_builder
    }
}
