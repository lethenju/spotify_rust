/*  
    @Author Julien LE THENO
    @mod Communicator : handles the communication to the Spotify API 
    using Curl. 
*/
extern crate curl;
use curl::easy::{Easy, List};
use std::str;

pub struct Communicator {
    access_token : String,
    easy_handle : Easy
}

impl Communicator {
    pub fn construct( _access_token : &str) -> Communicator {
        let mut easy_handle = Easy::new(); 
        let mut access_token = _access_token.to_string();
        return Communicator {access_token, easy_handle};
    }

    pub fn perform(&self, url :&str, _query :&str, _body :&str, _list_headers : List,
        _type:&str, result :&mut String) {

    }
    pub fn set_access_token(&self, access_token : &str) {

    }
}