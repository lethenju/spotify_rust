use imgui::*;
use spotify_api::model::album::FullAlbum;
use spotify_api::model::album::SimplifiedAlbum;
use spotify_api::model::album::SimplifiedAlbumWithTracks;
use spotify_api::model::artist::SimplifiedArtistWithAlbums;
use spotify_api::model::track::SimplifiedTrack;
use spotify_api::model::context::FullPlayingContext;
use spotify_api::model::context::FullPlayingContextTimeStamped;

use spotify_api::EasyAPI;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::sync::{Arc, Mutex};

use rand::*;
use std::thread;

use std::time::Duration;
use std::time::{SystemTime};
pub struct UiState {
    filter_album: String,
    genres_available: HashSet<String>,
    artists_displayed: Vec<SimplifiedArtistWithAlbums>,
    albums_displayed: Vec<SimplifiedAlbumWithTracks>,
    show_my_albums: bool,
    dark_theme: bool,
    pub font_normal : Option<FontId>,
    pub font_header1 : Option<FontId>,
    pub font_header2 : Option<FontId>,
    pub font_light : Option<FontId>,
}

pub struct AppContext {
    pub ui_state: UiState,
    pub playing_context: Option<FullPlayingContextTimeStamped>,
    pub albums_data: Vec<FullAlbum>,
    pub easy_api: Arc<Mutex<EasyAPI>>,
    pub rx_album_library: std::sync::mpsc::Receiver<Vec<FullAlbum>>,
    pub tx_description : std::sync::mpsc::Sender<String>,
    pub rx_description: std::sync::mpsc::Receiver<String>,
    pub tx_album_tracks : std::sync::mpsc::Sender<Vec<SimplifiedTrack>>,
    pub rx_album_tracks: std::sync::mpsc::Receiver<Vec<SimplifiedTrack>>,
    pub tx_albums_from_artist : std::sync::mpsc::Sender<Vec<SimplifiedAlbum>>,
    pub rx_albums_from_artist: std::sync::mpsc::Receiver<Vec<SimplifiedAlbum>>,

    pub tx_player_context : std::sync::mpsc::Sender<Option<FullPlayingContext>>,
    pub rx_player_context: std::sync::mpsc::Receiver<Option<FullPlayingContext>>,
}

pub fn init_ui_state() -> UiState {
    return UiState {
        filter_album: String::new(),
        genres_available: HashSet::new(),
        artists_displayed: Vec::new(),
        albums_displayed: Vec::new(),
        show_my_albums: false,
        font_normal : None,
        font_header1 : None,
        font_header2 : None,
        font_light : None,
        dark_theme: true,
    };
}

fn show_spotify_player(ui: &Ui, app: &mut AppContext) {
    ui.text("Now listening to:");
    ui.spacing();

    match app.rx_player_context.try_recv() {
        Ok(playing_context) => {
            match playing_context {
                Some(context ) =>{
                    app.playing_context = Some(FullPlayingContextTimeStamped{ctx : context.clone(), timestamp_systime : SystemTime::now()});
                },
                _ => {}
            }
        }
        Err(_err) => {},
    }


    match &app.playing_context {
       Some(context) => {

 
            match &context.ctx.item {
                Some(track) => {
                    if ui.button(&track.artists[0].name) {
                        // add an artist window
                        // if there isnt one already

                        let new_artist = SimplifiedArtistWithAlbums {
                            data: track.artists[0].clone(),
                            albums: Vec::new(),
                            description: String::new(),
                            load_state : 0,
                        };
                        let mut is_present = false;
                        for alb in &app.ui_state.artists_displayed {
                            if alb.data.id == track.artists[0].id {
                                println!("Artist {} already shown up", track.artists[0].name);
                                is_present = true;
                                break;
                            }
                        }
                        if !is_present {
                            app.ui_state.artists_displayed.push(new_artist);
                        }
                    }

                    ui.same_line();
                    ui.text(&track.name);


                    match context.ctx.progress_ms {
                        Some(progress) => {

                            let progress_dur = Duration::from_millis(progress as u64);
                            let progress_corrected : Duration;
                            if context.ctx.is_playing{
                                progress_corrected = progress_dur + context.timestamp_systime.elapsed().unwrap();
                            }  else {
                                progress_corrected = progress_dur;
                            }
                            let duration = Duration::from_millis(track.duration_ms as u64);
                            ui.text(format!("{}m{}s / {}m{}s",progress_corrected.as_secs()/60, progress_corrected.as_secs()%60,
                                                              duration.as_secs()/60, duration.as_secs()%60,));
                        }
                        _ => {}
                    }
                },
                _ => {}
            };

       },
       _ => {}
    };
}

fn show_library(ui: &Ui, app: &mut AppContext) {
    let _font_title = ui.push_font(app.ui_state.font_header1.unwrap());
    ui.text("Library");
    _font_title.pop();
    ui.spacing();

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

            menu.end();

        }
        if let Some(menu) = ui.begin_menu("Choose random") {
            let mut rng = rand::thread_rng();

            let n: u16 = rng.gen();
            let album = &app.albums_data[(n as usize) % app.albums_data.len()];
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
            menu.end();
        }
        menu_bar.end();
    }

    ui.spacing();
    ui.input_text("", &mut app.ui_state.filter_album).build();

    ui.spacing();

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
            let release_year : String;
            if album.release_date.len() >=4 {
                release_year = album.release_date[..4].to_string();
            } else {
                release_year = "".to_string();
            }
            const GREY: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
            let grey_col = ui.push_style_color(StyleColor::Text,GREY);
            ui.text(format!("{}", release_year));
            grey_col.pop();

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
            let grey_col = ui.push_style_color(StyleColor::Text,GREY);
            if ui.button(format!("{}", &album.artists[0].name)) {
                // add an artist window
                app.ui_state
                .artists_displayed
                .push(SimplifiedArtistWithAlbums {
                        data: album.artists[0].clone(),
                        albums: Vec::new(),
                        description: String::new(),
                        load_state : 0,
                    })
            }
            grey_col.pop();
            ui.separator();
        }
    }
}

fn show_album(ui: &Ui, app: &mut AppContext, key: usize, key_remove: &mut usize) {

    let _font_title = ui.push_font(app.ui_state.font_header1.unwrap());
    ui.text(app.ui_state.albums_displayed[key].data.name.clone());
    _font_title.pop();
    ui.spacing();
    
    if app.ui_state.albums_displayed[key].tracks.len() == 0 {

        let (easy_api_thread, tx_thread) = (Arc::clone(&app.easy_api), app.tx_album_tracks.clone());
        let id_album = app.ui_state.albums_displayed[key].data.id.clone();
        thread::spawn(move || {
            match easy_api_thread
                .lock()
                .unwrap()
                .get_tracks_from_album(&id_album.clone())
             {
                Ok(tracks) => {
                    tx_thread.send(tracks.clone()).unwrap();
                }
                _ => {}
            };
        });
    }
    match app.rx_album_tracks.try_recv() {
        Ok(track_results) => {
            app.ui_state.albums_displayed[key].tracks = track_results.clone();
        }
        Err(_err) => {},
    }


    for track in &app.ui_state.albums_displayed[key].tracks {
        const GREY: [f32; 4] = [0.5, 0.5, 0.5, 1.0];

        let grey_col = ui.push_style_color(StyleColor::Text,GREY);
        ui.text(format!("{}", track.track_number));
        grey_col.pop();
        ui.same_line_with_pos(50.0);
        let display_name : String;
        if track.name.len() >= 34 {
            display_name = track.name[..25].to_string() + &"...".to_string();
        } else {
            display_name = track.name.to_string();
        }
        if ui.button(format!("{}", display_name)) {
            app.easy_api
                .lock()
                .unwrap()
                .play_track(track, Some(&app.ui_state.albums_displayed[key].data))
                .unwrap();
                /*
            app.playing_context.current_artist = Mutex::new(Some(
                app.ui_state.albums_displayed[key].data.artists[0].clone(),
            ));
            app.playing_context.current_track = Mutex::new(Some(track.clone()));
            app.playing_context.current_album =
                Mutex::new(Some(app.ui_state.albums_displayed[key].data.clone()));
                */
        }
        ui.same_line_with_pos(300.0);
        let duration_ms = Duration::from_millis(track.duration_ms.into());
        let seconds = duration_ms.as_secs() % 60;
        let minutes = (duration_ms.as_secs() / 60) % 60;
        let grey_col = ui.push_style_color(StyleColor::Text,GREY);
        ui.text(format!("{}:{}",minutes, seconds));
        grey_col.pop();
        ui.separator();
    }
    ui.spacing();
    if ui.button(format!("CLOSE")) {
        // Remove itself from album_displayed
        *key_remove = key + 1;
    }
}

fn show_artist(ui: &Ui, app: &mut AppContext, key: usize, key_remove: &mut usize) {
    if app.ui_state.artists_displayed[key].load_state == 0 {
        app.ui_state.artists_displayed[key].load_state = 1;
        let (easy_api_thread, tx_thread) = (Arc::clone(&app.easy_api), app.tx_description.clone());
        let name_artist = app.ui_state.artists_displayed[key].data.name.clone();
        thread::spawn(move || {
            match easy_api_thread
            .lock()
            .unwrap()
            .get_wiki_description(name_artist.clone())
            {
                Ok(description) => {
                    tx_thread.send(description.clone()).unwrap();
                }
                _ => {}
            };
        });

        let (easy_api_thread2, tx_thread2) = (Arc::clone(&app.easy_api), app.tx_albums_from_artist.clone());
        let id_artist = app.ui_state.artists_displayed[key].data.id.clone();
        thread::spawn(move || {
            println!("Getting albums from artist id {}", id_artist);
            match easy_api_thread2
                .lock()
                .unwrap()
                .get_albums_from_artist(&id_artist)
            {
                Ok(albums) => {
                    tx_thread2.send(albums.clone()).unwrap();
                }
                _ => {}
            }
        });
    }

    if app.ui_state.artists_displayed[key].load_state < 3 // 2 item to load
    {
    match app.rx_description.try_recv() {
        Ok(description) => {
                app.ui_state.artists_displayed[key].description = description.clone();
                app.ui_state.artists_displayed[key].load_state += 1;
            }
            Err(_err) => {},
        }
        match app.rx_albums_from_artist.try_recv() {
            Ok(albums_results) => {
                app.ui_state.artists_displayed[key].albums = albums_results;
                app.ui_state.artists_displayed[key].load_state += 1;
            }
            Err(_err) => {},
        }
    }


    let _font_title = ui.push_font(app.ui_state.font_header1.unwrap());
    ui.text(&app.ui_state.artists_displayed[key].data.name);
    _font_title.pop();
    ui.spacing();

    if app.ui_state.artists_displayed[key].description.len() > 10 { // filter the "null" and ""
        ui.push_text_wrap_pos();
        ui.text(format!(
            "{}",
            app.ui_state.artists_displayed[key].description
        ));
    }
    for genre in &app.ui_state.artists_displayed[key].data.genres {
        ui.text(format!("{}", genre[0]));
    }
    let _font_title = ui.push_font(app.ui_state.font_header2.unwrap());
    ui.text("Albums");
    _font_title.pop();
    ui.spacing();
    for album in &app.ui_state.artists_displayed[key].albums {

        let release_year : String;
        if album.release_date.len() >=4 {
            release_year = album.release_date[..4].to_string();
        } else {
            release_year = "".to_string();
        }
        const GREY: [f32; 4] = [0.5, 0.5, 0.5, 1.0];

        let grey_col = ui.push_style_color(StyleColor::Text,GREY);
        ui.text(format!("{}", release_year));
        grey_col.pop();
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
        ui.separator();
    }

    ui.spacing();
    if ui.button(format!("CLOSE")) {
        // Remove itself from artists_displayed
        *key_remove = key + 1;
    }
}

pub fn main_loop(ui: &mut Ui<'_>, app: &mut AppContext) {
 
    const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    const TRANSPARENT: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
    const LIGHT_GREY: [f32; 4] = [0.8, 0.8, 0.8, 1.0];
    const SUPER_LIGHT_GREY: [f32; 4] = [0.9, 0.9, 0.9, 1.0];
    /*if app.ui_state.dark_theme {
        let _main_bg = ui.push_style_color(StyleColor::WindowBg,BLACK);
        let _title_bg = ui.push_style_color(StyleColor::TitleBg,BLACK);
        let _title_bg_act = ui.push_style_color(StyleColor::TitleBgActive,LIGHT_GREY);
        let _title_bg_coll = ui.push_style_color(StyleColor::TitleBgCollapsed,LIGHT_GREY);
        let _frame_bg = ui.push_style_color(StyleColor::FrameBg,SUPER_LIGHT_GREY);
        let _frame_bg_act = ui.push_style_color(StyleColor::FrameBgActive,LIGHT_GREY);
        let _frame_bg_hov = ui.push_style_color(StyleColor::FrameBgHovered,LIGHT_GREY);
        let _menu_bar_bg = ui.push_style_color(StyleColor::MenuBarBg,BLACK);
        let _popup_bg = ui.push_style_color(StyleColor::PopupBg,SUPER_LIGHT_GREY);
        let _text_color = ui.push_style_color(StyleColor::Text,WHITE);
        let _button_transparent = ui.push_style_color(StyleColor::Button,TRANSPARENT);
        let _button_hovered = ui.push_style_color(StyleColor::ButtonHovered,LIGHT_GREY);
    } else {*/
        let _main_bg = ui.push_style_color(StyleColor::WindowBg,WHITE);
        let _title_bg = ui.push_style_color(StyleColor::TitleBg,WHITE);
        let _title_bg_act = ui.push_style_color(StyleColor::TitleBgActive,LIGHT_GREY);
        let _title_bg_coll = ui.push_style_color(StyleColor::TitleBgCollapsed,LIGHT_GREY);
        let _frame_bg = ui.push_style_color(StyleColor::FrameBg,SUPER_LIGHT_GREY);
        let _frame_bg_act = ui.push_style_color(StyleColor::FrameBgActive,LIGHT_GREY);
        let _frame_bg_hov = ui.push_style_color(StyleColor::FrameBgHovered,LIGHT_GREY);
        let _menu_bar_bg = ui.push_style_color(StyleColor::MenuBarBg,WHITE);
        let _popup_bg = ui.push_style_color(StyleColor::PopupBg,SUPER_LIGHT_GREY);
        let _text_color = ui.push_style_color(StyleColor::Text,BLACK);
        let _button_transparent = ui.push_style_color(StyleColor::Button,TRANSPARENT);
        let _button_hovered = ui.push_style_color(StyleColor::ButtonHovered,LIGHT_GREY);
    //}
    let _rounding = ui.push_style_var(StyleVar::WindowRounding(10.0));
    let _frame_rounding = ui.push_style_var(StyleVar::FrameRounding(5.0));
    let _win_padding = ui.push_style_var(StyleVar::WindowPadding([20.0,20.0]));
    let _font_normal = ui.push_font(app.ui_state.font_normal.unwrap());


    if let Some(menu_bar) = ui.begin_main_menu_bar() {
        if let Some(menu) = ui.begin_menu("Window") {
            //MenuItem::new("Undo").shortcut("CTRL+Z").build(ui);
            ui.checkbox("Show my albums", &mut app.ui_state.show_my_albums);
            menu.end();
        }
        if let Some(menu) = ui.begin_menu("Theme") {
            //MenuItem::new("Undo").shortcut("CTRL+Z").build(ui);
            ui.checkbox("Dark theme", &mut app.ui_state.dark_theme);
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
        .size([400.0, 500.0], Condition::FirstUseEver)
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
    _font_normal.pop()

}
