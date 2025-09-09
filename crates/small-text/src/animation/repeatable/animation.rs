use crate::{
    AnimationRepeatMode,
    AnimationStep,
};

use super::{
    FinitelyRepeatableAnimation,
    InfinitelyRepeatableAnimation,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RepeatableAnimation {
    Finitely(FinitelyRepeatableAnimation),
    Infinitely(InfinitelyRepeatableAnimation),
}

impl<'a> RepeatableAnimation {
    pub fn new(
        steps: Vec<AnimationStep>,
        repeat_mode: AnimationRepeatMode,
    ) -> Self {
        match repeat_mode {
            AnimationRepeatMode::Finite(max_iteration) => {
                let animation =
                    FinitelyRepeatableAnimation::new(steps, max_iteration);
                Self::Finitely(animation)
            }
            AnimationRepeatMode::Infinite => {
                let animation = InfinitelyRepeatableAnimation::new(steps);
                Self::Infinitely(animation)
            }
        }
    }

    /// Returns the current animation step if the iteration
    /// limit is not reached; otherwise returns `None`.
    pub fn current_step(&'a self) -> Option<AnimationStep> {
        match self {
            Self::Finitely(animation) => animation.current_step(),
            Self::Infinitely(animation) => animation.current_step().into(),
        }
    }

    /// Advances the animation and returns the current step
    /// if the iteration limit is not reached; otherwise
    /// returns `None`.
    pub fn next_step(&'a mut self) -> Option<AnimationStep> {
        match self {
            Self::Finitely(animation) => animation.next_step(),
            Self::Infinitely(animation) => animation.next_step().into(),
        }
    }
}
