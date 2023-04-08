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

mod interface;
mod data_manager;

use std::time::Duration;

use crate::interface::AppContext;
use crate::interface::main_loop;
use crate::interface::init_ui_state;



fn main() -> Result<(), failure::Error> {
    
    // Creating the data channels between UI and data gathering threads
    let (tx, rx) = mpsc::channel();
    let (tx_description, rx_description) = mpsc::channel();
    let (tx_album_tracks, rx_album_tracks) = mpsc::channel();
    let (tx_albums_from_artist, rx_albums_from_artist) = mpsc::channel();
    let (tx_player_context, rx_player_context) = mpsc::channel();

    // Creating the app context
    let mut app = AppContext{
        ui_state : init_ui_state(),
        playing_context : None,
        easy_api : Arc::new(Mutex::new(EasyAPI::new())),
        albums_data : Vec::new(),

        data_store : data_manager::DataStore::new(),

        rx_album_library : rx,
        tx_description : tx_description,
        rx_description : rx_description,

        tx_album_tracks : tx_album_tracks,
        rx_album_tracks : rx_album_tracks,

        tx_albums_from_artist : tx_albums_from_artist,
        rx_albums_from_artist : rx_albums_from_artist,

        tx_player_context : tx_player_context,
        rx_player_context : rx_player_context,
    };

    let my_response = app.easy_api.lock().unwrap().refresh();
    
    match my_response {
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

    let (easy_api_thread, tx_thread) = (Arc::clone(&app.easy_api), app.tx_player_context.clone());
    thread::spawn(move || {
        loop{
            // Check every 5 seconds
            thread::sleep(Duration::from_secs(5));
            match easy_api_thread
            .lock()
            .unwrap()
            .get_playback_state()
            {
                Ok(playing_context) => {
                    tx_thread.send(playing_context.clone()).unwrap();
                }
                _ => {}
            };
        }

    });


    let mut system = support::init(file!());
    app.ui_state.font_normal = Some(system.imgui.fonts().add_font(&[FontSource::TtfData {
        data: include_bytes!("../resources/SF-UI-Display-Regular.ttf"),
        size_pixels: 20.0,//system.font_size,
        config: None,
    }]));
    app.ui_state.font_header1 = Some(system.imgui.fonts().add_font(&[FontSource::TtfData {
        data: include_bytes!("../resources/SF-UI-Display-Black.ttf"),
        size_pixels: 60.0,//system.font_size,
        config: None,
    }]));
    app.ui_state.font_header2 = Some(system.imgui.fonts().add_font(&[FontSource::TtfData {
        data: include_bytes!("../resources/SF-UI-Display-Black.ttf"),
        size_pixels: 40.0,//system.font_size,
        config: None,
    }]));
    system
        .renderer
        .reload_font_texture(&mut system.imgui)
        .expect("Failed to reload fonts");

    system.main_loop(move |_, ui| {
        main_loop(ui, &mut app);

    });
    Ok(())
}
