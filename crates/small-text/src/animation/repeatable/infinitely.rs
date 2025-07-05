use crate::AnimationStep;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InfinitelyRepeatableAnimation {
    steps: Vec<AnimationStep>,
    current_index: usize,
}

impl InfinitelyRepeatableAnimation {
    pub fn new(steps: Vec<AnimationStep>) -> Self {
        Self {
            steps: steps,
            current_index: 0,
        }
    }

    /// Returns the current animation step.
    pub fn current_step(&self) -> AnimationStep {
        self.steps.get(self.current_index).unwrap().clone()
    }

    /// Advances the animation and returns the current
    /// animation step.
    pub fn next_step(&mut self) -> AnimationStep {
        if self.current_index != self.steps.len().saturating_sub(1) {
            self.current_index += 1;
        } else {
            self.current_index = 0;
        };

        self.steps.get(self.current_index).unwrap().clone()
    }
}
