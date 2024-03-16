extern crate failure;
extern crate spotify_api;
extern crate text_io;

use std::convert::TryFrom;
#[allow(dead_code)]
mod token_retrieval;
mod support;

use spotify_api::EasyAPI;
use imgui::*;
use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

mod interface;

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
        // Get number of albums from API and if different, download missing albums
        let mut count_result = 0;
        match easy_api_thread.lock().unwrap().get_my_albums_count() {
            Ok(count) =>  {count_result = count}, 
            Err(_e)  =>  {} 
        }

        let mut redownload : bool  = false;

        if albums_data_library.len() as u32 != count_result
        {
            println!("Different size for album library local {} and saved {}", albums_data_library.len(), count_result);
            // Delete local albums
            albums_data_library.clear();
            redownload = true
        }
        else if albums_data_library.is_empty() {
            println!("Empty local library ");
            redownload = true
        }

        if redownload {

            println!(".. Redownloading entire album db");
            let page_size: u16 = 50;
            // Rounding up integer
            let number_of_requests = u16::try_from(count_result / u32::from(page_size)).unwrap() +
                                          u16::from((count_result % u32::from(page_size)) > 0);

            for i in 0..number_of_requests {
                let mut albums_data_chunk = Vec::new();
                // Display percentage for loaded albums
                let percent_loaded : f32 = (i as f32 * page_size as f32 / count_result as f32) * 100 as f32;
                println!("Loading albums {}% ", percent_loaded );
                easy_api_thread.lock().unwrap().get_my_albums_chunk(i * page_size, page_size, &mut albums_data_chunk).unwrap();

                tx_thread.send(albums_data_chunk.clone()).unwrap();
                albums_data_library.extend(albums_data_chunk.clone());
            }
            println!("Local library downloaded : saving..");

            easy_api_thread.lock().unwrap().write_library(albums_data_library).unwrap();
            println!("Local library saved !");

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
