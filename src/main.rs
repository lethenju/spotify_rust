extern crate failure;
extern crate spotify_cli;
extern crate termion;
extern crate tui;

#[allow(dead_code)]
mod interface;

use interface::{Albums, Tracks};
pub use spotify_cli::EasyAPI;
use std::io::{self, stdout, BufRead, Read, Write};
use std::str;
use std::thread;
use std::time::Duration;
use termion::async_stdin;
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
    /*let mut search = String::new();
    {
        println!("Search for a playlist :");
        let stdin = io::stdin();
        let mut iterator = stdin.lock().lines();
        search = iterator.next().unwrap().unwrap().to_string();
    }

    println!("Ok, lets find {}", search);
*/
    let mut easy_api = EasyAPI::construct();
    easy_api.refresh().unwrap();

    let mut album_names = Vec::new();
    easy_api.get_my_albums(&mut album_names).unwrap();
    let mut album_ids = Vec::new();
    easy_api.get_my_albums_ids(&mut album_ids).unwrap();

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

    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();
    // App
    let mut albums = Albums::new(album_names, album_ids);
    let mut tracks = Tracks::new();

    let mut track_names = Vec::new();
    let mut track_items = Vec::<Vec<&str>>::new();
    let mut track_ids = Vec::new();

    loop {
        let size = terminal.size()?;
        if size != albums.size {
            terminal.resize(size)?;
            albums.size = size;
        }

        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                .split(size);

            let style = Style::default().fg(Color::Green).bg(Color::DarkGray);

            let chunks_middle = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                .split(chunks[0]);

            SelectableList::default()
                .block(Block::default().borders(Borders::ALL).title("Albums"))
                .items(&albums.album_name)
                .select(albums.selected)
                .style(style)
                .highlight_style(style.fg(Color::White).modifier(Modifier::Bold))
                .highlight_symbol(">")
                .render(&mut f, chunks_middle[0]);

            SelectableList::default()
                .block(Block::default().borders(Borders::ALL).title("Tracks"))
                .items(&tracks.track_name)
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
                    tracks.selected = None;
                    albums.selected = Some(0);
                }
                Key::Down => {
                    albums.selected = if let Some(selected) = albums.selected {
                        if selected >= albums.album_name.len() - 1 {
                            Some(0)
                        } else {
                            Some(selected + 1)
                        }
                    } else {
                        None
                    };
                    tracks.selected = if let Some(selected) = tracks.selected {
                        if selected >= tracks.track_name.len() - 1 {
                            Some(0)
                        } else {
                            Some(selected + 1)
                        }
                    } else {
                        None
                    }
                }
                Key::Up => {
                    albums.selected = if let Some(selected) = albums.selected {
                        if selected > 0 {
                            Some(selected - 1)
                        } else {
                            Some(albums.album_name.len() - 1)
                        }
                    } else {
                        None
                    };

                    tracks.selected = if let Some(selected) = tracks.selected {
                        if selected > 0 {
                            Some(selected - 1)
                        } else {
                            Some(tracks.track_name.len() - 1)
                        }
                    } else {
                        None
                    }
                }
                Key::Right => {
                    tracks.selected = if let Some(selected) = tracks.selected {
                        easy_api
                            .play_track_from_id(&tracks.track_id[selected])
                            .unwrap();
                        Some(selected)
                    } else {
                        Some(1)
                    };

                    albums.selected = if let Some(selected) = albums.selected {
                        easy_api
                            .get_tracks_from_album(&albums.album_id[selected], &mut track_names)
                            .unwrap();

                        easy_api
                            .get_tracks_id_from_album(&albums.album_id[selected], &mut track_ids)
                            .unwrap();

                        tracks.clear_tracks();
                        tracks.add_tracks(&mut track_names, &mut track_ids);
                        albums.selected = None;
                        tracks.selected = Some(0);

                        Some(selected)
                    } else {
                        None
                    };
                    albums.selected = None
                }

                _ => {}
            },
            Event::Tick => {
                albums.advance();

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
