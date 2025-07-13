use std::{
    cmp::Ordering,
    collections::{
        HashMap,
        HashSet,
    },
    fmt::Debug,
    hash::Hash,
};

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::Widget,
};

use super::{
    SmallTextStyle,
    SymbolStyle,
    Target,
};
use crate::{
    Animation,
    AnimationStyle,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Symbol {
    pub(crate) value: char,
    pub(crate) style: SymbolStyle,
}

impl Symbol {
    pub(crate) fn new(value: char, style: SymbolStyle) -> Self {
        Self { value, style }
    }
}

/// A widget that displays one-character height text,
/// that can be animated.
///
/// # Example
///
/// ```rust
/// use std::{
///    collections::HashMap,
///    time::Duration,
/// };
///
/// use ratatui::style::{Color, Modifier};
/// use ratatui_small_text::{
///     AnimationTarget,
///     AnimationAction,
///     AnimationRepeatMode,
///     AnimationAdvanceMode,
///     AnimationStepBuilder,
///     AnimationStyleBuilder,
///     Target,
///     SymbolStyleBuilder,
///     SmallTextStyleBuilder,
///     SmallTextWidget,
/// };
///
/// let first_step = AnimationStepBuilder::default()
///     .with_duration(Duration::from_millis(100))
///     .for_target(AnimationTarget::Single(0))
///     .add_modifier(Modifier::BOLD)
///     .then()
///     .for_target(AnimationTarget::UntouchedThisStep)
///     .update_foreground_color(Color::Red)
///     .update_background_color(Color::White)
///     .add_modifier(Modifier::BOLD)
///     .then()
///     .build();
/// let second_step = AnimationStepBuilder::default()
///     .with_duration(Duration::from_millis(100))
///     .for_target(AnimationTarget::Single(1))
///     .update_foreground_color(Color::Green)
///     .remove_all_modifiers()
///     .then()
///     .for_target(AnimationTarget::UntouchedThisStep)
///     .update_foreground_color(Color::White)
///     .update_background_color(Color::Red)
///     .add_modifier(Modifier::BOLD)
///     .then()
///     .build();
/// let animation_style = AnimationStyleBuilder::default()
///     .with_repeat_mode(AnimationRepeatMode::Infinite)
///     .with_advance_mode(AnimationAdvanceMode::Auto)
///     .with_steps(vec![first_step, second_step])
///     .build()
///     .unwrap();
/// let animation_styles = HashMap::from([(0, animation_style)]);
///
/// let symbol_style = SymbolStyleBuilder::default()
///     .with_background_color(Color::Gray)
///     .with_foreground_color(Color::Blue)
///     .with_modifier(Modifier::UNDERLINED)
///     .build()
///     .unwrap();
/// let symbol_styles = HashMap::from([
///     (Target::Untouched, symbol_style),
/// ]);
/// let text_style = SmallTextStyleBuilder::default()
///     .with_text("Text example")
///     .with_symbol_styles(symbol_styles)
///     .with_animation_styles(animation_styles)
///     .build()
///     .unwrap();
///
/// let text = SmallTextWidget::new(text_style);
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SmallTextWidget<K = u8>
where
    K: PartialEq + Eq + Hash,
{
    symbols: HashMap<u16, Symbol>,
    active_animation: Option<Animation>,
    animation_styles: HashMap<K, AnimationStyle>,
}

impl<K> Widget for &mut SmallTextWidget<K>
where
    K: PartialEq + Eq + Hash,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        let required_width = self.symbols.iter().count() as u16;
        let available_width = area.width.min(required_width);

        let virtual_canvas: HashMap<u16, u16> = (0..0 + available_width)
            .zip(area.x..area.x + available_width)
            .collect();

        if self.active_animation.is_some() {
            let animation_is_ended =
                self.apply_animation(area.y, buf, &virtual_canvas);
            if animation_is_ended {
                self.disable_animation();
                self.apply_styles(area.y, buf, &virtual_canvas);
            }
        } else {
            self.apply_styles(area.y, buf, &virtual_canvas);
        }
    }
}

impl<K> SmallTextWidget<K>
where
    K: PartialEq + Eq + Hash,
{
    pub fn new(style: SmallTextStyle<K>) -> Self {
        let symbols = create_symbols(style.text, style.symbol_styles);

        Self {
            symbols,
            active_animation: None,
            animation_styles: style.animation_styles,
        }
    }

    /// Enables the animation associated with the specified key
    /// if it exists. Replaces any currently active animation
    /// with the new one.
    pub fn enable_animation(&mut self, key: &K) {
        if let Some(style) = self.animation_styles.get(key) {
            let symbols = self.symbols.clone();
            let animation = Animation::new(style.clone(), symbols);
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

    fn apply_styles(
        &mut self,
        y: u16,
        buf: &mut Buffer,
        virtual_canvas: &HashMap<u16, u16>,
    ) {
        for (x, symbol) in self.symbols.iter() {
            let real_x = virtual_canvas.get(x).unwrap();
            self.apply_style(buf, *real_x, y, *symbol);
        }
    }

    /// Applies a single animation frame to the buffer. Returns a
    /// flag indicating whether the animation was finished.
    fn apply_animation(
        &mut self,
        y: u16,
        buf: &mut Buffer,
        virtual_canvas: &HashMap<u16, u16>,
    ) -> bool {
        let active_animation = match self.active_animation.as_mut() {
            Some(animation) => animation,
            None => return true,
        };
        let current_frame = match active_animation.next_frame() {
            Some(frame) => frame,
            None => return true,
        };

        for (x, symbol) in current_frame.symbols {
            if let Some(real_x) = virtual_canvas.get(&x) {
                self.apply_style(buf, *real_x, y, symbol);
            }
        }

        false
    }

    fn apply_style(&self, buf: &mut Buffer, x: u16, y: u16, symbol: Symbol) {
        let ratatui_style = Style::default()
            .fg(symbol.style.foreground_color)
            .bg(symbol.style.background_color)
            .add_modifier(symbol.style.modifier);
        buf[(x, y)].set_char(symbol.value).set_style(ratatui_style);
    }
}

fn create_symbols(
    text: &str,
    symbol_styles: HashMap<Target, SymbolStyle>,
) -> HashMap<u16, Symbol> {
    let text_char_count = text.chars().count() as u16;

    let mut symbol_styles: Vec<(Target, SymbolStyle)> =
        symbol_styles.into_iter().collect();
    symbol_styles.sort_by(|a, b| targets_sorter(a.0, b.0));

    let symbol_values: HashMap<u16, char> = text
        .chars()
        .enumerate()
        .map(|(x, symbol_value)| (x as u16, symbol_value))
        .collect();

    let mut styled_x_coords: HashSet<u16> = HashSet::new();
    let mut resolved_symbol_styles = HashMap::new();

    for (target, style) in symbol_styles.iter() {
        if *target == Target::Untouched {
            continue;
        }
        for x in resolve_target(*target, text_char_count) {
            if let Some(symbol_value) = symbol_values.get(&x) {
                let symbol = Symbol::new(*symbol_value, *style);
                resolved_symbol_styles.insert(x, symbol);
                styled_x_coords.insert(x);
            };
        }
    }

    for (target, style) in symbol_styles.iter() {
        if *target != Target::Untouched {
            continue;
        }
        for x in 0..text_char_count {
            if styled_x_coords.contains(&x) {
                continue;
            }
            if let Some(symbol_value) = symbol_values.get(&x) {
                let symbol = Symbol::new(*symbol_value, *style);
                resolved_symbol_styles.insert(x, symbol);
            };
        }
    }

    resolved_symbol_styles
}

fn targets_sorter(a: Target, b: Target) -> Ordering {
    let priority = |item: &Target| match item {
        Target::Single(_) => 4,
        Target::Range(_, _) => 3,
        Target::Every(_) => 2,
        Target::AllExceptEvery(_) => 1,
        Target::Untouched => 0,
    };
    priority(&a).cmp(&priority(&b))
}

fn resolve_target(
    target: Target,
    char_count: u16,
) -> Box<dyn Iterator<Item = u16>> {
    let all = 0..char_count;

    match target {
        Target::Single(x) => Box::new(std::iter::once(x)),
        Target::Range(start, end) => Box::new(start..end),
        Target::Every(n) => Box::new(all.step_by(n as usize)),
        Target::AllExceptEvery(n) => Box::new(all.filter(move |x| x % n != 0)),
        Target::Untouched => Box::new(std::iter::empty()),
    }
}
