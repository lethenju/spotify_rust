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
use spotify_api::model::album::FullAlbum;
use spotify_api::model::album::SimplifiedAlbumWithTracks;
use spotify_api::model::artist::SimplifiedArtist;
use std::ffi::OsStr;

fn main() -> Result<(), failure::Error> {
    //let mut easy_api.lock().unwrap() =
    let easy_api = Arc::new(Mutex::new(EasyAPI::new()));
    match easy_api.lock().unwrap().refresh() {
        Ok(()) => {}
        Err(_err) => {
            token_retrieval::retrieve_tokens(&mut easy_api.lock().unwrap()).unwrap();
            easy_api.lock().unwrap().refresh().unwrap();
        }
    }

    // Total album library
    let mut albums_data : Vec<FullAlbum> = Vec::new();

    let mut filter_album = String::new();
    // Albums actually displayed on the screen
    let mut albums_displayed : Vec<SimplifiedAlbumWithTracks> = Vec::new();
    // Artists actually displayed on the screen
    let mut artists_displayed : Vec<SimplifiedArtist> = Vec::new();

    let (tx, rx) = mpsc::channel();
    let (easy_api_thread, tx_thread) = (Arc::clone(&easy_api),tx.clone());
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
    let mut current_artist = Mutex::new(easy_api.lock().unwrap().get_currently_playing_artist().unwrap());
    let mut current_track = Mutex::new(easy_api.lock().unwrap().get_currently_playing_track().unwrap());
    let current_track_full = Mutex::new(easy_api.lock().unwrap().get_currently_playing_track_full().unwrap());
    let current_track_full_guard = current_track_full.lock().unwrap();
    let mut current_album = match &*current_track_full_guard {
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
            let current_artist_guard = current_artist.lock().unwrap();
            match &*current_artist_guard {
                Some(artist) => {
                    if ui.button(&artist.name){
                        // add an artist window
                    }
                    ui.same_line();
                }
                None => {}
            };
            let current_track_guard = current_track.lock().unwrap();
            match &*current_track_guard {
                Some(track) => {
                    ui.text(&track.name);
                    let current_album_local = current_album.lock().unwrap();
                    match &*current_album_local{
                        Some(album) => {
                            if ui.button(&album.name) {
                                println!("Need to load album {} ", album.name);
                                let mut new_album = SimplifiedAlbumWithTracks{data : album.clone(),tracks : Vec::new()};
                                let mut is_present = false;
                                for alb in &albums_displayed  {
                                    if alb.data.id == album.id {
                                        println!("Album {} already shown up", album.name);
                                        is_present = true;
                                        break;
                                    }
                                }
                                if !is_present {
                                    albums_displayed.push(new_album);
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
        .build(ui, || {
            let _roboto = ui.push_font(roboto);
            if let Some(menu_bar) = ui.begin_menu_bar() {
                if let Some(menu) = ui.begin_menu("Menu") {
                    MenuItem::new("Filter").build(ui);
                    if (ui.button("Filtrer par date de parution")){
                        // C'est parti
                    }

                    menu.end();
                }
            }
            ui.input_text("", &mut filter_album).build();
            for album in &albums_data {
                let mut show = true;
                if !filter_album.is_empty(){
                    show = false;
                    let my_os_filter = OsStr::new(&filter_album);
                    if album.name.to_string().contains(my_os_filter.to_str().unwrap()) ||
                       album.artists[0].name.to_string().contains(my_os_filter.to_str().unwrap()) {
                        show = true;
                    }
                }

                if show {
                    if ui.button(format!("{} - {}", album.artists[0].name, album.name)){
                        println!("Need to load album {} ", album.name);
                        let mut new_album = SimplifiedAlbumWithTracks{data : album.clone().to_simplified(),tracks : Vec::new()};
                        let mut is_present = false;
                        for alb in &albums_displayed  {
                            if alb.data.id == album.id {
                                is_present = true;
                                break;
                            }
                        }
                        if !is_present {
                            albums_displayed.push(new_album);
                        }
                    }
                }
            }

            match rx.try_recv() {
            Ok(albums) => { albums_data.extend(albums.clone())}
            Err(_err) => {()}
            }

            _roboto.pop();
        });
        let mut key_remove = 0;
        for key in 0..albums_displayed.len() {
            
            Window::new(format!("{} - {}", albums_displayed[key].data.artists[0].name, albums_displayed[key].data.name))
            .size([500.0, 500.0], Condition::FirstUseEver)
            .build(ui, || {

                let _roboto = ui.push_font(roboto);
                if albums_displayed[key].tracks.len() == 0
                {
                    let track_results = easy_api.lock().unwrap().get_tracks_from_album(&albums_displayed[key].data.id).unwrap();
                    albums_displayed[key].tracks = track_results.clone();
                }
                for track in &albums_displayed[key].tracks {
                    if ui.button(format!("- {}",track.name)) {
                        easy_api.lock().unwrap().play_track(track, Some(&albums_displayed[key].data)).unwrap();
                        current_artist = Mutex::new(Some(albums_displayed[key].data.artists[0].clone()));
                        current_track = Mutex::new(Some(track.clone()));
                        current_album = Mutex::new(Some(albums_displayed[key].data.clone()));

                    }
                }
                ui.spacing();
                if ui.button(format!("CLOSE")){
                    // Remove itself from album_displayed
                    key_remove = key+1;
                }

                _roboto.pop();
            });
        }
        if key_remove > 0
        {
            albums_displayed.remove(key_remove-1);
        }

    });
    Ok(())
}
