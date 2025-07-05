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
    widgets::Widget,
};

use super::{
    Animation,
    AnimationStyle,
    SmallTextStyle,
    SymbolStyle,
    Target,
};

#[derive(Debug, Default, Clone)]
struct Symbol {
    real_x: u16,
    value: char,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SmallTextWidget<'a, K = u8>
where
    K: PartialEq + Eq + Hash,
{
    text: &'a str,
    text_char_count: u16,

    symbol_styles: Vec<(Target, SymbolStyle)>,

    active_animation: Option<Animation>,
    animation_styles: HashMap<K, AnimationStyle>,
}

impl<'a, K> Widget for &mut SmallTextWidget<'a, K>
where
    K: PartialEq + Eq + Hash,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        let available_width = area.width.min(self.text_char_count);

        let symbols: Vec<Symbol> = (area.x..area.x + available_width)
            .zip(self.text.chars())
            .map(|(real_x, value)| Symbol { real_x, value })
            .collect();
        let virtual_canvas: HashMap<u16, Symbol> =
            (0..0 + available_width).zip(symbols).collect();

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

impl<'a, K> SmallTextWidget<'a, K>
where
    K: PartialEq + Eq + Hash,
{
    pub fn new(style: SmallTextStyle<'a, K>) -> Self {
        let mut symbol_styles: Vec<(Target, SymbolStyle)> =
            style.symbol_styles.into_iter().collect();
        symbol_styles.sort_by(|a, b| targets_sorter(a.0, b.0));

        Self {
            text: style.text,
            text_char_count: style.text.chars().count() as u16,
            symbol_styles: symbol_styles,
            active_animation: None,
            animation_styles: style.animation_styles,
        }
    }

    /// Enables the animation associated with the specified key
    /// if it exists. Replaces any currently active animation
    /// with the new one.
    pub fn enable_animation(&mut self, key: &K) {
        if let Some(style) = self.animation_styles.get(key) {
            let symbol_styles = self.calculate_symbol_styles();
            let animation = Animation::new(style.clone(), symbol_styles);
            self.active_animation = Some(animation);
        }
    }

    /// Disables the currently active animation, if any.
    pub fn disable_animation(&mut self) {
        self.active_animation = None;
    }

    /// Pauses the currently active animation if it is not
    /// already paused.
    pub fn pause_animation(&mut self) {
        if let Some(animation) = self.active_animation.as_mut() {
            animation.pause();
        }
    }

    /// Unpauses the currently active animation if it is
    /// paused.
    pub fn unpause_animation(&mut self) {
        if let Some(animation) = self.active_animation.as_mut() {
            animation.unpause();
        }
    }

    /// Advances the currently active animation if its advance
    /// mode is [`AnimationAdvanceMode::Manual`].
    pub fn advance_animation(&mut self) {
        if let Some(animation) = self.active_animation.as_mut() {
            animation.advance();
        }
    }

    fn apply_styles(
        &mut self,
        y: u16,
        buf: &mut Buffer,
        virtual_canvas: &HashMap<u16, Symbol>,
    ) {
        let mut unstyled_symbol_x_coords: HashSet<u16> =
            virtual_canvas.keys().copied().collect();

        for (target, style) in self.symbol_styles.iter() {
            match target {
                Target::Single(x) => {
                    if let Some(symbol) = virtual_canvas.get(x) {
                        buf[(symbol.real_x, y)]
                            .set_char(symbol.value)
                            .set_bg(style.background_color)
                            .set_fg(style.foreground_color);

                        unstyled_symbol_x_coords.remove(x);
                    }
                }
                Target::Range(start, end) => {
                    for x in *start..*end {
                        if let Some(symbol) = virtual_canvas.get(&x) {
                            buf[(symbol.real_x, y)]
                                .set_char(symbol.value)
                                .set_bg(style.background_color)
                                .set_fg(style.foreground_color);
                            unstyled_symbol_x_coords.remove(&x);
                        }
                    }
                }
                Target::Untouched => {
                    for x in unstyled_symbol_x_coords.iter() {
                        if let Some(symbol) = virtual_canvas.get(&x) {
                            buf[(symbol.real_x, y)]
                                .set_char(symbol.value)
                                .set_bg(style.background_color)
                                .set_fg(style.foreground_color);
                        }
                    }
                }
            }
        }
    }

    fn calculate_symbol_styles(&self) -> HashMap<u16, SymbolStyle> {
        let mut unstyled_symbol_x_coords: HashSet<u16> =
            (0..self.text_char_count).collect();
        let mut symbol_styles: HashMap<u16, SymbolStyle> = HashMap::new();

        for (target, style) in self.symbol_styles.iter() {
            match target {
                Target::Single(x) => {
                    unstyled_symbol_x_coords.remove(x);
                    symbol_styles.insert(*x, *style);
                }
                Target::Range(start, end) => {
                    for x in *start..*end {
                        unstyled_symbol_x_coords.remove(&x);
                        symbol_styles.insert(x, *style);
                    }
                }
                Target::Untouched => {
                    for x in unstyled_symbol_x_coords.iter() {
                        symbol_styles.insert(*x, *style);
                    }
                }
            }
        }

        symbol_styles
    }

    fn apply_animation(
        &mut self,
        y: u16,
        buf: &mut Buffer,
        virtual_canvas: &HashMap<u16, Symbol>,
    ) -> bool {
        let active_animation = match self.active_animation.as_mut() {
            Some(animation) => animation,
            None => return true,
        };
        let current_frame = match active_animation.next_frame() {
            Some(frame) => frame,
            None => return true,
        };

        for (x, style) in current_frame.symbol_styles {
            if let Some(symbol) = virtual_canvas.get(&x) {
                buf[(symbol.real_x, y)]
                    .set_char(symbol.value)
                    .set_bg(style.background_color)
                    .set_fg(style.foreground_color);
            }
        }

        false
    }
}

fn targets_sorter(a: Target, b: Target) -> Ordering {
    let priority = |item: &Target| match item {
        Target::Single(_) => 2,
        Target::Range(_, _) => 1,
        Target::Untouched => 0,
    };
    priority(&a).cmp(&priority(&b))
}
