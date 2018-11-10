/*  
    @Author Julien LE THENO
    @mod EasyAPI : handles the whole Spotify API bindings,
    and the API rights
*/
mod files;
mod command;
pub use self::command::Command;

pub struct EasyAPI {
    command: Command
}

impl EasyAPI {
    pub fn construct() -> EasyAPI {
        let command = Command::construct();
        return EasyAPI {command};
    }
    pub fn search_and_play_first(&self, _type :&str, _search :&str) {

    }
    pub fn search_and_choose_on_cli(&self) {
        
    }
    pub fn refresh(&self) {
       // files::load_keys(refresh_token: &mut String, base_64_secret: &mut String);
    }
}
