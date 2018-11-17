/*  
    @Author Julien LE THENO
    @mod EasyAPI : handles the whole Spotify API bindings,
    and the API rights
*/
extern crate serde_json;

mod files;
mod command;

use self::command::Command;
use self::serde_json::{Value};

pub struct EasyAPI {
    command: Command
}

impl EasyAPI {
    pub fn construct() -> EasyAPI {
        let command = Command::construct();
        return EasyAPI {command};
    }
    pub fn search_and_play_first(&mut self, _type :&str, _search :&str) {
        let mut result = String::new();
        self.command.search(_search, _type,&mut result);
        let v: Value = serde_json::from_str(result.as_str()).unwrap();
        // work for playlist, we should verify the JSON out for other types to get the right thing
        result =   v["playlists"]["items"][0]["id"].to_string(); // just getting the first result here
        result = result[1..].to_string(); // removing last '"'
        result.pop(); // removing first '"'
        //println!("{}",result); 

        self.command.play(result.as_str(), _type)


    }
    pub fn refresh(&mut self) {
        //println!("refreshing");
        let mut refresh_token = String::new();
        let mut base_64_secret = String::new();
        files::load_keys(&mut refresh_token,&mut base_64_secret);
        self.command.refresh(base_64_secret.as_str(), refresh_token.as_str());
        //println!("refreshing done");

    }
}
