use std::{
    collections::HashMap,
    fmt::Debug,
    hash::Hash,
};

#[cfg(feature = "crossterm")]
use crossterm::event::Event;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::Widget,
};

use super::{
    Animation,
    AnimationEvent,
    AnimationStyle,
};
#[cfg(feature = "crossterm")]
use crate::InteractionEvent;
use crate::{
    SmallTextStyle,
    SmallTextWidget,
};

/// Provides a high-level API for working with animated
/// [`SmallTextWidget`] without the need for manual
/// animation control.
///
/// If you require full control over the underlying
/// animation mechanisms, consider using [`SmallTextWidget`]
/// and [`Animation`] separately.
///
/// This struct combines [`SmallTextWidget`] and [`Animation`]
/// into a single integrated component that handles
/// animation automatically.
///
/// # Example
///
/// ```rust
/// use std::{
///     time::Duration,
///     collections::HashMap,
/// };
///
/// use ratatui::style::{Color, Modifier};
/// use ratatui_small_text::{
///     Target,
///     AnimationTarget,
///     SymbolStyleBuilder,
///     AnimationAdvanceMode,
///     AnimationRepeatMode,
///     AnimationStepBuilder,
///     AnimationStyleBuilder,
///     SmallTextStyleBuilder,
///     AnimatedSmallTextWidget,
/// };
///
/// let first_step = AnimationStepBuilder::default()
///     .with_duration(Duration::from_millis(100))
///     .for_target(AnimationTarget::Every(2))
///     .update_foreground_color(Color::White)
///     .update_background_color(Color::Green)
///     .add_modifier(Modifier::BOLD)
///     .then()
///     .for_target(AnimationTarget::UntouchedThisStep)
///     .update_foreground_color(Color::Gray)
///     .update_background_color(Color::Blue)
///     .remove_all_modifiers()
///     .then()
///     .build();
/// let second_step = AnimationStepBuilder::default()
///     .with_duration(Duration::from_millis(100))
///     .for_target(AnimationTarget::Every(2))
///     .update_foreground_color(Color::Gray)
///     .update_background_color(Color::Blue)
///     .add_modifier(Modifier::BOLD)
///     .then()
///     .for_target(AnimationTarget::UntouchedThisStep)
///     .update_foreground_color(Color::White)
///     .update_background_color(Color::Green)
///     .remove_all_modifiers()
///     .then()
///     .build();
/// let animation_style = AnimationStyleBuilder::default()
///     .with_advance_mode(AnimationAdvanceMode::Auto)
///     .with_repeat_mode(AnimationRepeatMode::Finite(1))
///     .with_steps(vec![first_step, second_step])
///     .build()
///     .unwrap();
///
/// let symbol_style = SymbolStyleBuilder::default()
///     .with_background_color(Color::Gray)
///     .with_foreground_color(Color::Blue)
///     .with_modifier(Modifier::BOLD)
///     .build()
///     .unwrap();
/// let text_style = SmallTextStyleBuilder::default()
///     .with_text("Text example")
///     .for_target(Target::Every(2))
///     .set_background_color(Color::White)
///     .set_foreground_color(Color::Red)
///     .set_modifier(Modifier::UNDERLINED)
///     .then()
///     .for_target(Target::Untouched)
///     .set_style(symbol_style)
///     .then()
///     .build();
///
/// let animated_text = AnimatedSmallTextWidget::new(
///     text_style,
///     HashMap::from([(0, animation_style)]),
/// );
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct AnimatedSmallTextWidget<K>
where
    K: Debug + Hash + PartialEq + Eq,
{
    text: SmallTextWidget,
    animation_styles: HashMap<K, AnimationStyle>,
    active_animation: Option<Animation>,
}

impl<K> Widget for &mut AnimatedSmallTextWidget<K>
where
    K: Debug + Hash + PartialEq + Eq,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        if let Some(animation) = self.active_animation.as_mut()
            && let Some(frame) = animation.next_frame()
        {
            let text_symbols = self.text.mut_symbols();

            for (x, new_symbol) in frame.symbols {
                text_symbols.insert(x, new_symbol);
            }
        }

        self.text.render(area, buf);
    }
}

impl<K> AnimatedSmallTextWidget<K>
where
    K: Debug + Hash + PartialEq + Eq,
{
    pub fn new(
        text_style: SmallTextStyle,
        animation_styles: HashMap<K, AnimationStyle>,
    ) -> Self {
        let text = SmallTextWidget::new(text_style);

        Self {
            text,
            animation_styles,
            active_animation: None,
        }
    }

    pub fn take_animation_event(&mut self) -> Option<AnimationEvent> {
        if let Some(animation) = &mut self.active_animation {
            animation.take_last_event()
        } else {
            None
        }
    }

    #[cfg(feature = "crossterm")]
    pub fn handle_crossterm_event(
        &mut self,
        event: Event,
        area: Rect,
    ) -> Option<InteractionEvent> {
        self.text.handle_event(event, area)
    }

    /// Enables the animation associated with the specified key
    /// if it exists. Replaces any currently active animation
    /// with the new one.
    pub fn enable_animation(&mut self, key: &K) {
        if let Some(style) = self.animation_styles.get(key) {
            let text_symbols = self.text.symbols().clone();
            let animation = Animation::new(style.clone(), text_symbols);
            self.active_animation = Some(animation);
        }
    }

    /// Disables the currently active animation, if any;
    /// otherwise has no effect.
    pub fn disable_animation(&mut self) {
        self.active_animation = None;
    }

    /// Pauses the currently active animation if it is not
    /// already paused; otherwise has no effect.
    pub fn pause_animation(&mut self) {
        self.active_animation.as_mut().map(|a| a.pause());
    }

    /// Unpauses the currently active animation if it is
    /// paused; otherwise has no effect.
    pub fn unpause_animation(&mut self) {
        self.active_animation.as_mut().map(|a| a.unpause());
    }

    /// Advances the currently active animation if its advance
    /// mode is [`AnimationAdvanceMode::Manual`]. Has no effect
    /// if no animation is active or if it's in automatic mode.
    pub fn advance_animation(&mut self) {
        self.active_animation.as_mut().map(|a| a.advance());
    }
}
