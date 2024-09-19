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
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Cell, List, ListItem, Paragraph, Row, Table},
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

    /// Collect user input to use as query in the search menu widget
    /// Utilizes the App struct to store the current query string
    fn handle_input(&mut self) {
        self.input.clear();

        loop {
            match read().expect("Failed to read user input") {
                Event::Key(KeyEvent {
                    code: KeyCode::Char(c),
                    ..
                }) => self.input.push(c),
                Event::Key(KeyEvent {
                    code: KeyCode::Backspace,
                    ..
                }) => {
                    self.input.pop();
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Esc, ..
                }) => break,
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    ..
                }) => {
                    self.send_request(&self.input.clone());
                    break;
                }
                _ => (),
            }
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
            let header = render_header();

            let placeholder = match app.input.is_empty() {
                true => "('/') to start typing",
                false => &app.input,
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
            let data = &app.forecast;

            match &items[selected_index] {
                &"Forecast" => {
                    let table_chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(
                            [
                                Constraint::Percentage(25),
                                Constraint::Percentage(25),
                                Constraint::Percentage(25),
                                Constraint::Percentage(25),
                            ]
                            .as_ref(),
                        )
                        .split(horizontal_layout[1]);

                    let morning = render_table("Morning");
                    let afternoon = render_table("Afternoon");
                    let evening = render_table("Evening");
                    let night = render_table("Night");

                    rect.render_widget(morning, table_chunks[0]);
                    rect.render_widget(afternoon, table_chunks[1]);
                    rect.render_widget(evening, table_chunks[2]);
                    rect.render_widget(night, table_chunks[3])
                }
                &"Current" => {
                    let current = render_forecast(data);

                    rect.render_widget(current, horizontal_layout[1]);
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

        match process_keypress(&mut terminal) {
            Some(input) => match input {
                Input::QUIT => break,
                Input::SEARCH => {
                    app.handle_input();
                    continue;
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
            },
            None => (),
        }
    }

    println!("{}", app.input);

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

fn render_table<'a>(title: &'a str) -> Table<'a> {
    Table::new(vec![
        Row::new(vec![
            Cell::from("Cell 11").style(Style::default().fg(Color::White)),
            Cell::from("Cell 12").style(Style::default().fg(Color::White)),
        ]),
        Row::new(vec![
            Cell::from("Cell 21").style(Style::default().fg(Color::White)),
            Cell::from("Cell 22").style(Style::default().fg(Color::White)),
        ]),
    ])
    .style(Style::default().fg(Color::White))
    .header(Row::new(vec!["Col 1", "Col 2"]).style(Style::default().fg(Color::Yellow)))
    .block(Block::default().title(title).borders(Borders::ALL))
    .widths([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
}

fn render_forecast<'a>(data: &'a String) -> Paragraph<'a> {
    Paragraph::new(data.to_string())
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
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

    let file = File::open("../ascii/header.txt").expect("Unable to read file 'ascii/header.txt'");
    let mut buf_reader = BufReader::new(file);
    buf_reader
        .read_to_string(&mut header)
        .expect("Failed to read header file");

    // TODO: default value for header if read from file fails

    header
}
