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
/// Handle to control the whole spotify API.
///
/// # Command
///
/// The command is the only attribute of the structure. It is a private
/// attribute that handles the communication with the Spotify headend
/// in a large sense
///
///
pub struct EasyAPI {
    command: Command,
}

impl EasyAPI {
    /// Creates a EasyAPI handle. The handle has to be mutable, because some
    /// inner state will be modified, as the access token.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut handle = EasyAPI::new();
    /// ```
    ///
    pub fn new() -> EasyAPI {
        let command = Command::new();
        return EasyAPI { command };
    }
}
