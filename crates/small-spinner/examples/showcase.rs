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
use ratatui_small_spinner::{
    SmallSpinnerStyleBuilder,
    SmallSpinnerType,
    SmallSpinnerWidget,
};
use strum::IntoEnumIterator;

pub fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);

    ratatui::restore();
    result
}

fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let mut app = AppWidget::new();

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
    spinners: Vec<(String, SmallSpinnerWidget)>,
    spinner_count: usize,
}

impl Widget for &mut AppWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let base_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Max(100), Constraint::Fill(1)])
            .split(area)[0];
        let row_layout_constraints: Vec<[Constraint; 4]> =
            (0..self.spinner_count.div_ceil(4))
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

            for ((text, spinner), spinner_area) in self
                .spinners
                .iter_mut()
                .skip((area_y * 4) as usize)
                .zip(layout.iter())
            {
                spinner.render(*spinner_area, buf);

                let line_area = Rect::new(
                    spinner_area.x + 2,
                    spinner_area.y,
                    spinner_area.width.saturating_sub(2),
                    1,
                );
                Line::from(text.as_ref())
                    .alignment(Alignment::Left)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
                    .render(line_area, buf);
            }
        }
    }
}

impl AppWidget {
    fn new() -> Self {
        let spinners = make_spinners();
        let spinner_count = spinners.iter().count();

        Self {
            spinners,
            spinner_count,
        }
    }
}

fn make_spinners() -> Vec<(String, SmallSpinnerWidget)> {
    let mut spinner_style_builder = SmallSpinnerStyleBuilder::default();
    let spinner_style_builder_ref = spinner_style_builder
        .with_alignment(Alignment::Left)
        .with_foreground_color(Color::White)
        .with_interval(Duration::from_millis(100));

    let mut spinners = Vec::new();

    let mut spinner_types: Vec<SmallSpinnerType> =
        SmallSpinnerType::iter().collect();
    spinner_types.sort_by(|a, b| a.as_ref().cmp(b.as_ref()));

    for spinner_type in spinner_types {
        let spinner_style = spinner_style_builder_ref
            .with_type(spinner_type)
            .build()
            .unwrap();
        let spinner = SmallSpinnerWidget::new(spinner_style);

        spinners.push((spinner_type.as_ref().to_string(), spinner));
    }

    spinners
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
