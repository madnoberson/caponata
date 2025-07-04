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
use ratatui_small_text::{
    AnimationAction,
    AnimationAdvanceMode,
    AnimationRepeatMode,
    AnimationStepBuilder,
    AnimationStyleBuilder,
    AnimationTargetedSymbols,
    SmallTextStyleBuilder,
    SmallTextWidget,
    SymbolStyle,
    SymbolStyleBuilder,
    TargetedSymbols,
};

pub fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);

    ratatui::restore();
    result
}

fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let first_step_actions = HashMap::from([
        (
            AnimationTargetedSymbols::Single(0),
            vec![
                AnimationAction::UpdateForegroundColor(Color::White),
                AnimationAction::UpdateBackgroundColor(Color::Red),
            ],
        ),
        (
            AnimationTargetedSymbols::UntouchedThisStep,
            vec![
                AnimationAction::UpdateForegroundColor(Color::White),
                AnimationAction::UpdateBackgroundColor(Color::Green),
                AnimationAction::AddModifier(Modifier::BOLD),
            ],
        ),
    ]);
    let first_animation_step = AnimationStepBuilder::default()
        .with_actions(first_step_actions)
        .with_duration(Duration::from_millis(200))
        .build()
        .unwrap();
    let second_step_actions = HashMap::from([
        (
            AnimationTargetedSymbols::Single(0),
            vec![
                AnimationAction::UpdateForegroundColor(Color::Red),
                AnimationAction::UpdateBackgroundColor(Color::Gray),
            ],
        ),
        (
            AnimationTargetedSymbols::UntouchedThisStep,
            vec![
                AnimationAction::UpdateForegroundColor(Color::White),
                AnimationAction::UpdateBackgroundColor(Color::Yellow),
                AnimationAction::AddModifier(Modifier::BOLD),
            ],
        ),
    ]);
    let second_animation_step = AnimationStepBuilder::default()
        .with_actions(second_step_actions)
        .with_duration(Duration::from_millis(200))
        .build()
        .unwrap();

    let animation_style = AnimationStyleBuilder::default()
        .with_advance_mode(AnimationAdvanceMode::Auto)
        .with_repeat_mode(AnimationRepeatMode::Infinite)
        .with_steps([first_animation_step, second_animation_step])
        .build()
        .unwrap();

    let symbol_styles =
        HashMap::from([(TargetedSymbols::Untouched, SymbolStyle::default())]);
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
