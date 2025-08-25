use std::{
    collections::HashMap,
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

#[derive(Builder)]
#[builder(setter(prefix = "with", into, strip_option))]
pub struct TickerAnimationStyle<'a> {
    text: &'a str,

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
        let mut text_chars: Vec<char> = self.text.chars().collect();
        let mut current_index = (self.text.chars().count() - 1) as i16;

        while current_index >= 0 {
            let last_char = text_chars.pop().unwrap();
            text_chars.insert(0, last_char);
            current_index -= 1;

            let mut actions: HashMap<AnimationTarget, Vec<AnimationAction>> =
                HashMap::new();
            for (char_index, char_value) in text_chars.iter().enumerate() {
                let target = AnimationTarget::Single(char_index as u16);
                let action = AnimationAction::UpdateCharacter(*char_value);
                actions.insert(target, vec![action]);
            }

            let step = AnimationStep::new(actions, self.duration);
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
