use std::{
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
    SmallTextStyleBuilder,
    SmallTextWidget,
    Target,
};

pub fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);

    ratatui::restore();
    result
}

fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let text_style = SmallTextStyleBuilder::default()
        .with_text("Small text!")
        .for_target(Target::Every(2))
        .set_background_color(Color::Blue)
        .set_foreground_color(Color::Red)
        .set_modifier(Modifier::UNDERLINED)
        .then()
        .for_target(Target::Every(3))
        .set_background_color(Color::Green)
        .set_foreground_color(Color::Black)
        .set_modifier(Modifier::BOLD)
        .then()
        .for_target(Target::Untouched)
        .set_background_color(Color::LightYellow)
        .set_foreground_color(Color::White)
        .set_modifier(Modifier::ITALIC)
        .then()
        .build();
    let mut text = SmallTextWidget::new(text_style);

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
