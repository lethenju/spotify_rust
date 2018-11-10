/*  
    @Author Julien LE THENO
    @mod Command : handles the command building and a Communicator
    object
*/
mod communicator;
use self::communicator::Communicator;

pub struct Command {
    communicator: Communicator
}

impl Command {
    pub fn construct() -> Command {
        let communicator = Communicator::construct("NULL");
        return Command {communicator};
    }
    pub fn play(&self, _spotify_id :&str) {

    }
    pub fn pause(&self) {

    }
    pub fn next(&self) {

    }
    pub fn search(&self, _spotify_id :&str, result :&mut String) {

    }
    pub fn refresh(&mut self, _base_64_secret :&str, _refresh_token :&str) {
        let mut access_token = String::new();
        self.communicator.refresh(_base_64_secret,_refresh_token);
    }
}   
