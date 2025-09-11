use derive_builder::Builder;

use super::{
    AnimationAdvanceMode,
    AnimationRepeatMode,
    AnimationStep,
};

/// A styling configuration for the animation.
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
///     AnimationRepeatMode,
///     AnimationAdvanceMode,
///     AnimationStepBuilder,
///     AnimationStyleBuilder,
/// };
///
/// let first_step = AnimationStepBuilder::default()
///     .with_duration(Duration::from_millis(100))
///     .for_target(AnimationTarget::Single(0))
///     .add_modifier(Modifier::BOLD)
///     .then()
///     .for_target(AnimationTarget::UntouchedThisStep)
///     .update_foreground_color(Color::Red)
///     .update_background_color(Color::White)
///     .add_modifier(Modifier::BOLD)
///     .then()
///     .build();
/// let second_step = AnimationStepBuilder::default()
///     .with_duration(Duration::from_millis(100))
///     .for_target(AnimationTarget::Single(1))
///     .update_foreground_color(Color::Green)
///     .remove_all_modifiers()
///     .then()
///     .for_target(AnimationTarget::UntouchedThisStep)
///     .update_foreground_color(Color::White)
///     .update_background_color(Color::Red)
///     .add_modifier(Modifier::BOLD)
///     .then()
///     .build();
/// let animation_style = AnimationStyleBuilder::default()
///     .with_repeat_mode(AnimationRepeatMode::Infinite)
///     .with_advance_mode(AnimationAdvanceMode::Auto)
///     .with_steps(vec![first_step, second_step])
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq, Builder)]
#[builder(setter(prefix = "with", into))]
pub struct AnimationStyle {
    #[builder(default)]
    pub(crate) repeat_mode: AnimationRepeatMode,

    #[builder(default)]
    pub(crate) advance_mode: AnimationAdvanceMode,

    #[builder(default)]
    pub(crate) steps: Vec<AnimationStep>,
}

impl<'a> AnimationStyle {
    pub fn new(
        repeat_mode: AnimationRepeatMode,
        advance_mode: AnimationAdvanceMode,
        steps: Vec<AnimationStep>,
    ) -> Self {
        Self {
            repeat_mode,
            advance_mode,
            steps,
        }
    }
}
