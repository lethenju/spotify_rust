/*  
    @Author Julien LE THENO
    @mod EasyAPI : handles the whole Spotify API bindings,
    and the API rights
*/
extern crate failure;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;
extern crate chrono;
mod command;
mod files;
pub mod model;

use self::command::Command;
use self::serde_json::Value;

pub struct EasyAPI {
    command: Command,
}

impl EasyAPI {
    /// Creates a EasyAPI handle
    pub fn new() -> EasyAPI {
        let command = Command::new();
        return EasyAPI { command };
    }
    /// Searches for a playlist and play the first item
    /// found with that name. Works only with playlist now..
    /// TODO extend from playlist mode to any mode..
    pub fn search_and_play_first(
        &mut self,
        type_: &str,
        search: &str,
    ) -> Result<(), std::io::Error> {
        let mut result = String::new();
        let errno = self.command.search(search, type_, &mut result);
        if errno.is_err() {
            return errno;
        }
        let v: Value = serde_json::from_str(result.as_str()).unwrap();
        // work for playlist, we should verify the JSON out for other types to get the right thing
        result = v["playlists"]["items"][0]["id"].to_string(); // just getting the first result here
        result = result[1..].to_string(); // removing last '"'
        result.pop(); // removing first '"'
                      //println!("{}",result);

        self.command.play(result.as_str(), type_, "", "")
    }

    ///  Get all the current user's albums
    pub fn get_my_albums(
        &mut self,
        final_result: &mut Vec<model::album::FullAlbum>,
    ) -> Result<(), std::io::Error> {
        for i in 0..5 {
            // SUPER dirty -> TODO get number of album to know how many chunks to get.
            self.get_my_albums_chunk(i * 50, final_result).unwrap();
        }
        Ok(())
    }
    /// Get 20 albums in the user's library with a given offset
    pub fn get_my_albums_chunk(
        &mut self,
        offset: u16,
        final_result: &mut Vec<model::album::FullAlbum>,
    ) -> Result<(), std::io::Error> {
        let mut result = String::new();
        let errno = self.command.get_my_albums(offset, &mut result);
        if errno.is_err() {
            return errno;
        }
        let v: Value = serde_json::from_str(result.as_str()).unwrap();
        // work for playlist, we should verify the JSON out for other types to get the right thing
        let size = v["items"].as_array().unwrap().len();
        for x in 0..size {
            final_result.push(serde_json::from_str(
                &serde_json::to_string(&v["items"][x]["album"]).unwrap(),
            )?);
        }
        Ok(())
    }

    ///  Get the track names from a given album id
    pub fn get_tracks_from_album(
        &mut self,
        id_album: &str,
    ) -> Result<Vec<model::track::FullTrack>, std::io::Error> {
        let mut result = String::new();
       // let mut final_result = Vec::new();
        match self.command.get_tracks_from_album(id_album, &mut result) {
            Ok(_ok) => {}
            Err(error) => return Err(error),
        }
        let v: Value = serde_json::from_str(result.as_str()).unwrap();
        // work for playlist, we should verify the JSON out for other types to get the right thing
        let size = v["items"].as_array().unwrap().len();
        for x in 0..size {
            /*
            let mut track_name = v["items"][x]["name"].to_string(); // just getting the first result here
            track_name = track_name[1..].to_string(); // removing last '"'
            track_name.pop(); // removing first '"'

            let mut track_id = v["items"][x]["id"].to_string(); // just getting the first result here
            track_id = track_id[1..].to_string(); // removing last '"'
            track_id.pop(); // removing first '"'
            final_result.push(Track {
                name: track_name,
                id: track_id,
                artist: None,
            });
            */
        }

        unimplemented!();
    }

    /// Searches for playlists with the name "search" in it.
    /// Stores the results in a reference to a vector of Playlist on "final_result"
    pub fn search_playlist(&mut self, search: &str) -> Result<Vec<model::playlist::FullPlaylist>, std::io::Error> {
        let mut result = String::new();
        //let mut final_result = Vec::new();
        match self.command.search(search, "playlist", &mut result) {
            Ok(_ok) => {}
            Err(error) => return Err(error),
        }
        let v: Value = serde_json::from_str(result.as_str()).unwrap();
        let size = v["playlists"]["items"].as_array().unwrap().len();
        for x in 0..size { /*
            let mut playlist_name = v["playlists"]["items"][x]["name"].to_string();
            playlist_name = playlist_name[1..].to_string(); // removing last '"'
            playlist_name.pop(); // removing first '"'

            let mut playlist_id = v["playlists"]["items"][x]["id"].to_string();
            playlist_id = playlist_id[1..].to_string(); // removing last '"'
            playlist_id.pop(); // removing first '"'

            final_result.push(Playlist {
                name: playlist_name,
                id: playlist_id,
                tracks: None, // TODO add tracks
            })*/
        }
        unimplemented!();
    }

    /// Searches for albums with the name "search" in it.
    /// Stores the results in a reference to a vector of Albums on "final_result"
    pub fn search_album(&mut self, search: &str) -> Result<Vec<model::album::FullAlbum>, std::io::Error> {
        let mut result = String::new();
        //let mut final_result = Vec::new();

        match self.command.search(search, "album", &mut result) {
            Ok(_ok) => {}
            Err(error) => return Err(error),
        }
        let v: Value = serde_json::from_str(result.as_str()).unwrap();
        let size = v["albums"]["items"].as_array().unwrap().len();
        for x in 0..size { /*
            let mut album_name = v["albums"]["items"][x]["name"].to_string();
            album_name = album_name[1..].to_string(); // removing last '"'
            album_name.pop(); // removing first '"'

            let mut album_id = v["albums"]["items"][x]["id"].to_string();
            album_id = album_id[1..].to_string(); // removing last '"'
            album_id.pop(); // removing first '"'

            final_result.push(Album {
                name: album_name,
                id: album_id,
                tracks: None,
            });*/
        }
        unimplemented!();
    }

    /// Searches for tracks with the name "search" in it.
    /// Stores the results in a reference to a vector of Tracks on "final_result"
    pub fn search_track(&mut self, search: &str) -> Result<Vec<model::track::FullTrack>, std::io::Error> {
        let mut result = String::new();
        //let mut final_result = Vec::new();

        match self.command.search(search, "track", &mut result) {
            Ok(_ok) => {}
            Err(error) => return Err(error),
        }
        let v: Value = serde_json::from_str(result.as_str()).unwrap();
        let size = v["tracks"]["items"].as_array().unwrap().len();
        for x in 0..size {/*
            let mut track_name = v["tracks"]["items"][x]["name"].to_string();
            track_name = track_name[1..].to_string(); // removing last '"'
            track_name.pop(); // removing first '"'

            let mut track_id = v["tracks"]["item/][x]["id"].to_string();
            track_id = track_id[1..].to_string(); // removing last '"'
            track_id.pop(); // removing first '"'

            final_result.push(Track {
                name: track_name,
                id: track_id,
                artist: None,
            });*/
        }

        unimplemented!();
    }

    /// Searches for artists with the name "search" in it.
    /// Stores the results in a reference to a vector of Artists on "final_result"
    pub fn search_artist(&mut self, search: &str) -> Result<Vec<model::artist::FullArtist>, std::io::Error> {
        let mut result = String::new();
        //let mut final_result = Vec::new();
        match self.command.search(search, "artist", &mut result) {
            Ok(_ok) => {}
            Err(error) => return Err(error),
        }
        let v: Value = serde_json::from_str(result.as_str()).unwrap();
        let size = v["artists"]["items"].as_array().unwrap().len();
        for x in 0..size {
            /*
            let mut artist_name = v["artists"]["items"][x]["name"].to_string();
            artist_name = artist_name[1..].to_string(); // removing last '"'
            artist_name.pop(); // removing first '"'

            let mut artist_id = v["artists"]["items"][x]["id"].to_string();
            artist_id = artist_id[1..].to_string(); // removing last '"'
            artist_id.pop(); // removing first '"'

            final_result.push(Artist {
                name: artist_name,
                id: artist_id,
                albums: None, // TODO add albums
            });*/
        }

        unimplemented!();
    }

    /// Pauses the playback.
    /// Not implemented yet
    pub fn pause(&mut self) -> Result<(), std::io::Error> {
        unimplemented!();
    }
    /// Returns the next song in the active context
    /// Not implemented yet
    pub fn get_next(&mut self) -> Result<model::track::FullTrack, std::io::Error> {
        unimplemented!();
    }
    /// Returns the previous track
    /// Not implemented yet
    pub fn get_previous(&mut self) -> Result<model::track::FullTrack, std::io::Error> {
        unimplemented!();
    }

    /// Gets the currently playing artist on the final_result argument
    /// final_result setted to "" if no track is playing
    pub fn get_currently_playing_artist(&mut self) -> Result<Option<model::artist::SimplifiedArtist>, std::io::Error> {
        let mut result = String::new();
        let errno = self.command.get_currently_playing(&mut result);
        match errno {
            Err(error) => return Err(error),
            _ => {}
        }

        if result.len() != 0 {
            /*
            let v: Value = serde_json::from_str(result.as_str()).unwrap();
            let mut artist_name = v["item"]["artists"][0]["name"].to_string();
            artist_name = artist_name[1..].to_string(); // removing last '"'
            artist_name.pop(); // removing first '"'
            let mut artist_id = v["item"]["artists"][0]["id"].to_string();
            artist_id = artist_id[1..].to_string(); // removing last '"'
            artist_id.pop(); // removing first '"'

            return Ok(Some(Artist {
                name: artist_name,
                id: artist_id,
                albums: None, // TODO Add artist
            }));*/
        }
        Ok(None)
    }

    /// Gets the currently playing track on the final_result argument
    /// final_result setted to "" if no track is playing
    pub fn get_currently_playing_track(&mut self) -> Result<Option<model::track::FullTrack>, std::io::Error> {
        let mut result = String::new();
        let errno = self.command.get_currently_playing(&mut result);
        match errno {
            Err(error) => return Err(error),
            _ => {}
        }
        if result.len() != 0 {
            /*let v: Value = serde_json::from_str(result.as_str()).unwrap();
            let mut track_name = v["item"]["name"].to_string();
            track_name = track_name[1..].to_string(); // removing last '"'
            track_name.pop(); // removing first '"'
            let mut track_id = v["item"]["name"].to_string();
            track_id = track_id[1..].to_string(); // removing last '"'
            track_id.pop(); // removing first '"'
            return Ok(Some(Track {
                name: track_name,
                id: track_id,
                artist: None, // TODO Add artist
            })) */
        }
        Ok(None)
    }
    /// Plays a track in a context ( for now just Album..)
    pub fn play_track(
        &mut self,
        track: &model::track::FullTrack,
        context: Option<&model::album::FullAlbum>,
    ) -> Result<(), std::io::Error> {
        let error = {
            match context {
                Some(context) => {
                    self.command
                        .play(track.id.as_str(), "track", context.id.as_str(), "album")
                }
                None => self.command.play(track.id.as_str(), "track", "", ""),
            }
        };
        match error {
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    println!("No devices connected for the playback!")
                }
            }
            _ => return error,
        }

        Ok(())
    }

    /// Refreshes the access token by requesting a new one
    pub fn refresh(&mut self) -> Result<(), std::io::Error> {
        let mut refresh_token = String::new();
        let mut base_64_secret = String::new();
        match files::load_keys(&mut refresh_token, &mut base_64_secret) {
            Ok(()) => {}
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "No file :(",
                ));
            }
        }
        return self
            .command
            .refresh(base_64_secret.as_str(), refresh_token.as_str());
    }

    pub fn retrieve_refresh_token(
        &mut self,
        base_64_secret: String,
        access_token: String,
    ) -> Result<String, std::io::Error> {
        let refresh_token = self
            .command
            .retrieve_refresh_token(base_64_secret.as_str(), access_token.as_str());
        Ok(refresh_token.unwrap())
    }
}
