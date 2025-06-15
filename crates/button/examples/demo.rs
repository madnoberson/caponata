use std::{
    io,
    time::Duration,
};

use crossterm::{
    event::{
        EnableMouseCapture,
        Event,
        KeyCode,
        poll,
        read,
    },
    execute,
};
use ratatui::{
    DefaultTerminal,
    layout::{
        Constraint,
        Direction,
        Layout,
        Rect,
    },
    style::{
        Color,
        Modifier,
    },
};

use ratatui_button::{
    ButtonEvent,
    ButtonStateStyleBuilder,
    ButtonStatus,
    ButtonStyleBuilder,
    ButtonThickness,
    ButtonWidget,
};
use ratatui_small_spinner::SmallSpinnerStyleBuilder;

fn main() -> Result<(), io::Error> {
    execute!(io::stdout(), EnableMouseCapture).unwrap();

    let mut terminal = ratatui::init();
    let result = run(&mut terminal);

    ratatui::restore();
    result
}

fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let mut button = make_button();

    let mut should_exit = false;
    let mut widget_area = Rect::default();
    let mut is_spinner_enabled = false;

    while !should_exit {
        terminal.draw(|frame| {
            widget_area = allocate_area(frame.area());
            frame.render_widget(&mut button, widget_area);
        })?;
        (should_exit, is_spinner_enabled) =
            handle_event(&mut button, widget_area, is_spinner_enabled)?;
    }
    Ok(())
}

fn make_button() -> ButtonWidget<'static> {
    let spinner_style = SmallSpinnerStyleBuilder::default()
        .with_background_color(Color::Rgb(150, 0, 0))
        .with_interval(Duration::from_millis(100))
        .build()
        .unwrap();

    let normal_button_style = ButtonStateStyleBuilder::default()
        .with_text("Click or hover me!")
        .with_text_color(Color::White)
        .with_background_color(Color::Red)
        .build()
        .unwrap();
    let hovered_button_style = ButtonStateStyleBuilder::default()
        .with_text("I'm hovered!")
        .with_text_color(Color::White)
        .with_background_color(Color::LightRed)
        .with_text_modifier(Modifier::BOLD)
        .with_thickness(ButtonThickness::OneEightBlock)
        .build()
        .unwrap();
    let pressed_button_style = ButtonStateStyleBuilder::default()
        .with_text("I'm pressed!")
        .with_text_color(Color::White)
        .with_background_color(Color::Rgb(150, 0, 0))
        .with_text_modifier(Modifier::BOLD)
        .with_thickness(ButtonThickness::HalfBlock)
        .with_spinner_style(spinner_style)
        .build()
        .unwrap();
    let disabled_button_style = ButtonStateStyleBuilder::default()
        .with_text("I'm disabled!")
        .with_text_color(Color::White)
        .with_background_color(Color::DarkGray)
        .build()
        .unwrap();

    let button_style = ButtonStyleBuilder::default()
        .with_normal_style(normal_button_style)
        .with_hovered_style(hovered_button_style)
        .with_pressed_style(pressed_button_style)
        .with_disabled_style(disabled_button_style)
        .build()
        .unwrap();
    ButtonWidget::new(button_style)
}

fn allocate_area(area: Rect) -> Rect {
    let middle_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Fill(1); 10])
        .split(area)[5];
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(2),
            Constraint::Fill(1),
            Constraint::Fill(2),
        ])
        .split(middle_area)[1]
}

fn handle_event(
    button: &mut ButtonWidget,
    widget_area: Rect,
    is_spinner_enabled: bool,
) -> io::Result<(bool, bool)> {
    let timeout = Duration::from_millis(100);

    let mut button_event: Option<ButtonEvent> = None;
    let mut is_spinner_enabled = is_spinner_enabled;
    let mut should_exit = false;

    if poll(timeout)? {
        let event = read()?;

        match event {
            Event::Key(key) => {
                if let KeyCode::Char(key_code) = key.code {
                    match key_code {
                        'q' => should_exit = true,
                        'e' => {
                            if button.status() == ButtonStatus::Disabled {
                                button.enable();
                            } else {
                                button.disable();
                            }
                        }
                        's' => {
                            if is_spinner_enabled {
                                is_spinner_enabled = false;
                                button.disable_spinner();
                            } else {
                                is_spinner_enabled = true;
                                button.enable_spinner();
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {
                button_event = button.on_crossterm_event(event, widget_area);
            }
        };
    }

    if let Some(event) = button_event {
        if event == ButtonEvent::Clicked {
            if button.status() == ButtonStatus::Pressed {
                button.unpress();
            } else {
                button.press();
            }
        }
    }

    Ok((should_exit, is_spinner_enabled))
}
