use std::{
    collections::HashMap,
    io,
    time::Duration,
};

use crossterm::event::{
    Event,
    KeyCode,
    poll,
    read,
};
use ratatui::{
    DefaultTerminal,
    style::Color,
};
use ratatui_small_text::{
    AnimatedSmallTextWidget,
    AnimationAdvanceMode,
    AnimationRepeatMode,
    AnimationStyle,
    SmallTextStyleBuilder,
    WaveAnimationStyleBuilder,
};

pub fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);

    ratatui::restore();
    result
}

fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let animation_style: AnimationStyle = WaveAnimationStyleBuilder::default()
        .with_text_char_count(11 as u16)
        .with_duration(Duration::from_millis(100))
        .with_foreground_color(Color::Red)
        .with_advance_mode(AnimationAdvanceMode::Auto)
        .with_repeat_mode(AnimationRepeatMode::Infinite)
        .build()
        .unwrap()
        .into();
    let text_style = SmallTextStyleBuilder::default()
        .with_text("Small text!")
        .build();

    let animation_styles = HashMap::from([(0, animation_style)]);
    let mut text = AnimatedSmallTextWidget::new(text_style, animation_styles);
    text.enable_animation(&0);

    let mut is_running = true;
    while is_running {
        terminal.draw(|frame| {
            frame.render_widget(&mut text, frame.area());
        })?;
        is_running = !handle_event()?;
    }

    Ok(())
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
