use crate::{
    AnimationRepeatMode,
    AnimationStep,
    animation::RepeatableAnimation,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ManuallyAdvancableAnimation {
    repeatable_animation: RepeatableAnimation,
    is_advanced: bool,
}

impl ManuallyAdvancableAnimation {
    pub fn new(
        steps: Vec<AnimationStep>,
        repeat_mode: AnimationRepeatMode,
    ) -> Self {
        let repeatable_animation =
            RepeatableAnimation::new(steps, repeat_mode);

        Self {
            repeatable_animation,
            is_advanced: false,
        }
    }
    /// Returns the current animation step if the iteration
    /// limit is not reached; otherwise returns `None`.
    pub fn current_step(&self) -> Option<AnimationStep> {
        self.repeatable_animation.current_step()
    }

    /// Advances the animation and returns the current step
    /// if the iteration limit is not reached and the `advance`
    /// method was called beforehand; otherwise returns `None`.
    pub fn next_step(&mut self) -> Option<AnimationStep> {
        if !self.is_advanced {
            return None;
        }
        self.is_advanced = false;
        self.repeatable_animation.next_step()
    }

    /// Marks the animation ready to advance on the next call
    /// to the `next_step` method.
    pub fn advance(&mut self) {
        self.is_advanced = true;
    }
}
