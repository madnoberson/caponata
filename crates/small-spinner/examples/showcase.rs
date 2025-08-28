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
    let spinner_types = get_spinner_types();

    for spinner_type in spinner_types {
        let spinner_style = spinner_style_builder_ref
            .with_type(spinner_type)
            .build()
            .unwrap();
        let spinner = SmallSpinnerWidget::new(spinner_style);

        let spinner_name = get_spinner_name(spinner_type);
        spinners.push((spinner_name, spinner));
    }

    spinners
}

fn get_spinner_types() -> [SmallSpinnerType; 25] {
    [
        SmallSpinnerType::Arrow,
        SmallSpinnerType::Ascii,
        SmallSpinnerType::BlackCircle,
        SmallSpinnerType::BoxDrawing,
        SmallSpinnerType::BrailleDouble,
        SmallSpinnerType::BrailleEight,
        SmallSpinnerType::BrailleEightDouble,
        SmallSpinnerType::BrailleOne,
        SmallSpinnerType::BrailleSix,
        SmallSpinnerType::BrailleSixDouble,
        SmallSpinnerType::Canadian,
        SmallSpinnerType::Clock,
        SmallSpinnerType::DoubleArrow,
        SmallSpinnerType::HorizontalBlock,
        SmallSpinnerType::MoonPhases,
        SmallSpinnerType::OghamA,
        SmallSpinnerType::OghamB,
        SmallSpinnerType::OghamC,
        SmallSpinnerType::Parenthesis,
        SmallSpinnerType::QuadrantBlock,
        SmallSpinnerType::QuadrantBlockCrack,
        SmallSpinnerType::TriangleCorners,
        SmallSpinnerType::VerticalBlock,
        SmallSpinnerType::WhiteCircle,
        SmallSpinnerType::WhiteSquare,
    ]
}

fn get_spinner_name(spinner_type: SmallSpinnerType) -> String {
    match spinner_type {
        SmallSpinnerType::Arrow => "arrow",
        SmallSpinnerType::Ascii => "ascii",
        SmallSpinnerType::BlackCircle => "black circle",
        SmallSpinnerType::BoxDrawing => "box drawing",
        SmallSpinnerType::BrailleDouble => "braille double",
        SmallSpinnerType::BrailleEight => "braille eight",
        SmallSpinnerType::BrailleEightDouble => "braille eight double",
        SmallSpinnerType::BrailleOne => "braille one",
        SmallSpinnerType::BrailleSix => "braille six",
        SmallSpinnerType::BrailleSixDouble => "braille six double",
        SmallSpinnerType::Canadian => "canadian",
        SmallSpinnerType::Clock => "clock",
        SmallSpinnerType::DoubleArrow => "double arrow",
        SmallSpinnerType::HorizontalBlock => "horizontal block",
        SmallSpinnerType::MoonPhases => "moon phases",
        SmallSpinnerType::OghamA => "ogham a",
        SmallSpinnerType::OghamB => "ogham b",
        SmallSpinnerType::OghamC => "ogham c",
        SmallSpinnerType::Parenthesis => "parenthesis",
        SmallSpinnerType::QuadrantBlock => "quadrant block",
        SmallSpinnerType::QuadrantBlockCrack => "quadrant block crack",
        SmallSpinnerType::TriangleCorners => "triangle corners",
        SmallSpinnerType::VerticalBlock => "vertical block",
        SmallSpinnerType::WhiteCircle => "white circle",
        SmallSpinnerType::WhiteSquare => "white square",
    }
    .to_string()
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
