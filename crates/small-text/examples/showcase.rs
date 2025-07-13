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
    style::{
        Color,
        Modifier,
    },
};
use ratatui_small_text::{
    AnimationAdvanceMode,
    AnimationRepeatMode,
    AnimationStepBuilder,
    AnimationStyleBuilder,
    AnimationTarget,
    SmallTextStyleBuilder,
    SmallTextWidget,
    SymbolStyle,
    Target,
};

pub fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);

    ratatui::restore();
    result
}

fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let first_animation_step = AnimationStepBuilder::default()
        .with_duration(Duration::from_secs(1))
        .for_target(AnimationTarget::Every(2))
        .update_foreground_color(Color::White)
        .update_background_color(Color::Red)
        .add_modifier(Modifier::BOLD)
        .then()
        .for_target(AnimationTarget::AllExceptEvery(2))
        .update_character('!')
        .update_foreground_color(Color::Gray)
        .update_background_color(Color::Blue)
        .remove_modifier(Modifier::BOLD)
        .then()
        .build();
    let second_animation_step = AnimationStepBuilder::default()
        .with_duration(Duration::from_secs(1))
        .for_target(AnimationTarget::AllExceptEvery(2))
        .update_character('@')
        .update_foreground_color(Color::White)
        .update_background_color(Color::Red)
        .add_modifier(Modifier::BOLD)
        .then()
        .for_target(AnimationTarget::Every(2))
        .update_foreground_color(Color::Gray)
        .update_background_color(Color::Blue)
        .remove_modifier(Modifier::BOLD)
        .then()
        .build();
    let animation_style = AnimationStyleBuilder::default()
        .with_advance_mode(AnimationAdvanceMode::Auto)
        .with_repeat_mode(AnimationRepeatMode::Infinite)
        .with_steps([first_animation_step, second_animation_step])
        .build()
        .unwrap();

    let symbol_styles =
        HashMap::from([(Target::Untouched, SymbolStyle::default())]);
    let text_style = SmallTextStyleBuilder::default()
        .with_text("Small text!")
        .with_animation_styles(HashMap::from([(0, animation_style)]))
        .with_symbol_styles(symbol_styles)
        .build()
        .unwrap();

    let mut text = SmallTextWidget::<u8>::new(text_style);
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
