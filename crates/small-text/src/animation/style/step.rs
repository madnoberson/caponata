use std::{collections::HashMap, time::Duration};

use ratatui::style::{Color, Modifier};

use super::{AnimationAction, AnimationTarget};

/// A single step in the text animation.
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

/// A helper for defining animation actions for a specific
/// [`AnimationTarget`].
///
/// Created by calling [`AnimationStepBuilder::for_target`].
/// It allows chaining animation actions for the selected
/// target. Once the actions are defined, call
/// [`AnimationActionAccumulator::then`] to return to an
/// updated [`AnimationStepBuilder`] and continue constructing
/// the step.
///
/// # Example
///
/// ```rust
/// use std::time::Duration;
///
/// use ratatui::style::{Color, Modifier};
/// use ratatui_small_text::{AnimationTarget, AnimationStepBuilder};
///
/// let animation_step = AnimationStepBuilder::default()
///     .for_target(AnimationTarget::Single(0))
///     .add_modifier(Modifier::BOLD)
///     .update_background_color(Color::Blue)
///     .then()
///     .build();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnimationActionAccumulator {
    target: AnimationTarget,
    actions: Vec<AnimationAction>,
    step_builder: AnimationStepBuilder,
}

impl AnimationActionAccumulator {
    /// Adds [`AnimationAction::UpdateCharacter`] to the
    /// current target actions.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_small_text::{AnimationTarget, AnimationStepBuilder};
    ///
    /// let mut builder = AnimationStepBuilder::default()
    ///     .for_target(AnimationTarget::Single(0))
    ///     .update_character('!')
    ///     .then();
    /// ```
    pub fn update_character(self, character: char) -> Self {
        let action = AnimationAction::UpdateCharacter(character);
        self.do_action(action)
    }

    /// Adds [`AnimationAction::UpdateForegroundColor`] to the
    /// current target actions.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui::style::Color;
    /// use ratatui_small_text::{AnimationTarget, AnimationStepBuilder};
    ///
    /// let mut builder = AnimationStepBuilder::default()
    ///     .for_target(AnimationTarget::Single(0))
    ///     .update_foreground_color(Color::Blue)
    ///     .then();
    /// ```
    pub fn update_foreground_color(self, color: Color) -> Self {
        let action = AnimationAction::UpdateForegroundColor(color);
        self.do_action(action)
    }

    /// Add [`AnimationAction::UpdateBackgroundColor`] to the
    /// current target actions.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui::style::Color;
    /// use ratatui_small_text::{AnimationTarget, AnimationStepBuilder};
    ///
    /// let mut builder = AnimationStepBuilder::default()
    ///     .for_target(AnimationTarget::Single(0))
    ///     .update_background_color(Color::Yellow)
    ///     .then();
    /// ```
    pub fn update_background_color(self, color: Color) -> Self {
        let action = AnimationAction::UpdateBackgroundColor(color);
        self.do_action(action)
    }

    /// Adds [`AnimationAction::AddModifier`] to the current
    /// target actions.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui::style::Modifier;
    /// use ratatui_small_text::{AnimationTarget, AnimationStepBuilder};
    ///
    /// let mut builder = AnimationStepBuilder::default()
    ///     .for_target(AnimationTarget::Single(0))
    ///     .add_modifier(Modifier::BOLD)
    ///     .then();
    /// ```
    pub fn add_modifier(self, modifier: Modifier) -> Self {
        let action = AnimationAction::AddModifier(modifier);
        self.do_action(action)
    }

    /// Add [`AnimationAction::RemoveModifier`] to the current
    /// target actions.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui::style::Modifier;
    /// use ratatui_small_text::{AnimationTarget, AnimationStepBuilder};
    ///
    /// let mut builder = AnimationStepBuilder::default()
    ///     .for_target(AnimationTarget::Single(0))
    ///     .remove_modifier(Modifier::BOLD)
    ///     .then();
    /// ```
    pub fn remove_modifier(self, modifier: Modifier) -> Self {
        let action = AnimationAction::RemoveModifier(modifier);
        self.do_action(action)
    }

    /// Add [`AnimationAction::RemoveAllModifiers`] to the
    /// current target actions.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_small_text::{AnimationTarget, AnimationStepBuilder};
    ///
    /// let mut builder = AnimationStepBuilder::default()
    ///     .for_target(AnimationTarget::Single(0))
    ///     .remove_all_modifiers()
    ///     .then();
    /// ```
    pub fn remove_all_modifiers(self) -> Self {
        let action = AnimationAction::RemoveAllModifiers;
        self.do_action(action)
    }

    /// Add the given action to the current target actions.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_small_text::{
    ///     AnimationAction,
    ///     AnimationTarget,
    ///     AnimationStepBuilder,
    /// };
    ///
    /// let mut builder = AnimationStepBuilder::default()
    ///     .for_target(AnimationTarget::Single(0))
    ///     .do_action(AnimationAction::RemoveAllModifiers)
    ///     .then();
    /// ```
    pub fn do_action(mut self, action: AnimationAction) -> Self {
        self.actions.push(action);
        self
    }

    /// Finalizes the current block of actions and returns an
    /// updated [`AnimationStepBuilder`] with the new actions.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui::style::{Color, Modifier};
    /// use ratatui_small_text::{AnimationTarget, AnimationStepBuilder};
    ///
    /// let mut builder = AnimationStepBuilder::default()
    ///     .for_target(AnimationTarget::Single(0))
    ///     .update_background_color(Color::Green)
    ///     .add_modifier(Modifier::BOLD)
    ///     .then()
    ///     .for_target(AnimationTarget::Single(1))
    ///     .remove_all_modifiers()
    ///     .then();
    /// ```
    pub fn then(mut self) -> AnimationStepBuilder {
        let actions = self.actions.clone();
        self.step_builder.actions.extend([(self.target, actions)]);
        self.step_builder
    }
}

/// A builder for constructing an [`AnimationStep`].
///
/// This struct provides a fluent interface for creating
/// an [`AnimationStep`] by specifying the duration and a
/// series of animation actions to be applied to specific
/// targets. It allows chaining of method calls to build
/// animation step in a readable manner.
///
/// # Example
///
/// ```rust
/// use std::{
///     collections::HashMap,
///     time::Duration,
/// };
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
///     .for_target(AnimationTarget::UntouchedThisStep)
///     .update_background_color(Color::Red)
///     .then()
///     .for_target(AnimationTarget::UntouchedThisStep)
///     .update_background_color(Color::Green)
///     .then()
///     .for_target(AnimationTarget::UntouchedThisStep)
///     .update_background_color(Color::Blue)
///     .then()
///     .build();
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct AnimationStepBuilder {
    duration: Duration,
    actions: HashMap<AnimationTarget, Vec<AnimationAction>>,
}

impl AnimationStepBuilder {
    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Begins accumulating animation actions for the given
    /// target.
    ///
    /// Returns an [`AnimationActionAccumulator`] used to
    /// accumulate actions for the specified [`AnimationTarget`].
    /// Call [`AnimationActionAccumulator::then`] to return to
    /// the builder with the new actions.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui::style::{Color, Modifier};
    /// use ratatui_small_text::{AnimationTarget, AnimationStepBuilder};
    ///
    /// let builder = AnimationStepBuilder::default()
    ///     .for_target(AnimationTarget::UntouchedThisStep)
    ///     .update_background_color(Color::Red)
    ///     .add_modifier(Modifier::BOLD)
    ///     .then()
    ///     .for_target(AnimationTarget::Single(0));
    /// ```
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

    /// Finalizes the building and returns the constructed
    /// [`AnimationStep`].
    ///
    /// This method collects all accumulated actions and the
    /// optional duration into a complete [`AnimationStep`]
    /// instance.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::time::Duration;
    ///
    /// use ratatui::style::{Color, Modifier};
    /// use ratatui_small_text::{AnimationTarget, AnimationStepBuilder};
    ///
    /// let animation_step = AnimationStepBuilder::default()
    ///     .with_duration(Duration::from_millis(120))
    ///     .for_target(AnimationTarget::Single(0))
    ///     .add_modifier(Modifier::BOLD)
    ///     .then()
    ///     .build();
    /// ```
    pub fn build(self) -> AnimationStep {
        AnimationStep {
            actions: self.actions,
            duration: self.duration,
        }
    }
}
