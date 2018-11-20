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

    pub fn search(
        &mut self,
        _type: &str,
        _search: &str,
        final_result: &mut Vec<String>,
    ) -> Result<(), failure::Error> {
        let mut result = String::new();
        self.command.search(_search, _type, &mut result);
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
    pub fn get_currently_playing_artist(
        &mut self,
        final_result: &mut String,
    ) -> Result<(), failure::Error> {
        let mut result = String::new();
        self.command.get_currently_playing(&mut result);
        if result.len() == 0 {
            *final_result = "no track".to_string();
        } else {
            let v: Value = serde_json::from_str(result.as_str())
                .unwrap_or(Err(()))
                .unwrap();
            result = v["item"]["artists"][0]["name"].to_string();
            result = result[1..].to_string(); // removing last '"'
            result.pop(); // removing first '"'
            *final_result = result;
        }
        Ok(())
    }

    pub fn get_currently_playing_track(
        &mut self,
        final_result: &mut String,
    ) -> Result<(), failure::Error> {
        let mut result = String::new();
        self.command.get_currently_playing(&mut result);
        if result.len() == 0 {
            *final_result = "no track".to_string();
        } else {
            let v: Value = serde_json::from_str(result.as_str()).unwrap();
            result = v["item"]["name"].to_string();
            result = result[1..].to_string(); // removing last '"'
            result.pop(); // removing first '"'
            *final_result = result;
        }
        Ok(())
    }
    pub fn refresh(&mut self) -> Result<(), failure::Error> {
        let mut refresh_token = String::new();
        let mut base_64_secret = String::new();
        files::load_keys(&mut refresh_token, &mut base_64_secret);
        self.command
            .refresh(base_64_secret.as_str(), refresh_token.as_str());
        Ok(())
    }
}
