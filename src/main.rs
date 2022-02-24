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
use spotify_api::model::artist::SimplifiedArtist;
use spotify_api::model::track::SimplifiedTrack;
use spotify_api::model::track::FullTrack;

mod interface;


use crate::interface::PlayingContext;
use crate::interface::AppContext;
use crate::interface::main_loop;
use crate::interface::init_ui_state;


fn main() -> Result<(), failure::Error> {
    //let mut easy_api.lock().unwrap() =

    let (tx, rx) = mpsc::channel();
    let (tx_description, rx_description) = mpsc::channel();

    let mut app = AppContext{
        ui_state : init_ui_state(),
        playing_context : PlayingContext{
            current_artist : Mutex::new(None::<SimplifiedArtist>),
            current_track : Mutex::new(None::<SimplifiedTrack>),
            current_track_full : Mutex::new(None::<FullTrack>),
            current_album : Mutex::new(None::<SimplifiedAlbum>),
        },
        easy_api : Arc::new(Mutex::new(EasyAPI::new())),
        albums_data : Vec::new(),
        rx_album_library : rx,
        tx_description : tx_description,
        rx_description : rx_description,
    };

    match app.easy_api.lock().unwrap().refresh() {
        Ok(()) => {}
        Err(_err) => {
            token_retrieval::retrieve_tokens(&mut app.easy_api.lock().unwrap()).unwrap();
            app.easy_api.lock().unwrap().refresh().unwrap();
        }
    }

    let (easy_api_thread, tx_thread) = (Arc::clone(&app.easy_api),tx.clone());
    let _handle = thread::spawn(move || {

        let mut albums_data_library = Vec::new();
         match easy_api_thread.lock().unwrap().read_library( &mut albums_data_library) {
            Ok(()) => {},
            Err(_err) => {}
        }
        if albums_data_library.is_empty() {
            println!("Empty local library : downloading..");

            let mut ended = false;
            let mut i = 0;
            while !ended {
                let mut albums_data_chunk = Vec::new();
                println!("Loading albums {} ", i);
                easy_api_thread.lock().unwrap().get_my_albums_chunk(i, &mut albums_data_chunk).unwrap();
                if albums_data_chunk.len() <20 {
                    ended =  true;
                }
                tx_thread.send(albums_data_chunk.clone()).unwrap();
                //albums_data_library.extend(albums_data_chunk.clone());
                i+=20;
            }
            println!("Local library downloaded : saving..");

            easy_api_thread.lock().unwrap().write_library(albums_data_library).unwrap();
        } else {
            tx_thread.send(albums_data_library).unwrap();

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
        main_loop(ui, &mut app);

    });
    Ok(())
}
