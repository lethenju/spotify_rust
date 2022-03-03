/*  
    @Author Julien LE THENO
    @mod Command : handles the command building and a Communicator
    object
*/
extern crate curl;
extern crate percent_encoding;


mod communicator;
use self::communicator::Communicator;
use self::curl::easy::List;
use self::percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};

pub struct Command {
    communicator: Communicator,
}

impl Command {
    pub fn new() -> Command {
        let communicator = Communicator::new("NULL");
        return Command { communicator };
    }
    pub fn play(
        &mut self,
        _spotify_id: &str,
        _type: &str,
        context_id: &str,
        context_type: &str,
    ) -> Result<(), std::io::Error> {
        let mut body_params = String::new();
        if context_id.len() > 0 {
            // if there is a context
            let context_id = format!(
                "\"context_uri\":\"spotify:{}:{}\"",
                context_type, context_id
            ).to_string();
            body_params = format!(
                "{{{},\"offset\": {{\"uri\": \"spotify:{}:{}\"}}}}",
                context_id, _type, _spotify_id
            ).to_string();
        } else if _type.len() > 0 {
            // if there is no context but not just "resume"
            body_params =
                format!("{{\"uris\": [\"spotify:{}:{}\"]}}", _type, _spotify_id).to_string();
        }
        //println!("{}", body_params);
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
        return self.communicator.perform(
            "https://api.spotify.com/v1/me/player/play",
            "",
            &body_params.as_str(),
            list_headers,
            "PUT",
            &mut result,
        );
    }
    pub fn pause(&self) -> Result<(), std::io::Error> {
        unimplemented!()
    }
    pub fn next(&self) -> Result<(), std::io::Error> {
        unimplemented!()
    }
    pub fn search(
        &mut self,
        _name: &str,
        _type: &str,
        result: &mut String,
    ) -> Result<(), std::io::Error> {
        let mut list_headers = List::new();
        let _auth = format!(
            "{}{}",
            "Authorization: Bearer ",
            self.communicator.get_access_token()
        );
        list_headers.append(&_auth).unwrap();

        let _search = utf8_percent_encode(_name, DEFAULT_ENCODE_SET).to_string();
        let _req = format!("q={}&type={}", _search, _type);
        return self.communicator.perform(
            "https://api.spotify.com/v1/search",
            &_req,
            "",
            list_headers,
            "GET",
            &mut *result,
        );
    }
    pub fn get_my_albums(
        &mut self,
        offset: u16,
        result: &mut String,
    ) -> Result<(), std::io::Error> {
        let mut list_headers = List::new();
        let _auth = format!(
            "{}{}",
            "Authorization: Bearer ",
            self.communicator.get_access_token()
        );
        list_headers.append(&_auth).unwrap();

        return self.communicator.perform(
            "https://api.spotify.com/v1/me/albums",
            format!("limit=20&offset={}", offset).as_str(),
            "",
            list_headers,
            "GET",
            &mut *result,
        );
    }
    pub fn get_tracks_from_album(
        &mut self,
        id: &str,
        result: &mut String,
    ) -> Result<(), std::io::Error> {
        // GET https://api.spotify.com/v1/albums/{id}/tracks
        let mut list_headers = List::new();
        let _auth = format!(
            "{}{}",
            "Authorization: Bearer ",
            self.communicator.get_access_token()
        );
        list_headers.append(&_auth).unwrap();

        return self.communicator.perform(
            format!("https://api.spotify.com/v1/albums/{}/tracks", id).as_str(),
            "",
            "",
            list_headers,
            "GET",
            &mut *result,
        );
    }
    pub fn get_albums_from_artist(&mut self,
        id: &str,
        result: &mut String,
    ) -> Result<(), std::io::Error> {
        // GET https://api.spotify.com/v1/artists/{}/albums
        let mut list_headers = List::new();
        let _auth = format!(
            "{}{}",
            "Authorization: Bearer ",
            self.communicator.get_access_token()
        );
        list_headers.append(&_auth).unwrap();

        return self.communicator.perform(
            format!("https://api.spotify.com/v1/artists/{}/albums", id).as_str(),
            "",
            "",
            list_headers,
            "GET",
            &mut *result,
        );
    }
    pub fn get_currently_playing(&mut self, result: &mut String) -> Result<(), std::io::Error> {
        let mut list_headers = List::new();
        let _auth = format!(
            "{}{}",
            "Authorization: Bearer ",
            self.communicator.get_access_token()
        );
        list_headers.append(&_auth).unwrap();

        return self.communicator.perform(
            "https://api.spotify.com/v1/me/player/currently-playing",
            "",
            "",
            list_headers,
            "GET",
            &mut *result,
        );
    }

    pub fn get_playback_state(&mut self, result: &mut String) -> Result<(), std::io::Error> {
        let mut list_headers = List::new();
        let _auth = format!(
            "{}{}",
            "Authorization: Bearer ",
            self.communicator.get_access_token()
        );
        list_headers.append(&_auth).unwrap();

        return self.communicator.perform(
            "https://api.spotify.com/v1/me/player",
            "",
            "",
            list_headers,
            "GET",
            &mut *result,
        );
    }
    pub fn refresh(
        &mut self,
        base_64_secret: &str,
        refresh_token: &str,
    ) -> Result<(), std::io::Error> {
        return self.communicator.refresh(base_64_secret, refresh_token);
    }
    pub fn retrieve_refresh_token(
        &mut self,
        base_64_secret: &str,
        authorization_code: &str,
    ) -> Result<String, std::io::Error> {
        Ok(self
            .communicator
            .retrieve_refresh_token(base_64_secret, authorization_code)
            .unwrap())
    }







    pub fn get_wiki_description(&mut self, search: String, result: &mut String) -> Result<(), std::io::Error> {
        let _search = utf8_percent_encode(&search, DEFAULT_ENCODE_SET).to_string();
        return self.communicator.perform(
            format!("https://en.wikipedia.org/w/api.php?action=query&prop=extracts&explaintext=true&format=json&exchars=300&titles={}",_search).as_str(),
            "",
            "",
            List::new(),
            "GET",
            &mut *result,
        );
    }
}
