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
use spotify_api::model::album::SimplifiedAlbumWithTrack;


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

    let mut albums_data = Vec::new();
    let mut albums_displayed = Vec::new();

    easy_api.lock().unwrap().get_my_albums_chunk(0, &mut albums_data).unwrap();

    let (tx, rx) = mpsc::channel();
    let (easy_api_thread, tx_thread) = (Arc::clone(&easy_api),tx.clone());
    let _handle = thread::spawn(move || {
        let mut ended = false;
        let mut i = 0;
        while !ended {
            let mut albums_data_chunk = Vec::new();
            println!("Loading albums {} ", i);
            //let mut data = data.lock().unwrap();
            easy_api_thread.lock().unwrap().get_my_albums_chunk(i, &mut albums_data_chunk).unwrap();
            //albums_data.extend(albums_data_chunk);
            if albums_data_chunk.len() <20 {
                ended =  true;
            }
            tx_thread.send(albums_data_chunk).unwrap();
            i+=20
        }
    });

    let system = support::init(file!());
    let current_artist_name = match easy_api.lock().unwrap().get_currently_playing_artist().unwrap() {
        Some(artist) => artist.name,
        None => "".to_string(),
    };
    let current_track_name = match easy_api.lock().unwrap().get_currently_playing_track().unwrap() {
        Some(track) => track.name,
        None => "".to_string(),
    };
    system.main_loop(move |_, ui| {
        Window::new("Spotify Player")
        .size([300.0, 110.0], Condition::FirstUseEver)
        .build(ui, || {

            ui.text("Now listening to:");
            ui.text(&current_artist_name);
            ui.text(&current_track_name);
            //ui.button("Pause");
        });
        Window::new("Spotify Albums")
        .size([500.0, 800.0], Condition::Appearing)
        .build(ui, || {
            for album in &albums_data {
                if ui.button(format!("{} - {}", album.artists[0].name, album.name)){
                    println!("Need to load album {} ", album.name);
                    albums_displayed.push(SimplifiedAlbumWithTrack{data : album.clone(),tracks : Vec::new()});
                }
            }

            match rx.try_recv() {
            Ok(albums) => { albums_data.extend(albums.clone())}
            Err(_err) => {()}
            }
        });
        let mut key_remove = 0;
        for key in 0..albums_displayed.len() {
            Window::new(format!("{} - {}", albums_displayed[key].data.artists[0].name, albums_displayed[key].data.name))
            .size([500.0, 500.0], Condition::FirstUseEver)
            .build(ui, || {
                if albums_displayed[key].tracks.len() == 0
                {
                    let track_results = easy_api.lock().unwrap().get_tracks_from_album(&albums_displayed[key].data.id).unwrap();
                    albums_displayed[key].tracks = track_results.clone();
                }
                for track in &albums_displayed[key].tracks {
                    if ui.button(format!("- {}",track.name)) {
                        easy_api.lock().unwrap().play_track(track, Some(&albums_displayed[key].data)).unwrap();
                    }
                }
                if ui.button(format!("CLOSE")){
                    // Remove itself from album_displayed
                    key_remove = key+1;
                }
            });
        }
        if key_remove > 0
        {
            albums_displayed.remove(key_remove-1);
        }

    });
    Ok(())
}
