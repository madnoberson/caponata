use std::{
    collections::HashMap,
    time::Duration,
};

use derive_builder::Builder;
use ratatui::style::Color;

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

impl Into<AnimationStyle> for WaveAnimationStyle {
    fn into(self) -> AnimationStyle {
        let (dim_foreground_color, dimmest_foreground_color) =
            match self.foreground_color {
                Some(color) => dim_color(color),
                None => (None, None),
            };
        let (dim_background_color, dimmest_background_color) =
            match self.background_color {
                Some(color) => dim_color(color),
                None => (None, None),
            };

        let mut steps: Vec<AnimationStep> = Vec::new();

        for i in 0..self.text_char_count {
            let mut step_actions = HashMap::new();

            {
                let target = AnimationTarget::UntouchedThisStep;
                let action =
                    AnimationAction::UpdateForegroundColor(Color::White);
                step_actions.insert(target, vec![action]);
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
                let mut actions = Vec::with_capacity(2);

                if let Some(color) = dim_foreground_color {
                    let action = AnimationAction::UpdateForegroundColor(color);
                    actions.push(action);
                }
                if let Some(color) = dim_background_color {
                    let action = AnimationAction::UpdateBackgroundColor(color);
                    actions.push(action);
                }
                step_actions.insert(target, actions);
            }

            if i.saturating_sub(2) != 0 {
                let target = AnimationTarget::Single(i - 2);
                let mut actions = Vec::with_capacity(2);

                if let Some(color) = dimmest_foreground_color {
                    let action = AnimationAction::UpdateForegroundColor(color);
                    actions.push(action);
                }
                if let Some(color) = dimmest_background_color {
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

fn dim_color(color: Color) -> (Option<Color>, Option<Color>) {
    if let Some((r, g, b)) = color_to_rgb(color) {
        (
            Color::Rgb(
                r.saturating_sub(100),
                g.saturating_sub(100),
                b.saturating_sub(100),
            )
            .into(),
            Color::Rgb(
                r.saturating_sub(140),
                g.saturating_sub(140),
                b.saturating_sub(140),
            )
            .into(),
        )
    } else {
        (None, None)
    }
}

fn color_to_rgb(color: Color) -> Option<(u8, u8, u8)> {
    match color {
        Color::Rgb(r, g, b) => Some((r, g, b)),
        Color::Indexed(_) => None,
        Color::Black => Some((0, 0, 0)),
        Color::Red => Some((255, 0, 0)),
        Color::Green => Some((0, 255, 0)),
        Color::Yellow => Some((255, 255, 0)),
        Color::Blue => Some((0, 0, 255)),
        Color::Magenta => Some((255, 0, 255)),
        Color::Cyan => Some((0, 255, 255)),
        Color::White => Some((255, 255, 255)),
        Color::LightRed => Some((255, 127, 127)),
        Color::LightGreen => Some((127, 255, 127)),
        Color::LightYellow => Some((255, 255, 127)),
        Color::LightBlue => Some((127, 127, 255)),
        Color::LightMagenta => Some((255, 127, 255)),
        Color::LightCyan => Some((127, 255, 255)),
        Color::Gray => Some((128, 128, 128)),
        Color::DarkGray => Some((64, 64, 64)),
        Color::Reset => None,
    }
}
