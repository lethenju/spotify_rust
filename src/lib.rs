/*  
    @Author Julien LE THENO
    @mod EasyAPI : handles the whole Spotify API bindings,
    and the API rights
*/
extern crate failure;
extern crate serde_json;

mod command;
mod files;

use self::command::Command;
use self::serde_json::Value;

pub struct EasyAPI {
    command: Command,
}

impl EasyAPI {
    pub fn construct() -> EasyAPI {
        let command = Command::construct();
        return EasyAPI { command };
    }
    /// Searches for a playlist and play the first item
    /// found with that name. Works only with playlist now..
    /// TODO extend from playlist mode to any mode..
    pub fn search_and_play_first(
        &mut self,
        _type: &str,
        _search: &str,
    ) -> Result<(), failure::Error> {
        let mut result = String::new();
        self.command.search(_search, _type, &mut result);
        let v: Value = serde_json::from_str(result.as_str()).unwrap();
        // work for playlist, we should verify the JSON out for other types to get the right thing
        result = v["playlists"]["items"][0]["id"].to_string(); // just getting the first result here
        result = result[1..].to_string(); // removing last '"'
        result.pop(); // removing first '"'
                      //println!("{}",result);

        self.command.play(result.as_str(), _type);
        Ok(())
    }
    /// Searches for playlist with the "search" parameter str.
    ///  results are added to the final_result String vector in
    ///  parameter.
    pub fn search_playlist(
        &mut self,
        _search: &str,
        final_result: &mut Vec<String>,
    ) -> Result<(), failure::Error> {
        let mut result = String::new();
        self.command.search(_search, "playlist", &mut result);
        let v: Value = serde_json::from_str(result.as_str()).unwrap();
        // work for playlist, we should verify the JSON out for other types to get the right thing
        let size = v["playlists"]["items"].as_array().unwrap().len();
        for x in 0..size {
            result = v["playlists"]["items"][x]["name"].to_string(); // just getting the first result here
            result = result[1..].to_string(); // removing last '"'
            result.pop(); // removing first '"'
            final_result.push(result);
        }
        Ok(())
    }

    ///  Get the current users album
    pub fn get_my_albums(&mut self, final_result: &mut Vec<String>) -> Result<(), failure::Error> {
        let mut result = String::new();
        self.command.get_my_albums(&mut result);
        let v: Value = serde_json::from_str(result.as_str()).unwrap();
        // work for playlist, we should verify the JSON out for other types to get the right thing
        let size = v["items"].as_array().unwrap().len();
        for x in 0..size {
            result = v["items"][x]["album"]["name"].to_string(); // just getting the first result here
            result = result[1..].to_string(); // removing last '"'
            result.pop(); // removing first '"'
            final_result.push(result);
        }
        Ok(())
    }
    ///  Get the current users album ids
    pub fn get_my_albums_ids(
        &mut self,
        final_result: &mut Vec<String>,
    ) -> Result<(), failure::Error> {
        let mut result = String::new();
        self.command.get_my_albums(&mut result);
        let v: Value = serde_json::from_str(result.as_str()).unwrap();
        // work for playlist, we should verify the JSON out for other types to get the right thing
        let size = v["items"].as_array().unwrap().len();
        for x in 0..size {
            result = v["items"][x]["album"]["id"].to_string(); // just getting the first result here
            result = result[1..].to_string(); // removing last '"'
            result.pop(); // removing first '"'
            final_result.push(result);
        }
        Ok(())
    }
    ///  Get the track names from a given album id
    pub fn get_tracks_from_album(
        &mut self,
        id: &str,
        final_result: &mut Vec<String>,
    ) -> Result<(), failure::Error> {
        let mut result = String::new();
        self.command.get_tracks_from_album(id, &mut result);
        let v: Value = serde_json::from_str(result.as_str()).unwrap();
        // work for playlist, we should verify the JSON out for other types to get the right thing
        let size = v["items"].as_array().unwrap().len();
        for x in 0..size {
            result = v["items"][x]["name"].to_string(); // just getting the first result here
            result = result[1..].to_string(); // removing last '"'
            result.pop(); // removing first '"'
            final_result.push(result);
        }
        Ok(())
    }
    ///  Get the tracks id from a given  album id
    pub fn get_tracks_id_from_album(
        &mut self,
        id: &str,
        final_result: &mut Vec<String>,
    ) -> Result<(), failure::Error> {
        let mut result = String::new();
        self.command.get_tracks_from_album(id, &mut result);
        let v: Value = serde_json::from_str(result.as_str()).unwrap();
        // work for playlist, we should verify the JSON out for other types to get the right thing
        let size = v["items"].as_array().unwrap().len();
        for x in 0..size {
            result = v["items"][x]["id"].to_string(); // just getting the first result here
            result = result[1..].to_string(); // removing last '"'
            result.pop(); // removing first '"'
            final_result.push(result);
        }
        Ok(())
    }

    /// TODO
    /// Not implemented yet
    pub fn search_album(
        &mut self,
        _search: &str,
        final_result: &mut Vec<String>,
    ) -> Result<(), failure::Error> {
        Ok(())
    }
    /// TODO
    /// Not implemented yet
    pub fn search_track(
        &mut self,
        _search: &str,
        final_result: &mut Vec<String>,
    ) -> Result<(), failure::Error> {
        Ok(())
    }
    /// TODO
    /// Not implemented yet
    pub fn search_artist(
        &mut self,
        _search: &str,
        final_result: &mut Vec<String>,
    ) -> Result<(), failure::Error> {
        Ok(())
    }

    /// Gets the currently playing artist on the final_result argument
    /// final_result setted to "" if no track is playing
    pub fn get_currently_playing_artist(
        &mut self,
        final_result: &mut String,
    ) -> Result<(), failure::Error> {
        let mut result = String::new();
        self.command.get_currently_playing(&mut result);
        if result.len() == 0 {
            *final_result = "".to_string();
        } else {
            let v: Value = serde_json::from_str(result.as_str()).unwrap();
            result = v["item"]["artists"][0]["name"].to_string();
            result = result[1..].to_string(); // removing last '"'
            result.pop(); // removing first '"'
            *final_result = result;
        }
        Ok(())
    }

    /// Gets the currently playing track on the final_result argument
    /// final_result setted to "" if no track is playing
    pub fn get_currently_playing_track(
        &mut self,
        final_result: &mut String,
    ) -> Result<(), failure::Error> {
        let mut result = String::new();
        self.command.get_currently_playing(&mut result);
        if result.len() == 0 {
            *final_result = "".to_string();
        } else {
            let v: Value = serde_json::from_str(result.as_str()).unwrap();
            result = v["item"]["name"].to_string();
            result = result[1..].to_string(); // removing last '"'
            result.pop(); // removing first '"'
            *final_result = result;
        }
        Ok(())
    }
    /// Plays a track given its ID
    pub fn play_track_from_id(&mut self, id: &str) -> Result<(), failure::Error> {
        self.command.play(id, "track");
        Ok(())
    }

    /// Refreshes the access token by requesting a new one
    pub fn refresh(&mut self) -> Result<(), failure::Error> {
        let mut refresh_token = String::new();
        let mut base_64_secret = String::new();
        files::load_keys(&mut refresh_token, &mut base_64_secret);
        self.command
            .refresh(base_64_secret.as_str(), refresh_token.as_str());
        Ok(())
    }
}
