use std::{
    collections::HashMap,
    io,
    time::Duration,
};

use caponata_small_text::{
    AnimatedSmallTextWidget,
    AnimationAdvanceMode,
    AnimationRepeatMode,
    ScannerAnimationStyleBuilder,
    SmallTextStyleBuilder,
    TickerAnimationDirection,
    TickerAnimationStyleBuilder,
    WaveAnimationStyleBuilder,
};
use crossterm::event::{
    Event,
    KeyCode,
    poll,
    read,
};
use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    layout::{
        Alignment,
        Constraint,
        Direction,
        Layout,
        Rect,
    },
    style::{
        Color,
        Modifier,
        Stylize,
    },
    text::Line,
    widgets::Widget,
};

pub fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);

    ratatui::restore();
    result
}

fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let mut app = AppWidget::new("Small text!");

    let mut is_running = true;
    while is_running {
        terminal.draw(|frame| {
            frame.render_widget(&mut app, frame.area());
        })?;
        is_running = !handle_event()?;
    }

    Ok(())
}

struct AppWidget {
    texts: Vec<(String, AnimatedSmallTextWidget<u16>)>,
    text_count: usize,
    text_char_count: usize,
}

impl Widget for &mut AppWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let base_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Max(85), Constraint::Fill(1)])
            .split(area)[0];
        let row_layout_constraints: Vec<[Constraint; 4]> =
            (0..self.text_count.div_ceil(4))
                .map(|_| [Constraint::Fill(1); 4])
                .collect();

        for (constraints, area_y) in row_layout_constraints
            .iter()
            .zip(base_layout.y..base_layout.y + base_layout.height)
        {
            let layout_area =
                Rect::new(base_layout.x, area_y, base_layout.width, 1);
            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(constraints)
                .split(layout_area);

            for ((animation_name, text), block_area) in self
                .texts
                .iter_mut()
                .skip((area_y * 4) as usize)
                .zip(layout.iter())
            {
                let animation_name = format!("{}:", animation_name);
                let animation_name_char_count =
                    animation_name.chars().count() as u16;

                let line_area = Rect::new(
                    block_area.x,
                    block_area.y,
                    animation_name_char_count,
                    1,
                );
                Line::from(animation_name.as_ref())
                    .alignment(Alignment::Left)
                    .fg(Color::White)
                    .bg(Color::Rgb(30, 30, 30))
                    .add_modifier(Modifier::BOLD)
                    .render(line_area, buf);

                let text_area = Rect::new(
                    block_area.x + animation_name_char_count + 1,
                    block_area.y,
                    self.text_char_count as u16,
                    1,
                );
                text.render(text_area, buf);
            }
        }
    }
}

impl AppWidget {
    fn new(text: &str) -> Self {
        let mut texts = make_texts(text);
        let text_count = texts.iter().count();
        let text_char_count = text.chars().count();

        for (_, text) in texts.iter_mut() {
            text.enable_animation(&0);
        }

        AppWidget {
            texts,
            text_count,
            text_char_count,
        }
    }
}

fn make_texts(text: &str) -> Vec<(String, AnimatedSmallTextWidget<u16>)> {
    let ticker_animated_text = make_ticker_animated_text(text);
    let scanner_animated_text = make_scanner_animated_text(text);
    let wave_animated_text = make_wave_animated_text(text);

    Vec::from([
        ("Ticker".to_string(), ticker_animated_text),
        ("Scanner".to_string(), scanner_animated_text),
        ("Wave".to_string(), wave_animated_text),
    ])
}

fn make_ticker_animated_text(text: &str) -> AnimatedSmallTextWidget<u16> {
    let text_style = SmallTextStyleBuilder::default().with_text(text).build();

    let animation_style = TickerAnimationStyleBuilder::default()
        .with_direction(TickerAnimationDirection::Forward)
        .with_duration(Duration::from_millis(100))
        .with_advance_mode(AnimationAdvanceMode::Auto)
        .with_repeat_mode(AnimationRepeatMode::Infinite)
        .build()
        .unwrap()
        .into();
    let animation_styles = HashMap::from([(0, animation_style)]);

    AnimatedSmallTextWidget::new(text_style, animation_styles)
}

fn make_scanner_animated_text(text: &str) -> AnimatedSmallTextWidget<u16> {
    let text_style = SmallTextStyleBuilder::default().with_text(text).build();

    let animation_style = ScannerAnimationStyleBuilder::default()
        .with_text_style(&text_style)
        .with_duration(Duration::from_millis(100))
        .with_foreground_color(Color::Red)
        .with_advance_mode(AnimationAdvanceMode::Auto)
        .with_repeat_mode(AnimationRepeatMode::Infinite)
        .build()
        .unwrap()
        .into();
    let animation_styles = HashMap::from([(0, animation_style)]);

    AnimatedSmallTextWidget::new(text_style, animation_styles)
}

fn make_wave_animated_text(text: &str) -> AnimatedSmallTextWidget<u16> {
    let text_style = SmallTextStyleBuilder::default().with_text(text).build();

    let animation_style = WaveAnimationStyleBuilder::default()
        .with_text_style(&text_style)
        .with_duration(Duration::from_millis(100))
        .with_foreground_color(Color::Red)
        .with_advance_mode(AnimationAdvanceMode::Auto)
        .with_repeat_mode(AnimationRepeatMode::Infinite)
        .build()
        .unwrap()
        .into();
    let animation_styles = HashMap::from([(0, animation_style)]);

    AnimatedSmallTextWidget::new(text_style, animation_styles)
}

/// Handles a crossterm event and returns a flag indicating
/// whether the application should be closed.
fn handle_event() -> io::Result<bool> {
    let timeout = Duration::from_millis(100);
    if !poll(timeout)? {
        return Ok(false);
    }

    match read()? {
        Event::Key(key) => {
            if let KeyCode::Char('q') = key.code {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        _ => Ok(false),
    }
}
