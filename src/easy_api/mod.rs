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
    pub fn refresh(&mut self) {
        
        let mut refresh_token = String::new();
        let mut base_64_secret = String::new();
        files::load_keys(&mut refresh_token,&mut base_64_secret);
        self.command.refresh(base_64_secret.as_str(), refresh_token.as_str());
    }
}
