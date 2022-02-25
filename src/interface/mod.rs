use imgui::*;
use spotify_api::model::album::FullAlbum;
use spotify_api::model::album::SimplifiedAlbum;
use spotify_api::model::album::SimplifiedAlbumWithTracks;
use spotify_api::model::artist::SimplifiedArtist;
use spotify_api::model::artist::SimplifiedArtistWithAlbums;
use spotify_api::model::track::FullTrack;
use spotify_api::model::track::SimplifiedTrack;
use spotify_api::EasyAPI;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::sync::{Arc, Mutex};

use std::thread;
use std::sync::mpsc;

use std::time::Duration;
pub struct UiState {
    filter_album: String,
    genres_available: HashSet<String>,
    artists_displayed: Vec<SimplifiedArtistWithAlbums>,
    albums_displayed: Vec<SimplifiedAlbumWithTracks>,
    show_my_albums: bool,
}
pub struct PlayingContext {
    pub current_artist: Mutex<Option<SimplifiedArtist>>,
    pub current_track: Mutex<Option<SimplifiedTrack>>,
    pub current_track_full: Mutex<Option<FullTrack>>,
    pub current_album: Mutex<Option<SimplifiedAlbum>>,
}
pub struct AppContext {
    pub ui_state: UiState,
    pub playing_context: PlayingContext,
    pub albums_data: Vec<FullAlbum>,
    pub easy_api: Arc<Mutex<EasyAPI>>,
    pub rx_album_library: std::sync::mpsc::Receiver<Vec<FullAlbum>>,
    pub tx_description : std::sync::mpsc::Sender<String>,
    pub rx_description: std::sync::mpsc::Receiver<String>,
}

pub fn init_ui_state() -> UiState {
    return UiState {
        filter_album: String::new(),
        genres_available: HashSet::new(),
        artists_displayed: Vec::new(),
        albums_displayed: Vec::new(),
        show_my_albums: false,
    };
}

fn show_spotify_player(ui: &Ui, app: &mut AppContext) {
    //let _roboto = ui.push_font(roboto);
    ui.text("Now listening to:");
    ui.spacing();
    let current_artist_guard = app.playing_context.current_artist.lock().unwrap();
    match &*current_artist_guard {
        Some(artist) => {
            if ui.button(&artist.name) {
                // add an artist window
                // if there isnt one already
                let new_artist = SimplifiedArtistWithAlbums {
                    data: artist.clone(),
                    albums: Vec::new(),
                    description: String::new(),
                };
                let mut is_present = false;
                for alb in &app.ui_state.artists_displayed {
                    if alb.data.id == artist.id {
                        println!("Artist {} already shown up", artist.name);
                        is_present = true;
                        break;
                    }
                }
                if !is_present {
                    app.ui_state.artists_displayed.push(new_artist);
                }
            }
            ui.same_line();
        }
        None => {}
    };
    let current_track_guard = app.playing_context.current_track.lock().unwrap();
    match &*current_track_guard {
        Some(track) => {
            ui.text(&track.name);
            let current_album_local = app.playing_context.current_album.lock().unwrap();
            match &*current_album_local {
                Some(album) => {
                    if ui.button(&album.name) {
                        println!("Need to load album {} ", album.name);
                        let new_album = SimplifiedAlbumWithTracks {
                            data: album.clone(),
                            tracks: Vec::new(),
                        };
                        let mut is_present = false;
                        for alb in &app.ui_state.albums_displayed {
                            if alb.data.id == album.id {
                                println!("Album {} already shown up", album.name);
                                is_present = true;
                                break;
                            }
                        }
                        if !is_present {
                            app.ui_state.albums_displayed.push(new_album);
                        }
                    }
                }
                None => {}
            }
        }
        None => {}
    };
    //ui.text(&current_track.name);
    //ui.button("Pause");
    //_roboto.pop()
}

fn show_library(ui: &Ui, app: &mut AppContext) {
    if let Some(menu_bar) = ui.begin_menu_bar() {
        if let Some(menu) = ui.begin_menu("Filters") {
            for genre in &app.ui_state.genres_available {
                ui.text(format!("{}", genre));
            }
            // TODO Filter by genres with checkbox
            menu.end();
        }

        if let Some(menu) = ui.begin_menu("Sort by") {
            if ui.button("Release Date Recent..Old") {
                app.albums_data
                    .sort_by(|a, b| b.release_date.cmp(&a.release_date));
            }
            if ui.button("Release Date Old..Recent") {
                app.albums_data
                    .sort_by(|a, b| a.release_date.cmp(&b.release_date));
            }
            if ui.button("Name A..Z") {
                app.albums_data.sort_by(|a, b| a.name.cmp(&b.name));
            }
            if ui.button("Name Z..A") {
                app.albums_data.sort_by(|a, b| b.name.cmp(&a.name));
            }
            if ui.button("Artist A..Z") {
                app.albums_data
                    .sort_by(|a, b| a.artists[0].name.cmp(&b.artists[0].name));
            }
            if ui.button("Artist Z..A") {
                app.albums_data
                    .sort_by(|a, b| b.artists[0].name.cmp(&a.artists[0].name));
            }
            /*
            if ui.button("Popularity Min..Max"){
                app.albums_data.sort_by(|a, b| a.popularity.cmp(&b.popularity));
            }
            if ui.button("Popularity Max..Min"){
                app.albums_data.sort_by(|a, b| b.popularity.cmp(&a.popularity));
            }
            */
            menu.end();
        }

        menu_bar.end();
    }

    ui.spacing();
    ui.input_text("", &mut app.ui_state.filter_album).build();
    for album in &app.albums_data {
        let mut show = true;
        if !app.ui_state.filter_album.is_empty() {
            show = false;
            let my_os_filter = OsStr::new(&app.ui_state.filter_album);
            if album
                .name
                .to_string()
                .contains(my_os_filter.to_str().unwrap())
                || album
                    .release_date
                    .to_string()
                    .contains(my_os_filter.to_str().unwrap())
                || album.artists[0]
                    .name
                    .to_string()
                    .contains(my_os_filter.to_str().unwrap())
            {
                show = true;
            }
        }
        if show {
            ui.text(format!("{}", album.release_date));
            ui.same_line_with_pos(100.0);

            if ui.button(format!("{}", album.name)) {
                println!("Need to load album {} ", album.name);
                let new_album = SimplifiedAlbumWithTracks {
                    data: album.clone().to_simplified(),
                    tracks: Vec::new(),
                };
                let mut is_present = false;
                for alb in &app.ui_state.albums_displayed {
                    if alb.data.id == album.id {
                        is_present = true;
                        break;
                    }
                }
                if !is_present {
                    app.ui_state.albums_displayed.push(new_album);
                }
            }
            ui.same_line();
            if ui.button(format!("{}", &album.artists[0].name)) {
                // add an artist window
                app.ui_state
                    .artists_displayed
                    .push(SimplifiedArtistWithAlbums {
                        data: album.artists[0].clone(),
                        albums: Vec::new(),
                        description: String::new(),
                    })
            }
        }
    }
}

fn show_album(ui: &Ui, app: &mut AppContext, key: usize, key_remove: &mut usize) {
    if app.ui_state.albums_displayed[key].tracks.len() == 0 {
        let track_results = app
            .easy_api
            .lock()
            .unwrap()
            .get_tracks_from_album(&app.ui_state.albums_displayed[key].data.id)
            .unwrap();
        app.ui_state.albums_displayed[key].tracks = track_results.clone();
    }

    for track in &app.ui_state.albums_displayed[key].tracks {
        ui.text(format!("{}", track.track_number));
        ui.same_line_with_pos(30.0);
        let display_name : String;
        if track.name.len() >= 34 {
            display_name = track.name[..34].to_string() + &"...".to_string();
        } else {
            display_name = track.name.to_string();
        }
        if ui.button(format!("{}", display_name)) {
            app.easy_api
                .lock()
                .unwrap()
                .play_track(track, Some(&app.ui_state.albums_displayed[key].data))
                .unwrap();
            app.playing_context.current_artist = Mutex::new(Some(
                app.ui_state.albums_displayed[key].data.artists[0].clone(),
            ));
            app.playing_context.current_track = Mutex::new(Some(track.clone()));
            app.playing_context.current_album =
                Mutex::new(Some(app.ui_state.albums_displayed[key].data.clone()));
        }
        ui.push_text_wrap_pos_with_pos(-1.0);
        ui.same_line_with_pos(300.0);
        let duration_ms = Duration::from_millis(track.duration_ms.into());
        let seconds = duration_ms.as_secs() % 60;
        let minutes = (duration_ms.as_secs() / 60) % 60;
        ui.text(format!("{}:{}",minutes, seconds));
    }
    ui.spacing();
    if ui.button(format!("CLOSE")) {
        // Remove itself from album_displayed
        *key_remove = key + 1;
    }
}

fn show_artist(ui: &Ui, app: &mut AppContext, key: usize, key_remove: &mut usize) {
    if app.ui_state.artists_displayed[key].albums.len() == 0 {
        let albums_results = app
            .easy_api
            .lock()
            .unwrap()
            .get_albums_from_artist(&app.ui_state.artists_displayed[key].data.id)
            .unwrap();
        app.ui_state.artists_displayed[key].albums = albums_results.clone();

        let (easy_api_thread, tx_thread) = (Arc::clone(&app.easy_api), app.tx_description.clone());
        let name_artist = app.ui_state.artists_displayed[key].data.name.clone();
        thread::spawn(move || {
            let description = easy_api_thread
                .lock()
                .unwrap()
                .get_wiki_description(name_artist.clone())
                .unwrap();
            tx_thread.send(description.clone()).unwrap();
        });
    }
    match app.rx_description.try_recv() {
        Ok(description) => {
            app.ui_state.artists_displayed[key].description = description.clone();
        }
        Err(_err) => {},
    }
    if app.ui_state.artists_displayed[key].description.len() > 10 { // filter the "null" and ""
        ui.push_text_wrap_pos();
        ui.text(format!(
            "{}",
            app.ui_state.artists_displayed[key].description
        ));
    }
    //ui.pop_text_wrap_pos();
    for genre in &app.ui_state.artists_displayed[key].data.genres {
        ui.text(format!("{}", genre[0]));
    }
    ui.text("Albums");
    ui.separator();
    for album in &app.ui_state.artists_displayed[key].albums {
        ui.text(format!("{}", album.release_date));
        ui.same_line_with_pos(100.0);
        if ui.button(format!("{}", album.name)) {
            println!("Need to load album {} ", album.name);
            let new_album = SimplifiedAlbumWithTracks {
                data: album.clone(),
                tracks: Vec::new(),
            };
            let mut is_present = false;
            for alb in &app.ui_state.albums_displayed {
                if alb.data.id == album.id {
                    is_present = true;
                    break;
                }
            }
            if !is_present {
                app.ui_state.albums_displayed.push(new_album);
            }
        }
    }

    ui.spacing();
    if ui.button(format!("CLOSE")) {
        // Remove itself from artists_displayed
        *key_remove = key + 1;
    }
}

pub fn main_loop(ui: &mut Ui<'_>, app: &mut AppContext) {
    // Todo
    if let Some(menu_bar) = ui.begin_main_menu_bar() {
        if let Some(menu) = ui.begin_menu("Window") {
            //MenuItem::new("Undo").shortcut("CTRL+Z").build(ui);
            ui.checkbox("Show my albums", &mut app.ui_state.show_my_albums);
            menu.end();
        }
        menu_bar.end();
    }

    Window::new("Spotify Player")
        .size([300.0, 110.0], Condition::FirstUseEver)
        .build(ui, || {
            show_spotify_player(ui, app);
        });
    if app.ui_state.show_my_albums {
        Window::new("Spotify Albums")
            .size([500.0, 800.0], Condition::Appearing)
            .menu_bar(true)
            .build(ui, || {
                show_library(ui, app);
            });
    }

    let mut key_remove = 0;
    for key in 0..app.ui_state.albums_displayed.len() {
        Window::new(format!(
            "{} - {}",
            app.ui_state.albums_displayed[key].data.artists[0].name,
            app.ui_state.albums_displayed[key].data.name
        ))
        .size([300.0, 500.0], Condition::FirstUseEver)
        .build(ui, || {
            show_album(ui, app, key, &mut key_remove);
        });
    }
    if key_remove > 0 {
        app.ui_state.albums_displayed.remove(key_remove - 1);
    }

    let mut key_remove = 0;
    for key in 0..app.ui_state.artists_displayed.len() {
        Window::new(format!(
            "{} ",
            app.ui_state.artists_displayed[key].data.name
        ))
        .size([400.0, 500.0], Condition::FirstUseEver)
        .build(ui, || {
            show_artist(ui, app, key, &mut key_remove);
        });
    }
    if key_remove > 0 {
        app.ui_state.artists_displayed.remove(key_remove - 1);
    }

    // Data gathering
    match app.rx_album_library.try_recv() {
        Ok(albums) => {
            app.albums_data.extend(albums.clone());
            for alb in albums {
                // TODO Debug : theres no genre at all lol
                match &alb.artists[0].genres {
                    Some(genres) => {
                        for genre in genres {
                            println!("Genre added {}", genre);
                            app.ui_state.genres_available.insert(genre.clone());
                        }
                    }
                    _ => {}
                }
            }
        }
        Err(_err) => (),
    }
}
