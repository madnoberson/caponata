use std::{
    collections::HashMap,
    time::Duration,
};

use derive_builder::Builder;
use ratatui::style::{
    Color,
    Modifier,
};

use crate::{
    AnimationAction,
    AnimationAdvanceMode,
    AnimationRepeatMode,
    AnimationStep,
    AnimationStyle,
    AnimationStyleBuilder,
    AnimationTarget,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Builder)]
#[builder(setter(prefix = "with", into, strip_option))]
pub struct WaveAnimationStyle {
    text_char_count: u16,

    #[builder(default)]
    duration: Duration,

    #[builder(default)]
    foreground_color: Option<Color>,

    #[builder(default)]
    background_color: Option<Color>,

    #[builder(default)]
    advance_mode: AnimationAdvanceMode,

    #[builder(default)]
    repeat_mode: AnimationRepeatMode,
}

impl<'a> Into<AnimationStyle> for WaveAnimationStyle {
    fn into(self) -> AnimationStyle {
        let mut steps: Vec<AnimationStep> = Vec::new();

        for i in 0..self.text_char_count {
            let mut step_actions = HashMap::new();

            {
                let target = AnimationTarget::UntouchedThisStep;
                let actions = vec![
                    AnimationAction::UpdateForegroundColor(Color::White),
                    AnimationAction::RemoveAllModifiers,
                ];
                step_actions.insert(target, actions);
            }

            {
                let target = AnimationTarget::Single(i);
                let mut actions = Vec::with_capacity(2);

                if let Some(color) = self.foreground_color {
                    let action = AnimationAction::UpdateForegroundColor(color);
                    actions.push(action);
                }
                if let Some(color) = self.background_color {
                    let action = AnimationAction::UpdateBackgroundColor(color);
                    actions.push(action);
                }
                step_actions.insert(target, actions);
            }

            if i.saturating_sub(1) != 0 {
                let target = AnimationTarget::Single(i - 1);
                let mut actions =
                    vec![AnimationAction::AddModifier(Modifier::DIM)];

                if let Some(color) = self.foreground_color {
                    let action = AnimationAction::UpdateForegroundColor(color);
                    actions.push(action);
                }
                if let Some(color) = self.background_color {
                    let action = AnimationAction::UpdateBackgroundColor(color);
                    actions.push(action);
                }
                step_actions.insert(target, actions);
            }

            let step = AnimationStep::new(step_actions, self.duration);
            steps.push(step);
        }

        AnimationStyleBuilder::default()
            .with_advance_mode(self.advance_mode)
            .with_repeat_mode(self.repeat_mode)
            .with_steps(steps)
            .build()
            .unwrap()
    }
}
