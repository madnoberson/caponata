use std::{
    collections::HashMap,
    fmt::Debug,
    hash::Hash,
};

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::Widget,
};

use crate::{
    Animation,
    AnimationStyle,
    SmallTextStyle,
    SmallTextWidget,
};

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
