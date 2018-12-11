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
pub mod apis;
mod command;
mod files;
pub mod model;

use self::command::Command;

pub struct EasyAPI {
    command: Command,
}

impl EasyAPI {
    /// Creates a EasyAPI handle
    pub fn new() -> EasyAPI {
        let command = Command::new();
        return EasyAPI { command };
    }

}
