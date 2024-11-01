use crossterm::{
    event::{read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Stdout};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Cell, List, ListItem, Paragraph, Row, Table},
    Terminal,
};

use crate::{
    api::get_current_weather,
    utils::read_file,
    weather::{get_data_for_days, Weather},
};

struct AppState {
    is_search_active: bool,
    input: String,
    weather: Weather,
}

impl AppState {
    fn new(weather: Weather) -> AppState {
        AppState {
            is_search_active: false,
            input: String::new(),
            weather,
        }
    }

    fn update_input(&mut self, c: char) {
        self.input.push(c);
    }

    fn remove_last_char(&mut self) {
        self.input.pop();
    }

    fn toggle_search(&mut self) {
        self.is_search_active = !self.is_search_active;

        if !self.is_search_active {
            self.input.clear();
        }
    }
}

struct TerminalState {
    stdout: Stdout,
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl TerminalState {
    fn new() -> Result<TerminalState, io::Error> {
        let stdout = io::stdout();

        let terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

        Ok(TerminalState { stdout, terminal })
    }
}

#[derive(PartialEq, Eq)]
enum Input {
    QUIT,
    SEARCH,
    DOWN,
    UP,
    CHAR,
    ESCAPE,
    REMOVE,
    ENTER,
}

pub fn start(location: &str) -> Result<(), io::Error> {
    enable_raw_mode()?;

    let mut terminal_state = TerminalState::new().unwrap();
    execute!(
        terminal_state.stdout,
        EnterAlternateScreen,
        EnableMouseCapture
    )?;

    let weather = match get_current_weather(location, None) {
        Some(data) => data,
        None => {
            let _ = restore(&mut terminal_state.terminal);
            return Ok(());
        }
    };

    let mut app_state = AppState::new(weather);

    let items = vec!["Current", "Forecast"];
    let mut selected_index: usize = 0;

    loop {
        terminal_state.terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(7),
                        Constraint::Length(3),
                        Constraint::Min(2),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);

            // ASCII art banner
            let header = render_header();

            let placeholder = match app_state.input.is_empty() {
                true => "('/') to start typing",
                false => &app_state.input,
            };

            // search menu
            let input = render_search_menu(placeholder);

            let horizontal_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
                .split(chunks[2]);

            // list weather forecast options
            let menu = render_menu(&items, selected_index);

            // weather data (current / forecast)
            let data = &app_state.weather.to_string();

            // change data display based on sub-menu selection
            match &items[selected_index] {
                // display the forecast data for upcoming week
                &"Forecast" => {}
                // display the forecast data for today
                &"Current" => {
                    let current = render_forecast(&data);

                    let table_chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(
                            [Constraint::Percentage(35), Constraint::Percentage(65)].as_ref(),
                        )
                        .split(horizontal_layout[1]);

                    rect.render_widget(current, table_chunks[0]);
                    rect.render_widget(
                        render_table("Today's Forecast", app_state.weather.clone()),
                        table_chunks[1],
                    )
                }
                &&_ => (),
            }

            // list of available commands
            let footer = render_footer();

            rect.render_widget(header, chunks[0]);
            rect.render_widget(input, chunks[1]);
            rect.render_widget(menu, horizontal_layout[0]);
            rect.render_widget(footer, chunks[3]);
        })?;

        match process_keypress(&mut terminal_state.terminal, &mut app_state) {
            Some(input) => match input {
                Input::QUIT => break,
                Input::SEARCH => {
                    app_state.toggle_search();
                }
                Input::DOWN => {
                    if selected_index < items.len() - 1 {
                        selected_index += 1;
                    }
                }
                Input::UP => {
                    if selected_index > 0 {
                        selected_index -= 1;
                    }
                }
                Input::CHAR => {
                    continue;
                }
                Input::ESCAPE => {
                    app_state.toggle_search();
                }
                Input::REMOVE => {
                    app_state.remove_last_char();
                }
                Input::ENTER => {
                    app_state.weather = match get_current_weather(&app_state.input, None) {
                        Some(data) => data,
                        None => {
                            let _ = restore(&mut terminal_state.terminal);
                            return Ok(());
                        }
                    };
                    app_state.toggle_search();
                }
            },
            None => (),
        }
    }

    println!("{}", app_state.input);

    Ok(())
}

fn render_header<'a>() -> Paragraph<'a> {
    Paragraph::new(read_header())
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::NONE))
}

fn render_search_menu<'a>(placeholder: &'a str) -> Paragraph<'a> {
    Paragraph::new(placeholder).block(Block::default().borders(Borders::ALL).title("Search(↵)"))
}

fn render_menu<'a>(items: &'a Vec<&str>, selected_index: usize) -> List<'a> {
    let mut list_items: Vec<ListItem> = vec![];

    let mut curr: usize = 0;
    for &item in items {
        if curr == selected_index {
            list_items.push(ListItem::new(item).style(Style::default().bg(Color::Gray)));
        } else {
            list_items.push(ListItem::new(item));
        }

        curr += 1;
    }

    List::new(list_items).block(Block::default().title("Options(↓↑)").borders(Borders::ALL))
}

fn render_table<'a>(title: &'a str, data: Weather) -> Table<'a> {
    // extract morning/afternoon/evening/night times from data to fill table
    let morning = data.get_morning_data().unwrap();
    let afternoon = data.get_afternoon_data().unwrap();
    let evening = data.get_evening_data().unwrap();
    let night = data.get_night_data().unwrap();

    Table::new(vec![
        Row::new(vec![
            Cell::from("Morning").style(Style::default().fg(Color::White)),
            Cell::from(format!("{}C / {}F", &morning.temp_c, &morning.temp_f))
                .style(Style::default().fg(Color::White)),
            Cell::from(morning.condition.text.to_string()).style(Style::default().fg(Color::White)),
        ])
        .bottom_margin(1),
        Row::new(vec![
            Cell::from("Afternoon").style(Style::default().fg(Color::White)),
            Cell::from(format!("{}C / {}F", &afternoon.temp_c, &afternoon.temp_f))
                .style(Style::default().fg(Color::White)),
            Cell::from(afternoon.condition.text.to_string())
                .style(Style::default().fg(Color::White)),
        ])
        .bottom_margin(1),
        Row::new(vec![
            Cell::from("Evening").style(Style::default().fg(Color::White)),
            Cell::from(format!("{}C / {}F", &evening.temp_c, &evening.temp_f))
                .style(Style::default().fg(Color::White)),
            Cell::from(evening.condition.text.to_string()).style(Style::default().fg(Color::White)),
        ])
        .bottom_margin(1),
        Row::new(vec![
            Cell::from("Night").style(Style::default().fg(Color::White)),
            Cell::from(format!("{}C / {}F", &night.temp_c, &night.temp_f))
                .style(Style::default().fg(Color::White)),
            Cell::from(night.condition.text.to_string()).style(Style::default().fg(Color::White)),
        ]),
    ])
    .style(Style::default().fg(Color::White))
    .header(
        Row::new(vec!["Time", "Temperature", "Condition"])
            .style(Style::default().fg(Color::Yellow)),
    )
    .block(Block::default().title(title).borders(Borders::ALL))
    .widths(
        [
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ]
        .as_ref(),
    )
}

fn render_forecast<'a>(data: &'a String) -> Paragraph<'a> {
    Paragraph::new(data.to_string())
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .title("Current Conditions")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White)),
        )
}

fn render_footer<'a>() -> Paragraph<'a> {
    Paragraph::new("Press 'q': QUIT program")
        .style(Style::default().fg(Color::LightGreen))
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Commands")
                .border_type(BorderType::Plain),
        )
}

/// Detects single event (keypress) by user, and returns the appropriate command from Input enum
/// Performs any necessary cleanup (i.e. restoring terminal settings) before returning.
fn process_keypress(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app_state: &mut AppState,
) -> Option<Input> {
    match read().expect("Failed to read user input") {
        Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            ..
        }) => {
            if app_state.is_search_active {
                app_state.update_input(c);
                return Some(Input::CHAR);
            }
            match c {
                'q' => {
                    let _ = restore(terminal);
                    Some(Input::QUIT)
                }
                '/' => {
                    // app_state.toggle_search();
                    Some(Input::SEARCH)
                }
                _ => None,
            }
        }
        Event::Key(KeyEvent {
            code: KeyCode::Esc, ..
        }) => Some(Input::ESCAPE),
        Event::Key(KeyEvent {
            code: KeyCode::Down,
            ..
        }) => Some(Input::DOWN),
        Event::Key(KeyEvent {
            code: KeyCode::Up, ..
        }) => Some(Input::UP),
        Event::Key(KeyEvent {
            code: KeyCode::Backspace,
            ..
        }) => Some(Input::REMOVE),
        Event::Key(KeyEvent {
            code: KeyCode::Enter,
            ..
        }) => Some(Input::ENTER),
        _ => None,
    }
}

fn restore(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), io::Error> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn read_header() -> String {
    read_file("../ascii/header.txt")
}
