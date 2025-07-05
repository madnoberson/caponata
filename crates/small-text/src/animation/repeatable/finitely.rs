use crate::AnimationStep;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FinitelyRepeatableAnimation {
    steps: Vec<AnimationStep>,
    current_index: usize,
    max_iteration: u16,
    current_iteration: u16,
}

impl FinitelyRepeatableAnimation {
    pub fn new(steps: Vec<AnimationStep>, max_iteration: u16) -> Self {
        Self {
            steps: steps,
            current_index: 0,
            max_iteration: max_iteration.saturating_sub(1),
            current_iteration: 0,
        }
    }

    /// Returns the current animation step if the iteration
    /// limit is not reached; otherwise returns `None`.
    pub fn current_step(&self) -> Option<AnimationStep> {
        let iterations_limit_is_reached = self.current_index
            == self.steps.len().saturating_sub(1)
            && self.current_iteration == self.max_iteration;
        if iterations_limit_is_reached {
            return None;
        }

        self.steps.get(self.current_index).unwrap().clone().into()
    }

    /// Advances the animation and returns the current step
    /// if the iteration limit is not reached; otherwise
    /// returns `None`.
    pub fn next_step(&mut self) -> Option<AnimationStep> {
        let iterations_limit_is_reached = match (
            self.current_index == self.steps.len().saturating_sub(1),
            self.current_iteration == self.max_iteration,
        ) {
            (false, false) | (false, true) => {
                self.current_index += 1;
                false
            }
            (true, false) => {
                self.current_index = 0;
                self.current_iteration += 1;
                false
            }
            (true, true) => true,
        };
        if iterations_limit_is_reached {
            return None;
        }

        self.steps.get(self.current_index).unwrap().clone().into()
    }
}
