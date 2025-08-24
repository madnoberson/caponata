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
        .with_text("Like snowflakes on a winter window ❄️")
        .for_target(Target::Every(2))
        .set_background_color(Color::Rgb(46, 52, 64))
        .set_foreground_color(Color::Rgb(143, 188, 187))
        .set_modifier(Modifier::BOLD)
        .then()
        .for_target(Target::Every(3))
        .set_background_color(Color::Rgb(59, 66, 82))
        .set_foreground_color(Color::Rgb(180, 142, 173))
        .set_modifier(Modifier::ITALIC)
        .then()
        .for_target(Target::Every(5))
        .set_background_color(Color::Rgb(76, 86, 106))
        .set_foreground_color(Color::Rgb(208, 135, 112))
        .then()
        .for_target(Target::Untouched)
        .set_background_color(Color::Rgb(236, 239, 244))
        .set_foreground_color(Color::Rgb(67, 76, 94))
        .set_modifier(Modifier::DIM)
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
