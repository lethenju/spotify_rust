/*  
    @Author Julien LE THENO
    @mod Command : handles the command building and a Communicator
    object
*/
extern crate curl;
extern crate percent_encoding;

mod communicator;
use self::communicator::Communicator;
use self::curl::easy::{Easy, List};
use self::percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};

pub struct Command {
    communicator: Communicator,
}

impl Command {
    pub fn construct() -> Command {
        let communicator = Communicator::construct("NULL");
        return Command { communicator };
    }
    pub fn play(&mut self, _spotify_id: &str, _type: &str) {
        let mut body_params = String::new();
        if _type != "track" {
            body_params = format!("{{\"context_uri\":\"spotify:{}:{}\"}}", _type, _spotify_id).to_string();
        } else {
            body_params = format!("{{\"uris\": [\"spotify:track:{}\"]}}", _spotify_id).to_string();
        }
        let mut list_headers = List::new();
        let _auth = format!(
            "{}{}",
            "Authorization: Bearer ",
            self.communicator.get_access_token()
        );

        list_headers.append("Accept: application/json").unwrap();
        list_headers
            .append("Content-Type: application/json")
            .unwrap();
        list_headers.append(&_auth).unwrap();
        let mut result = String::new();
        self.communicator.perform(
            "https://api.spotify.com/v1/me/player/play",
            "",
            &body_params.as_str(),
            list_headers,
            "PUT",
            &mut result,
        );
    }
    pub fn pause(&self) {}
    pub fn next(&self) {}
    pub fn search(&mut self, _name: &str, _type: &str, result: &mut String) {
        let mut list_headers = List::new();
        let _auth = format!(
            "{}{}",
            "Authorization: Bearer ",
            self.communicator.get_access_token()
        );
        list_headers.append(&_auth).unwrap();

        let _search = utf8_percent_encode(_name, DEFAULT_ENCODE_SET).to_string();
        let _req = format!("q={}&type={}", _search, _type);
        self.communicator.perform(
            "https://api.spotify.com/v1/search",
            &_req,
            "",
            list_headers,
            "GET",
            &mut *result,
        );
    }
    pub fn get_my_albums(&mut self, result: &mut String) {
        let mut list_headers = List::new();
        let _auth = format!(
            "{}{}",
            "Authorization: Bearer ",
            self.communicator.get_access_token()
        );
        list_headers.append(&_auth).unwrap();

        self.communicator.perform(
            "https://api.spotify.com/v1/me/albums",
            "limit=50",
            "",
            list_headers,
            "GET",
            &mut *result,
        );
    }
    pub fn get_tracks_from_album(&mut self, id: &str, result: &mut String) {
        // GET https://api.spotify.com/v1/albums/{id}/tracks
        let mut list_headers = List::new();
        let _auth = format!(
            "{}{}",
            "Authorization: Bearer ",
            self.communicator.get_access_token()
        );
        list_headers.append(&_auth).unwrap();

        self.communicator.perform(
            format!("https://api.spotify.com/v1/albums/{}/tracks", id).as_str(),
            "",
            "",
            list_headers,
            "GET",
            &mut *result,
        );
    }
    pub fn get_currently_playing(&mut self, result: &mut String) {
        let mut list_headers = List::new();
        let _auth = format!(
            "{}{}",
            "Authorization: Bearer ",
            self.communicator.get_access_token()
        );
        list_headers.append(&_auth).unwrap();

        self.communicator.perform(
            "https://api.spotify.com/v1/me/player/currently-playing",
            "",
            "",
            list_headers,
            "GET",
            &mut *result,
        );
    }
    pub fn refresh(&mut self, _base_64_secret: &str, _refresh_token: &str) {
        let mut access_token = String::new();
        self.communicator.refresh(_base_64_secret, _refresh_token);
    }
}
