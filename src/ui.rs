use color_eyre::Result;
use ratatui::{
    layout::{Constraint, Layout},
    widgets::Block,
    Frame,
};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{DefaultTerminal};
use Constraint::{Fill, Length, Min};
use crossterm::terminal::size;
use hyprland::data::Monitor;
use ratatui::layout::Constraint::Percentage;
use ratatui::layout::Rect;
use ratatui::widgets::Borders;
use crate::config::Config;
use crate::monitors::get_all_connected_monitors;

struct AppState {
    monitors: Vec<Monitor>,
    config: Config,
    middle_x: u16,
    middle_y: u16,
    fullscreen: bool,
    selected_section: usize,
}

impl AppState {
    fn new(monitors: Vec<Monitor>, config: Config, middle_x: u16, middle_y: u16) -> Self {
        Self {
            //square_x: 2,
            //square_y: 2,

            monitors,
            config,
            middle_x,
            middle_y,
            fullscreen: false,
            selected_section: 0,
        }
    }

    fn handle_input(&mut self, key: KeyCode) {
        const X_CHANGE: u16 = 10;
        const Y_CHANGE: u16 = 5;
        match key {
            /*KeyCode::Char('l') | KeyCode::Right => self.square_x += X_CHANGE,
            KeyCode::Char('h') | KeyCode::Left => self.square_x = if self.square_x > X_CHANGE { self.square_x - X_CHANGE } else { 0 },
            KeyCode::Char('j') | KeyCode::Down => self.square_y += Y_CHANGE,
            KeyCode::Char('k') | KeyCode::Up => self.square_y = if self.square_y > Y_CHANGE { self.square_y - Y_CHANGE } else { 0 },*/
            KeyCode::Char('f') => self.fullscreen = !self.fullscreen,
            KeyCode::Char('1') => self.selected_section = 0,
            KeyCode::Char('2') => self.selected_section = 1,
            KeyCode::Char('3') => self.selected_section = 2,
            _ => {}
        }
    }
}

pub(crate) fn main(config: Config) -> Result<()> {
    let monitors = get_all_connected_monitors();
    let default = config.monitors.get(&config.default).unwrap();
    let app_state = AppState::new(monitors, config.clone(), default.x as u16, default.y as u16);
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal, app_state);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, mut app_state: AppState) -> Result<()> {
    loop {
        terminal.draw(|f| render(f, &app_state))?;
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
            app_state.handle_input(key.code);
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, app_state: &AppState) {
    let display_area = Layout::vertical([Percentage(40)]);
    let [display] = display_area.areas(frame.area());

    frame.render_widget(Block::bordered().title("Displays"), display);
    for monitor in &app_state.monitors {
        let scale_factor = 1.0 / 70.0;

        let terminal_aspect_correction = get_terminal_aspect_ratio();

        let scaled_width = (monitor.width as f32 * scale_factor) as u16;
        let scaled_height = (monitor.height as f32 * scale_factor * terminal_aspect_correction) as u16;


        let monitor_rect = Rect {
            x: (display.width / 2) + (monitor.x as f32 * scale_factor) as u16,
            y: (display.height / 2) + (monitor.y as f32 * scale_factor * terminal_aspect_correction) as u16,
            width: scaled_width,
            height: scaled_height,
        };

        frame.render_widget(Block::bordered().title("Inner Rect"), monitor_rect);
    }
}

fn get_terminal_aspect_ratio() -> f32 {
    if let Ok((cols, rows)) = size() {
        (rows as f32 / cols as f32) * 2.0 // Adjust this factor based on testing
    } else {
        0.5 // Fallback correction factor
    }
}

fn render1(frame: &mut Frame, app_state: &AppState) {
    //let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
    //let [title_area, main_area, status_area] = vertical.areas(frame.area());
    let display_area = Layout::vertical([Percentage(40)]);
    let [display] = display_area.areas(frame.area());
    //let horizontal = Layout::horizontal([Fill(1); 2]);
    //let [left_area, right_area] = horizontal.areas(main_area);

    /*let inner_rect = Rect {
        x: display.x + app_state.square_x, // Offset inside right area
        y: display.y + app_state.square_y,
        width: 20, // Adjust to fit
        height: 5,
    };

    frame.render_widget(Block::bordered().title("Displays"), display);
    frame.render_widget(Block::bordered().title("Inner Rect"), inner_rect);*/
}