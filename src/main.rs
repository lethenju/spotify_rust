extern crate failure;
extern crate spotify_cli;
extern crate termion;
extern crate tui;

#[allow(dead_code)]
mod interface;

use interface::{App, Tracks};
pub use spotify_cli::EasyAPI;
use std::io::{self, BufRead};
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Alignment, Constraint, Corner, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, Paragraph, SelectableList, Text, Widget};
use tui::Terminal;

use interface::util::{Event, Events};

fn main() -> Result<(), failure::Error> {
    let mut search = String::new();
    {
        println!("Search for a playlist :");
        let stdin = io::stdin();
        let mut iterator = stdin.lock().lines();
        search = iterator.next().unwrap().unwrap().to_string();
    }

    println!("Ok, lets find {}", search);

    let mut easy_api = EasyAPI::construct();
    easy_api.refresh();

    let mut final_results = Vec::new();
    easy_api.search("playlist", search.as_str(), &mut final_results);

    // App
    let mut items = Vec::<&str>::new();
    for item in &final_results {
        items.push(item);
    }

    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();
    // App
    let mut app = App::new(items);
    let mut tracks = Tracks::new();

    let mut current_artist = String::new();
    let mut current_track = String::new();

    if easy_api
        .get_currently_playing_artist(&mut current_artist)
        .is_err()
    {
        current_artist = "None".to_string();
    }
    if easy_api
        .get_currently_playing_track(&mut current_track)
        .is_err()
    {
        current_track = "None".to_string();
    }
    

    loop {
        let size = terminal.size()?;
        if size != app.size {
            terminal.resize(size)?;
            app.size = size;
        }

        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                .split(app.size);

            let style = Style::default().fg(Color::Green).bg(Color::DarkGray);

            let chunks_middle = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                .split(chunks[0]);

            SelectableList::default()
                .block(Block::default().borders(Borders::ALL).title("Playlists"))
                .items(&app.items)
                .select(app.selected)
                .style(style)
                .highlight_style(style.fg(Color::White).modifier(Modifier::Bold))
                .highlight_symbol(">")
                .render(&mut f, chunks_middle[0]);

            SelectableList::default()
                .block(Block::default().borders(Borders::ALL).title("Tracks"))
                .items(&tracks.items)
                .select(tracks.selected)
                .style(style)
                .highlight_style(style.fg(Color::White).modifier(Modifier::Bold))
                .highlight_symbol(">")
                .render(&mut f, chunks_middle[1]);

            let text = [
                Text::styled(
                    format!("Artist : {}\n", &current_artist.as_str()),
                    Style::default().fg(Color::White).modifier(Modifier::Bold),
                ),
                Text::styled(
                    format!("Track : {}", &current_track.as_str()),
                    Style::default().fg(Color::White),
                ),
            ];
            Paragraph::new(text.iter())
                .block(Block::default().title("Now Playing").borders(Borders::ALL))
                .style(Style::default().fg(Color::White).bg(Color::Black))
                .alignment(Alignment::Center)
                .wrap(true)
                .render(&mut f, chunks[1])
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                Key::Left => {
                    app.selected = None;
                }
                Key::Down => {
                    app.selected = if let Some(selected) = app.selected {
                        if selected >= app.items.len() - 1 {
                            Some(0)
                        } else {
                            Some(selected + 1)
                        }
                    } else {
                        Some(0)
                    }
                }
                Key::Up => {
                    app.selected = if let Some(selected) = app.selected {
                        if selected > 0 {
                            Some(selected - 1)
                        } else {
                            Some(app.items.len() - 1)
                        }
                    } else {
                        Some(0)
                    }
                }
                Key::Right => {
                    app.selected = if let Some(selected) = app.selected {
                        easy_api.search_and_play_first("playlist", app.items[selected]);
                        Some(selected)
                    } else {
                        Some(0)
                    }
                }
                _ => {}
            },
            Event::Tick => {
                app.advance();

                if easy_api
                    .get_currently_playing_artist(&mut current_artist)
                    .is_err()
                {
                    current_artist = "None".to_string();
                }
                if easy_api
                    .get_currently_playing_track(&mut current_track)
                    .is_err()
                {
                    current_track = "None".to_string();
                }
                
            }
        }
    }

    Ok(())
}
