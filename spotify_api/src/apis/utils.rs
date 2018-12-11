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
}
