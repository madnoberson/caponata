use crate::{
    AnimationRepeatMode,
    AnimationStep,
    animation::RepeatableAnimation,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AutomaticallyAdvancableAnimation {
    repeatable_animation: RepeatableAnimation,
}

impl AutomaticallyAdvancableAnimation {
    pub fn new(
        steps: Vec<AnimationStep>,
        repeat_mode: AnimationRepeatMode,
    ) -> Self {
        let repeatable_animation =
            RepeatableAnimation::new(steps, repeat_mode);

        Self {
            repeatable_animation,
        }
    }

    /// Returns the current animation step if the iteration
    /// limit is not reached; otherwise returns `None`.
    pub fn current_step(&self) -> Option<AnimationStep> {
        self.repeatable_animation.current_step()
    }

    /// Advances the animation and returns the current step
    /// if the iteration limit is not reached; otherwise
    /// returns `None`.
    pub fn next_step(&mut self) -> Option<AnimationStep> {
        self.repeatable_animation.next_step()
    }
}
