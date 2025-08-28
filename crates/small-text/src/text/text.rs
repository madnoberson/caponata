use std::{
    collections::{
        HashMap,
        HashSet,
    },
    fmt::Debug,
};

#[cfg(feature = "crossterm")]
use crossterm::event::{
    Event,
    MouseButton,
    MouseEventKind,
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{
        Color,
        Modifier,
        Style,
    },
    widgets::Widget,
};

#[cfg(feature = "crossterm")]
use super::SmallTextEvent;
use super::{
    SmallTextStyle,
    SymbolStyle,
    Target,
    targets_sorter,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Symbol {
    pub value: char,
    pub foreground_color: Color,
    pub background_color: Color,
    pub modifier: Modifier,
}

impl Symbol {
    pub(crate) fn new(value: char, style: SymbolStyle) -> Self {
        Self {
            value,
            foreground_color: style.foreground_color,
            background_color: style.background_color,
            modifier: style.modifier,
        }
    }
}

/// A widget that displays one-character height text.
///
/// # Example
///
/// ```rust
/// use ratatui::style::{Color, Modifier};
/// use ratatui_small_text::{
///     Target,
///     SymbolStyleBuilder,
///     SmallTextStyleBuilder,
///     SmallTextWidget,
/// };
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
/// let text = SmallTextWidget::new(text_style);
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SmallTextWidget {
    symbols: HashMap<u16, Symbol>,

    #[cfg(feature = "crossterm")]
    pressed_buttons: HashSet<MouseButton>,
    #[cfg(feature = "crossterm")]
    is_hovered: bool,
}

impl Widget for &mut SmallTextWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let available_width =
            self.symbols.iter().count().min(area.width as usize) as u16;

        let virtual_canvas: HashMap<u16, u16> = (0..0 + available_width)
            .zip(area.x..area.x + available_width)
            .collect();

        self.apply_styles(area.y, buf, &virtual_canvas);
    }
}

impl SmallTextWidget {
    pub fn symbols(&self) -> &HashMap<u16, Symbol> {
        &self.symbols
    }

    pub fn mut_symbols(&mut self) -> &mut HashMap<u16, Symbol> {
        &mut self.symbols
    }

    fn apply_styles(
        &mut self,
        real_y: u16,
        buf: &mut Buffer,
        virtual_canvas: &HashMap<u16, u16>,
    ) {
        for (x, symbol) in self.symbols.iter() {
            let real_x = virtual_canvas.get(x).unwrap();

            let ratatui_style = Style::default()
                .fg(symbol.foreground_color)
                .bg(symbol.background_color)
                .add_modifier(symbol.modifier);

            buf[(*real_x, real_y)]
                .set_char(symbol.value)
                .set_style(ratatui_style);
        }
    }
}

#[cfg(not(feature = "crossterm"))]
impl SmallTextWidget {
    pub fn new(style: SmallTextStyle) -> Self {
        let symbols = create_symbols(style.text, style.symbol_styles);
        Self { symbols }
    }
}

#[cfg(feature = "crossterm")]
impl SmallTextWidget {
    pub fn new(style: SmallTextStyle) -> Self {
        let symbols = create_symbols(style.text, style.symbol_styles);

        Self {
            symbols,
            pressed_buttons: HashSet::new(),
            is_hovered: false,
        }
    }

    pub fn handle_event(
        &mut self,
        event: Event,
        area: Rect,
    ) -> Option<SmallTextEvent> {
        let available_width =
            self.symbols.iter().count().min(area.width as usize) as u16;

        let virtual_canvas: HashMap<u16, u16> = (area.x
            ..area.x + available_width)
            .zip(0..0 + available_width)
            .collect();

        let mouse_event = if let Event::Mouse(mouse_event) = event {
            mouse_event
        } else {
            return None;
        };

        let symbol =
            if let Some(virtual_x) = virtual_canvas.get(&mouse_event.column) {
                self.symbols.get(virtual_x).copied()
            } else {
                None
            };

        match mouse_event.kind {
            MouseEventKind::Moved => self.on_mouse_moved(symbol),
            MouseEventKind::Down(button) => {
                self.on_mouse_button_down(symbol, button)
            }
            MouseEventKind::Up(button) => {
                self.on_mouse_button_up(symbol, button)
            }
            _ => None,
        }
    }

    fn on_mouse_moved(
        &mut self,
        symbol: Option<Symbol>,
    ) -> Option<SmallTextEvent> {
        if let Some(hovered_symbol) = symbol {
            if !self.is_hovered {
                self.is_hovered = true;
                SmallTextEvent::Hovered(hovered_symbol).into()
            } else {
                SmallTextEvent::HoveredSymbolChanged(hovered_symbol).into()
            }
        } else {
            if self.is_hovered {
                self.is_hovered = false;
                SmallTextEvent::Unhovered.into()
            } else {
                None
            }
        }
    }

    fn on_mouse_button_down(
        &mut self,
        symbol: Option<Symbol>,
        pressed_button: MouseButton,
    ) -> Option<SmallTextEvent> {
        if let Some(pressed_symbol) = symbol
            && !self.pressed_buttons.contains(&pressed_button)
        {
            self.pressed_buttons.insert(pressed_button);
            return SmallTextEvent::Pressed(pressed_symbol).into();
        }
        None
    }

    fn on_mouse_button_up(
        &mut self,
        symbol: Option<Symbol>,
        released_button: MouseButton,
    ) -> Option<SmallTextEvent> {
        if let Some(released_symbol) = symbol
            && self.pressed_buttons.contains(&released_button)
        {
            self.pressed_buttons.remove(&released_button);
            return SmallTextEvent::Released(released_symbol).into();
        }
        None
    }
}

fn create_symbols(
    text: &str,
    symbol_styles: HashMap<Target, SymbolStyle>,
) -> HashMap<u16, Symbol> {
    let text_char_count = text.chars().count() as u16;

    let mut symbol_styles = symbol_styles.clone();
    let untouched_symbols_style = symbol_styles.remove(&Target::Untouched);

    let mut symbol_styles: Vec<(Target, SymbolStyle)> =
        symbol_styles.into_iter().collect();
    symbol_styles.sort_by(|a, b| targets_sorter(a.0, b.0));

    let symbol_values: HashMap<u16, char> = text
        .chars()
        .enumerate()
        .map(|(x, symbol_value)| (x as u16, symbol_value))
        .collect();

    let mut styled_x_coords: HashSet<u16> = HashSet::new();
    let mut resolved_symbols: HashMap<u16, Symbol> = HashMap::new();

    for (target, style) in symbol_styles.iter() {
        let resolved_symbol_coords: Vec<u16> =
            resolve_target(*target, text_char_count).collect();
        let resolved_symbol_values = symbol_values
            .iter()
            .filter(|(x, _)| resolved_symbol_coords.contains(x));

        for (x, value) in resolved_symbol_values {
            let symbol = Symbol::new(*value, *style);
            resolved_symbols.insert(*x, symbol);
            styled_x_coords.insert(*x);
        }
    }

    let untouched_symbol_coords: Vec<u16> = (0..text_char_count)
        .filter(|&x| !styled_x_coords.contains(&x))
        .collect();
    let untouched_symbol_values = symbol_values
        .iter()
        .filter(|(x, _)| untouched_symbol_coords.contains(x));

    if let Some(style) = untouched_symbols_style {
        for (x, value) in untouched_symbol_values {
            let symbol = Symbol::new(*value, style);
            resolved_symbols.insert(*x, symbol);
        }
    } else {
        for (x, value) in untouched_symbol_values {
            let symbol_style = SymbolStyle::default();
            let symbol = Symbol::new(*value, symbol_style);
            resolved_symbols.insert(*x, symbol);
        }
    }

    resolved_symbols
}

/// Returns virtual x coordinates resolved from provided
/// target. Panics if provided target is [`Target::Untouched`].
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
        Target::Untouched => unreachable!(),
    }
}
