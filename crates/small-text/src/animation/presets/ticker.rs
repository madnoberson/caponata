use std::{
    collections::{
        HashMap,
        VecDeque,
    },
    time::Duration,
};

use derive_builder::Builder;

use crate::{
    AnimationAction,
    AnimationAdvanceMode,
    AnimationRepeatMode,
    AnimationStep,
    AnimationStyle,
    AnimationStyleBuilder,
    AnimationTarget,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TickerDirection {
    #[default]
    Forward,
    Backward,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Builder)]
#[builder(setter(prefix = "with", into))]
pub struct TickerAnimationStyle<'a> {
    text: &'a str,

    #[builder(default)]
    direction: TickerDirection,

    #[builder(default)]
    duration: Duration,

    #[builder(default)]
    advance_mode: AnimationAdvanceMode,

    #[builder(default)]
    repeat_mode: AnimationRepeatMode,
}

impl<'a> Into<AnimationStyle> for TickerAnimationStyle<'a> {
    fn into(self) -> AnimationStyle {
        let mut steps: Vec<AnimationStep> = Vec::new();
        let mut text_chars: VecDeque<char> = self.text.chars().collect();
        let mut current_index = (self.text.chars().count() - 1) as i16;

        while current_index >= 0 {
            if self.direction == TickerDirection::Forward {
                let last_char = text_chars.pop_back().unwrap();
                text_chars.push_front(last_char);
            } else {
                let last_char = text_chars.pop_front().unwrap();
                text_chars.push_back(last_char);
            };
            current_index -= 1;

            let mut actions: HashMap<AnimationTarget, Vec<AnimationAction>> =
                HashMap::new();
            for (char_index, char_value) in text_chars.iter().enumerate() {
                let target = AnimationTarget::Single(char_index as u16);
                let action = AnimationAction::UpdateCharacter(*char_value);
                actions.insert(target, vec![action]);
            }

            let step = AnimationStep::new(actions, None, self.duration);
            steps.push(step);
        }

        return AnimationStyleBuilder::default()
            .with_advance_mode(self.advance_mode)
            .with_repeat_mode(self.repeat_mode)
            .with_steps(steps)
            .build()
            .unwrap();
    }
}
