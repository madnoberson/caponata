use crate::{
    AnimationAdvanceMode,
    AnimationRepeatMode,
    AnimationStep,
};

use super::{
    AutomaticallyAdvancableAnimation,
    ManuallyAdvancableAnimation,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum AdvancableAnimation {
    Manually(ManuallyAdvancableAnimation),
    Automatically(AutomaticallyAdvancableAnimation),
}

impl AdvancableAnimation {
    pub fn new(
        steps: Vec<AnimationStep>,
        repeat_mode: AnimationRepeatMode,
        advance_mode: AnimationAdvanceMode,
    ) -> Self {
        match advance_mode {
            AnimationAdvanceMode::Manual => {
                let animation =
                    ManuallyAdvancableAnimation::new(steps, repeat_mode);
                Self::Manually(animation)
            }
            AnimationAdvanceMode::Auto => {
                let animation =
                    AutomaticallyAdvancableAnimation::new(steps, repeat_mode);
                Self::Automatically(animation)
            }
        }
    }

    /// Returns the current animation step if the iteration
    /// limit is not reached; otherwise returns `None`.
    pub fn current_step(&self) -> Option<AnimationStep> {
        match self {
            Self::Manually(animation) => animation.current_step(),
            Self::Automatically(animation) => animation.current_step(),
        }
    }

    /// Advances the animation and returns the current step
    /// if the iteration limit is not reached. If the animation
    /// is manually advancable, the `advance` method must be
    /// called beforehand; otherwise returns `None`.
    pub fn next_step(&mut self) -> Option<AnimationStep> {
        match self {
            Self::Manually(animation) => animation.next_step(),
            Self::Automatically(animation) => animation.next_step(),
        }
    }

    /// If the animation is manually advancable, marks ir ready
    /// to advance on the next call to the `next_step` method.
    pub fn advance(&mut self) {
        if let Self::Manually(animation) = self {
            animation.advance();
        }
    }
}
