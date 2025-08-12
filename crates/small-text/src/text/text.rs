use std::{
    collections::{
        HashMap,
        HashSet,
    },
    fmt::Debug,
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
    pub fn new(style: SmallTextStyle) -> Self {
        let symbols = create_symbols(style.text, style.symbol_styles);
        Self { symbols }
    }

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
    let mut resolved_symbols: HashMap<u16, Symbol> = HashMap::new();

    for (target, style) in symbol_styles.iter() {
        if *target == Target::Untouched {
            continue;
        }
        for x in resolve_target(*target, text_char_count) {
            if let Some(symbol_value) = symbol_values.get(&x) {
                let symbol = Symbol::new(*symbol_value, *style);
                resolved_symbols.insert(x, symbol);
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
                resolved_symbols.insert(x, symbol);
            };
        }
    }

    for (x, value) in symbol_values {
        if styled_x_coords.contains(&x) {
            continue;
        }
        let symbol_style = SymbolStyle::default();
        let symbol = Symbol::new(value, symbol_style);
        resolved_symbols.insert(x, symbol);
    }

    resolved_symbols
}

/// Returns virtual x coordinates resolved from provided
/// target. Returns empty iterator if provided target is
/// [`Target::Untouched`].
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
