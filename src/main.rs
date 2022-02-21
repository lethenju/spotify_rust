extern crate failure;
extern crate spotify_api;
extern crate text_io;

#[allow(dead_code)]
mod token_retrieval;
mod support;

use spotify_api::EasyAPI;
use imgui::*;

/// Entry point of the text user interface
fn main() -> Result<(), failure::Error> {
    let mut easy_api = EasyAPI::new();
    match easy_api.refresh() {
        Ok(()) => {}
        Err(_err) => {
            token_retrieval::retrieve_tokens(&mut easy_api).unwrap();
            easy_api.refresh().unwrap();
        }
    }

    let mut albums_data = Vec::new();
    easy_api.get_my_albums_chunk(0, &mut albums_data).unwrap();

    let system = support::init(file!());
    let current_artist_name = match easy_api.get_currently_playing_artist().unwrap() {
        Some(artist) => artist.name,
        None => "".to_string(),
    };
    let current_track_name = match easy_api.get_currently_playing_track().unwrap() {
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
        .size([100.0, 110.0], Condition::FirstUseEver)
        .build(ui, || {

            ui.text("My Albums");
            for (albums_data.)
            ui.text(&current_artist_name);
            ui.text(&current_track_name);
            //ui.button("Pause");
        });

    });
    Ok(())
}
