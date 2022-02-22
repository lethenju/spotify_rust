extern crate failure;
extern crate spotify_api;
extern crate text_io;

#[allow(dead_code)]
mod token_retrieval;
mod support;

use spotify_api::EasyAPI;
use imgui::*;
use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use spotify_api::model::album::SimplifiedAlbum;
use spotify_api::model::album::FullAlbum;
use spotify_api::model::album::SimplifiedAlbumWithTracks;
use spotify_api::model::artist::SimplifiedArtist;
use spotify_api::model::track::SimplifiedTrack;
use spotify_api::model::track::FullTrack;
use std::ffi::OsStr;

struct UiState{
    filter_album : String,
    artists_displayed : Vec<SimplifiedArtist>,
    albums_displayed : Vec<SimplifiedAlbumWithTracks>,
}
struct PlayingContext{
    current_artist : Mutex<Option<SimplifiedArtist>>,
    current_track : Mutex<Option<SimplifiedTrack>>,
    current_track_full : Mutex<Option<FullTrack>>,
    current_album : Mutex<Option<SimplifiedAlbum>>,
}
struct AppContext{
    ui_state : UiState,
    playing_context : PlayingContext,
    albums_data : Vec<FullAlbum>,
    easy_api : Arc<Mutex<EasyAPI>>,
}


fn main() -> Result<(), failure::Error> {
    //let mut easy_api.lock().unwrap() =
    let mut app = AppContext{
        ui_state : UiState {
            filter_album : String::new(),
            artists_displayed : Vec::new(),
            albums_displayed : Vec::new(),
        },
        playing_context : PlayingContext{
            current_artist : Mutex::new(None::<SimplifiedArtist>),
            current_track : Mutex::new(None::<SimplifiedTrack>),
            current_track_full : Mutex::new(None::<FullTrack>),
            current_album : Mutex::new(None::<SimplifiedAlbum>),
        },
        easy_api : Arc::new(Mutex::new(EasyAPI::new())),
        albums_data : Vec::new(),
    };

    match app.easy_api.lock().unwrap().refresh() {
        Ok(()) => {}
        Err(_err) => {
            token_retrieval::retrieve_tokens(&mut app.easy_api.lock().unwrap()).unwrap();
            app.easy_api.lock().unwrap().refresh().unwrap();
        }
    }

    let (tx, rx) = mpsc::channel();
    let (easy_api_thread, tx_thread) = (Arc::clone(&app.easy_api),tx.clone());
    let _handle = thread::spawn(move || {
        let mut ended = false;
        let mut i = 0;
        while !ended {
            // Todo read local library first and then compare with synced
            let mut albums_data_chunk = Vec::new();
            println!("Loading albums {} ", i);
            easy_api_thread.lock().unwrap().get_my_albums_chunk(i, &mut albums_data_chunk).unwrap();
            if albums_data_chunk.len() <20 {
                ended =  true;
            }
            tx_thread.send(albums_data_chunk).unwrap();
            i+=20;
        }
    });

    let mut system = support::init(file!());
    app.playing_context.current_artist = Mutex::new(app.easy_api.lock().unwrap().get_currently_playing_artist().unwrap());
    app.playing_context.current_track = Mutex::new(app.easy_api.lock().unwrap().get_currently_playing_track().unwrap());
    app.playing_context.current_track_full = Mutex::new(app.easy_api.lock().unwrap().get_currently_playing_track_full().unwrap());
    let current_track_full_guard = app.playing_context.current_track_full.lock().unwrap().clone();
    app.playing_context.current_album = match current_track_full_guard {
        Some(track) => {Mutex::new(Some(track.album.clone()))},
        None => {Mutex::new(None)}
    };
    let roboto = system.imgui.fonts().add_font(&[FontSource::TtfData {
        data: include_bytes!("../resources/Roboto-Regular.ttf"),
        size_pixels: 20.0,//system.font_size,
        config: None,
    }]);
    system
        .renderer
        .reload_font_texture(&mut system.imgui)
        .expect("Failed to reload fonts");

    system.main_loop(move |_, ui| {
        Window::new("Spotify Player")
        .size([300.0, 110.0], Condition::FirstUseEver)
        .build(ui, || {
            let _roboto = ui.push_font(roboto);
            ui.text("Now listening to:");
            ui.spacing();
            let current_artist_guard = app.playing_context.current_artist.lock().unwrap();
            match &*current_artist_guard {
                Some(artist) => {
                    if ui.button(&artist.name){
                        // add an artist window
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
                    match &*current_album_local{
                        Some(album) => {
                            if ui.button(&album.name) {
                                println!("Need to load album {} ", album.name);
                                let new_album = SimplifiedAlbumWithTracks{data : album.clone(),tracks : Vec::new()};
                                let mut is_present = false;
                                for alb in &app.ui_state.albums_displayed  {
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
            _roboto.pop()
        });
        Window::new("Spotify Albums")
        .size([500.0, 800.0], Condition::Appearing)
        .menu_bar(true)
        .build(ui, || {
            if let Some(menu_bar) = ui.begin_menu_bar() {

                if let Some(menu) = ui.begin_menu("Filters") {
                    /*if (ui.button("Filtrer par date de parution")){
                        // C'est parti
                    }*/

                    menu.end();
                }

                if let Some(menu) = ui.begin_menu("Sort by") {

                    if ui.button("Release Date"){
                        // C'est parti
                        app.albums_data.sort_by(|a, b| b.release_date.cmp(&a.release_date));
                    }

                    menu.end();
                }

                menu_bar.end();
            }

            ui.spacing();
            ui.input_text("", &mut app.ui_state.filter_album).build();
            for album in &app.albums_data {
                let mut show = true;
                if !app.ui_state.filter_album.is_empty(){
                    show = false;
                    let my_os_filter = OsStr::new(&app.ui_state.filter_album);
                    if album.name.to_string().contains(my_os_filter.to_str().unwrap()) ||
                       album.artists[0].name.to_string().contains(my_os_filter.to_str().unwrap()) {
                        show = true;
                    }
                }

                if show {
                    if ui.button(format!("{} - {}", album.artists[0].name, album.name)){
                        println!("Need to load album {} ", album.name);
                        let new_album = SimplifiedAlbumWithTracks{data : album.clone().to_simplified(),tracks : Vec::new()};
                        let mut is_present = false;
                        for alb in &app.ui_state.albums_displayed  {
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
            }

            match rx.try_recv() {
            Ok(albums) => { app.albums_data.extend(albums.clone())}
            Err(_err) => {()}
            }
        });
        let mut key_remove = 0;
        for key in 0..app.ui_state.albums_displayed.len() {

            Window::new(format!("{} - {}", app.ui_state.albums_displayed[key].data.artists[0].name, app.ui_state.albums_displayed[key].data.name))
            .size([500.0, 500.0], Condition::FirstUseEver)
            .build(ui, || {


                if app.ui_state.albums_displayed[key].tracks.len() == 0
                {
                    let track_results = app.easy_api.lock().unwrap().get_tracks_from_album(&app.ui_state.albums_displayed[key].data.id).unwrap();
                    app.ui_state.albums_displayed[key].tracks = track_results.clone();
                }
                for track in &app.ui_state.albums_displayed[key].tracks {
                    if ui.button(format!("- {}",track.name)) {
                        app.easy_api.lock().unwrap().play_track(track, Some(&app.ui_state.albums_displayed[key].data)).unwrap();
                        app.playing_context.current_artist = Mutex::new(Some(app.ui_state.albums_displayed[key].data.artists[0].clone()));
                        app.playing_context.current_track = Mutex::new(Some(track.clone()));
                        app.playing_context.current_album = Mutex::new(Some(app.ui_state.albums_displayed[key].data.clone()));

                    }
                }
                ui.spacing();
                if ui.button(format!("CLOSE")){
                    // Remove itself from album_displayed
                    key_remove = key+1;
                }

            });
        }
        if key_remove > 0
        {
            app.ui_state.albums_displayed.remove(key_remove-1);
        }

    });
    Ok(())
}
