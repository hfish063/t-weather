use crossterm::{
    event::{read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    fs::File,
    io::{self, BufReader, Read, Stdout},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Terminal,
};

use crate::api::get_current_weather;

struct App {
    input: String,
    forecast: String,
}

impl App {
    fn new() -> App {
        App {
            input: String::new(),
            forecast: String::new(),
        }
    }

    /// Send request to weather API, and update the forecast field with resulting data
    fn send_request(&mut self, location: &str) {
        self.forecast = match get_current_weather(location, None) {
            Some(response) => response.to_string(),
            None => String::default(),
        }
    }
}

#[derive(PartialEq, Eq)]
enum Input {
    QUIT,
    SEARCH,
    DOWN,
    UP,
}

pub fn start(location: &str) -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    app.send_request(&location);

    let items = vec!["Current", "Forecast"];
    let mut selected_index: usize = 0;

    loop {
        terminal.draw(|rect| {
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
            let header = Paragraph::new(read_header())
                .style(Style::default().fg(Color::Cyan))
                .block(Block::default().borders(Borders::NONE));

            let placeholder = match app.input.is_empty() {
                true => "('/') to start typing",
                false => &app.input,
            };

            // search menu
            let input = Paragraph::new(placeholder)
                .block(Block::default().borders(Borders::ALL).title("Search(↵)"));

            let horizontal_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
                .split(chunks[2]);

            // weather forecast body
            let body: Block<'_> = Block::default()
                .title(match &items[selected_index] {
                    &"Current" => "Current Weather",
                    &"Forecast" => "Forecasted Weather",
                    &&_ => "",
                })
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain)
                .style(Style::default().fg(Color::White));

            // list weather forecast options
            let mut list_items: Vec<ListItem> = vec![];

            let mut curr: usize = 0;
            for &item in &items {
                if curr == selected_index {
                    list_items.push(ListItem::new(item).style(Style::default().bg(Color::Gray)));
                } else {
                    list_items.push(ListItem::new(item));
                }

                curr += 1;
            }

            let menu = List::new(list_items)
                .block(Block::default().title("Options(↓↑)").borders(Borders::ALL))
                .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                .highlight_symbol(">>");

            let data = &app.forecast;

            let forecast = Paragraph::new(data.to_string())
                .style(Style::default().fg(Color::White))
                .block(body);

            // list of available commands
            let footer = Paragraph::new("Press 'q': QUIT program")
                .style(Style::default().fg(Color::LightGreen))
                .alignment(Alignment::Left)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .title("Commands")
                        .border_type(BorderType::Plain),
                );

            rect.render_widget(header, chunks[0]);
            rect.render_widget(input, chunks[1]);
            rect.render_widget(forecast, horizontal_layout[1]);
            rect.render_widget(menu, horizontal_layout[0]);
            rect.render_widget(footer, chunks[3]);
        })?;

        match process_keypress(&mut terminal) {
            Some(input) => {
                if input == Input::QUIT {
                    break;
                } else if input == Input::SEARCH {
                    handle_input(&mut app);
                    continue;
                } else if input == Input::DOWN {
                    selected_index = 1;
                } else if input == Input::UP {
                    selected_index = 0;
                }
            }
            None => (),
        }
    }

    println!("{}", app.input);

    Ok(())
}

/// Detects single event (keypress) by user, and returns the appropriate command from Input enum
/// Performs any necessary cleanup (i.e. restoring terminal settings) before returning.
fn process_keypress(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Option<Input> {
    match read().expect("Failed to read user input") {
        Event::Key(KeyEvent {
            code: KeyCode::Char('q'),
            ..
        }) => {
            let _ = restore(terminal);
            Some(Input::QUIT)
        }
        Event::Key(KeyEvent {
            code: KeyCode::Char('/'),
            ..
        }) => Some(Input::SEARCH),
        Event::Key(KeyEvent {
            code: KeyCode::Down,
            ..
        }) => Some(Input::DOWN),
        Event::Key(KeyEvent {
            code: KeyCode::Up, ..
        }) => Some(Input::UP),
        _ => None,
    }
}

/// Collect user input to use as query in the search menu widget
/// Utilizes the App struct to store the current query string
fn handle_input(app: &mut App) {
    app.input.clear();

    loop {
        match read().expect("Failed to read user input") {
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                ..
            }) => app.input.push(c),
            Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                ..
            }) => {
                app.input.pop();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => break,
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => {
                app.send_request(&app.input.clone());
                break;
            }
            _ => (),
        }
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
    let mut header = String::new();

    let file = match File::open("../ascii/header.txt") {
        Ok(result) => result,
        Err(_) => panic!("Unable to read file 'ascii.txt'"),
    };

    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_string(&mut header).unwrap();

    // TODO: default value for header if read from file fails

    header
}

/// Convert data to user-friendly format, and include corresponding ASCII art for the conditions
fn render_data(data: &str) -> Option<String> {
    let result = String::default();

    // read_condition_art(data.condition)

    Some(result)
}

fn read_condition_art(condition: &str) -> String {
    String::default()
}
