use super::super::model;
use super::super::EasyAPI;
use serde_json::Value;

impl EasyAPI {
    /// Searches for a playlist and play the first item
    /// found with that name. Works only with playlist now..
    /// TODO extend from playlist mode to any mode..
    pub fn search_and_play_first(
        &mut self,
        type_: &str,
        search: &str,
    ) -> Result<(), std::io::Error> {
        let mut result = String::new();
        let errno = self.command.search(search, type_, &mut result);
        if errno.is_err() {
            return errno;
        }
        let v: Value = serde_json::from_str(result.as_str()).unwrap();
        // work for playlist, we should verify the JSON out for other types to get the right thing
        result = v["playlists"]["items"][0]["id"].to_string(); // just getting the first result here
        result = result[1..].to_string(); // removing last '"'
        result.pop(); // removing first '"'
                      //println!("{}",result);

        self.command.play(result.as_str(), type_, "", "")
    }

    /// Searches for playlists with the name "search" in it.
    /// Stores the results in a reference to a vector of Playlist on "final_result"
    pub fn search_playlist(
        &mut self,
        search: &str,
    ) -> Result<Vec<model::playlist::SimplifiedPlaylist>, std::io::Error> {
        let mut result = String::new();
        let mut final_result = Vec::new();
        match self.command.search(search, "playlist", &mut result) {
            Ok(_ok) => {}
            Err(error) => return Err(error),
        }
        let v: Value = serde_json::from_str(result.as_str()).unwrap();
        let size = v["playlists"]["items"].as_array().unwrap().len();
        for x in 0..size {
            final_result.push(serde_json::from_str(
                &serde_json::to_string(&v["playlists"]["items"][x]).unwrap(),
            )?);
        }
        Ok(final_result)
    }

    /// Searches for albums with the name "search" in it.
    /// Stores the results in a reference to a vector of Albums on "final_result"
    pub fn search_album(
        &mut self,
        search: &str,
    ) -> Result<Vec<model::album::SimplifiedAlbum>, std::io::Error> {
        let mut result = String::new();
        let mut final_result = Vec::new();

        match self.command.search(search, "album", &mut result) {
            Ok(_ok) => {}
            Err(error) => return Err(error),
        }
        let v: Value = serde_json::from_str(result.as_str()).unwrap();
        let size = v["albums"]["items"].as_array().unwrap().len();
        for x in 0..size {
            final_result.push(serde_json::from_str(
                &serde_json::to_string(&v["albums"]["items"][x]).unwrap(),
            )?);
        }

        Ok(final_result)
    }

    /// Searches for tracks with the name "search" in it.
    /// Stores the results in a reference to a vector of Tracks on "final_result"
    pub fn search_track(
        &mut self,
        search: &str,
    ) -> Result<Vec<model::track::SimplifiedTrack>, std::io::Error> {
        let mut result = String::new();
        let mut final_result = Vec::new();

        match self.command.search(search, "track", &mut result) {
            Ok(_ok) => {}
            Err(error) => return Err(error),
        }
        let v: Value = serde_json::from_str(result.as_str()).unwrap();
        let size = v["tracks"]["items"].as_array().unwrap().len();
        for x in 0..size {
            final_result.push(serde_json::from_str(
                &serde_json::to_string(&v["tracks"]["items"][x]).unwrap(),
            )?);
        }

        Ok(final_result)
    }

    /// Searches for artists with the name "search" in it.
    /// Stores the results in a reference to a vector of Artists on "final_result"
    pub fn search_artist(
        &mut self,
        search: &str,
    ) -> Result<Vec<model::artist::FullArtist>, std::io::Error> {
        let mut result = String::new();
        let mut final_result = Vec::new();
        match self.command.search(search, "artist", &mut result) {
            Ok(_ok) => {}
            Err(error) => return Err(error),
        }
        let v: Value = serde_json::from_str(result.as_str()).unwrap();
        let size = v["artists"]["items"].as_array().unwrap().len();
        for x in 0..size {
            final_result.push(serde_json::from_str(
                &serde_json::to_string(&v["artists"]["items"][x]).unwrap(),
            )?);
        }

        Ok(final_result)
    }
}
