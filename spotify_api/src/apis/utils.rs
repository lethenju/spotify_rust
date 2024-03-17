use std::collections::HashMap;

use crate::model::artist::SimplifiedArtist;

use super::super::model;
use super::super::EasyAPI;
use serde_json::Value;

impl EasyAPI {
    ///  Gets the track names from a given album id
    pub fn get_tracks_from_album(
        &mut self,
        id_album: &str,
    ) -> Result<Vec<model::track::SimplifiedTrack>, std::io::Error> {
        let mut result = String::new();
        let mut final_result = Vec::new();
        match self.command.get_tracks_from_album(id_album, &mut result) {
            Ok(_ok) => {}
            Err(error) => return Err(error),
        }
        let v: Value = serde_json::from_str(result.as_str()).unwrap();
        // work for playlist, we should verify the JSON out for other types to get the right thing
        let size = v["items"].as_array().unwrap().len();
        for x in 0..size {
            final_result.push(serde_json::from_str(
                &serde_json::to_string(&v["items"][x]).unwrap(),
            )?);
        }
        Ok(final_result)
    }

    /// Gets the genres list associated with an artist
    pub fn get_genres_from_artist(
        &mut self,
        id_artist: &str,
    ) -> Result<Vec<String>, std::io::Error> {
        let mut result: SimplifiedArtist = SimplifiedArtist
        { external_urls: HashMap::new(),
          href: String::new(),
          id: String::new(),
          name: String::new(),
          genres: Option::None,
          _type: model::senum::Type::Artist,
           uri: String::new() };
        let mut final_result: Vec<String> = Vec::new();
        match self.get_artist_data(id_artist, &mut result) {
            Ok(_ok) => {}
            Err(error) => return Err(error),
        }
        match result.genres {
            Some(genres) => {final_result = genres;},
            None => {}
        }
        Ok(final_result)
    }


    pub fn get_albums_from_artist(
        &mut self,
        id_artist: &str,
    ) -> Result<Vec<model::album::SimplifiedAlbum>, std::io::Error> {
        let mut result = String::new();
        let mut final_result = Vec::new();
        match self.command.get_albums_from_artist(id_artist, &mut result) {
            Ok(_ok) => {}
            Err(error) => return Err(error),
        }
        let v: Value = serde_json::from_str(result.as_str()).unwrap();
        // work for playlist, we should verify the JSON out for other types to get the right thing
        let size = v["items"].as_array().unwrap().len();
        for x in 0..size {
            final_result.push(serde_json::from_str(
                &serde_json::to_string(&v["items"][x]).unwrap(),
            )?);
        }
        Ok(final_result)
    }

    /// Gets the artist data either by checking the local data file or by downloading
    fn get_artist_data(&mut self, 
        id: &str,
        result :&mut SimplifiedArtist
    ) -> Result<(), std::io::Error> {
        let mut local_artists = Vec::new();
        self.read_artists(&mut local_artists);
    
        for artist in &local_artists
        {
            if artist.id == id
            {
                *result = artist.clone();
                return Ok(());
            }
        }
        
        let mut result_api = String::new();
        match self.command.get_artist_data(id,&mut result_api)
        {
            Ok(()) => {},
            Err(error) => {return Err(error)}
        }
        let v: SimplifiedArtist = serde_json::from_str(result_api.as_str()).unwrap();
        *result = v;
        return Ok(())
        // TODO
        // Store artists genres in a local file
        // If artist genres are available in that file, load them
        // Launch the get_artist_data api only otherwise
        // Update that local file accordingly
    }
    
}