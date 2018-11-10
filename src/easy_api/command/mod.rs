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
        let communicator = Communicator::construct("TODO");
        return Command {communicator};
    }
    pub fn play(&self, _spotify_id :&str) {

    }
    pub fn pause(&self) {

    }
    pub fn next(&self) {

    }
    pub fn search(&self, _spotify_id :&str, result : &mut String) {

    }
}