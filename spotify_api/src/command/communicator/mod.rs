/*  
    @Author Julien LE THENO
    @mod Communicator : handles the communication to the Spotify API 
    using Curl. 
*/
extern crate curl;
extern crate serde_json;

pub use self::curl::easy::{Easy, List};
use self::serde_json::Value;
use std::io::{Error, ErrorKind, Read};
use std::str;

pub struct Communicator {
    access_token: String,
    easy_handle: Easy,
}

impl Communicator {
    pub fn new(_access_token: &str) -> Communicator {
        let easy_handle = Easy::new();
        let access_token = _access_token.to_string();
        return Communicator {
            access_token,
            easy_handle,
        };
    }

    pub fn perform(
        &mut self,
        _url: &str,
        _query: &str,
        _body: &str,
        _list_headers: List,
        _type: &str,
        result: &mut String,
    ) -> Result<(), std::io::Error> {
        let mut data2 = Vec::new();
        {
            let mut data = _body.as_bytes();
            match _type {
                "GET" => {}
                "PUT" => {
                    self.easy_handle.put(true).unwrap();
                    self.easy_handle.post_field_size(data.len() as u64).unwrap();
                }
                "POST" => {
                    self.easy_handle.post(true).unwrap();
                }
                _ => {
                    println!("Type not recognized..");
                    return Err(std::io::Error::new(
                        ErrorKind::NotFound,
                        "type not recognized",
                    ));
                }
            }

            if !_query.is_empty() {
                let _url_final = format!("{}?{}", _url, _query);
                self.easy_handle.url(&_url_final).unwrap();
            } else {
                self.easy_handle.url(&_url).unwrap();
            }

            //let _auth = format!("{}{}", "Authorization: Bearer ", _access_token);
            //list_headers.append(&_auth).unwrap();
            self.easy_handle.http_headers(_list_headers).unwrap();

            let mut transfer = self.easy_handle.transfer();
            if _type == "PUT" {
                transfer
                    .read_function(|new_data| Ok(data.read(new_data).unwrap_or(0)))
                    .unwrap();

                transfer.perform().unwrap();
            } else if _type == "GET" {
                transfer
                    .write_function(|new_data| {
                        data2.extend_from_slice(new_data);
                        Ok(new_data.len())
                    }).unwrap();
                match transfer.perform() {
                    Ok(()) => {}
                    _ => {
                        println!("Connection error");
                        return Err(Error::new(ErrorKind::NotConnected, "Connection error"));
                    }
                }
            }
        }
        match self.easy_handle.response_code() {
            Ok(200) => {}
            Ok(204) => {}
            _ => {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("{:?}", self.easy_handle.response_code().unwrap()),
                ));
            }
        }
        self.easy_handle.reset();
        let s = str::from_utf8(&data2).unwrap();
        *result = s.to_string();
        Ok(())
    }
    /**
     * Generate another access token
     */
    pub fn refresh(
        &mut self,
        _base64_secret: &str,
        _refresh_token: &str,
        code: &str,
    ) -> Result<(), std::io::Error> {
        //println!("Refreshing access token");
        let mut data = Vec::new();
        {
            self.easy_handle.post(true).unwrap();
            if code.len() > 0 {
                self.easy_handle
                    .post_fields_copy(
                        &format!("{}{}", "grant_type=authorization_code&code=", code).as_bytes(),
                    ).unwrap();
            } else {
                self.easy_handle
                    .post_fields_copy(
                        &format!(
                            "{}{}",
                            "grant_type=refresh_token&refresh_token=", _refresh_token
                        ).as_bytes(),
                    ).unwrap();
            }

            self.easy_handle
                .url("https://accounts.spotify.com/api/token")
                .unwrap();
            let mut list = List::new();
            let _request2 = format!("{}{}", "Authorization: Basic ", _base64_secret);
            list.append(&_request2).unwrap();
            self.easy_handle.http_headers(list).unwrap();
            let mut transfer = self.easy_handle.transfer();
            transfer
                .write_function(|new_data| {
                    data.extend_from_slice(new_data);
                    Ok(new_data.len())
                }).unwrap();
            match transfer.perform() {
                Ok(()) => {}
                _ => {
                    println!("Connection error");
                    return Err(Error::new(ErrorKind::NotConnected, "Connection error"));
                }
            }
        }
        match self.easy_handle.response_code() {
            Ok(200) => {}
            Ok(204) => {}
            _ => {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("{:?}", self.easy_handle.response_code().unwrap()),
                ));
            }
        }
        self.easy_handle.reset();

        let s = str::from_utf8(&data).unwrap();

        let v: Value = serde_json::from_str(s).unwrap();

        let mut result = v["access_token"].to_string();
        result = result[1..].to_string();
        result.pop();
        //println!("Ok ! Access token : {}",result.as_str());
        self.access_token = result;
        Ok(())
    }
    pub fn get_access_token(&self) -> &str {
        return self.access_token.as_str();
    }
}
