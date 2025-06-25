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
    layout::Alignment,
    style::Color,
};
use ratatui_small_spinner::{
    SmallSpinnerStyleBuilder,
    SmallSpinnerType,
    SmallSpinnerWidget,
};

pub fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);

    ratatui::restore();
    result
}

fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let spinner_style = SmallSpinnerStyleBuilder::default()
        .with_type(SmallSpinnerType::TriangleCorners)
        .with_alignment(Alignment::Left)
        .with_foreground_color(Color::White)
        .with_interval(Duration::from_millis(100))
        .build()
        .unwrap();
    let mut spinner = SmallSpinnerWidget::new(spinner_style);

    let mut is_running = true;
    while is_running {
        terminal.draw(|frame| {
            frame.render_widget(&mut spinner, frame.area());
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
