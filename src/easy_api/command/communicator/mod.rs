/*  
    @Author Julien LE THENO
    @mod Communicator : handles the communication to the Spotify API 
    using Curl. 
*/
extern crate curl;
extern crate serde_json;

use std::io::Read;
use self::serde_json::{Value};
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

    pub fn perform(&mut self, url :&str, _query :&str, _body :&str, _list_headers : List,
        _type:&str, result :&mut String) {

    }
    /**
    * Generate another access token
    */
    pub fn refresh(&mut self, _base64_secret: &str, _refresh_token : &str, result : &mut String) {
        println!("Refreshing access token");
        let mut data = Vec::new();
        {
            self.easy_handle.post(true).unwrap();
            self.easy_handle.post_fields_copy(&format!("{}{}","grant_type=refresh_token&refresh_token=",_refresh_token).as_bytes()).unwrap();
        
        
            self.easy_handle.url("https://accounts.spotify.com/api/token").unwrap();
            let mut list = List::new();
            let _request2 = format!("{}{}", "Authorization: Basic ", _base64_secret);    
            list.append(&_request2).unwrap();
            self.easy_handle.http_headers(list).unwrap();
            let mut transfer = self.easy_handle.transfer();
            transfer.write_function(|new_data|{
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            }).unwrap();
            transfer.perform().unwrap();
        }
        let s = str::from_utf8(&data).unwrap();

        let v: Value = serde_json::from_str(s).unwrap();

        *result =   v["access_token"].to_string();
        *result = result[1..].to_string();
        result.pop();
        println!("Ok ! Access token : {}",result.as_str());
    }
    
    pub fn set_access_token(&mut self, access_token : &str) {
        self.access_token = access_token.to_string();
    }
}