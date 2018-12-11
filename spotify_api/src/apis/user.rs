use super::super::model;
use super::super::EasyAPI;
use serde_json::Value;

impl EasyAPI {
    ///  Get all the current user's albums
    pub fn get_my_albums(
        &mut self,
        final_result: &mut Vec<model::album::SimplifiedAlbum>,
    ) -> Result<(), std::io::Error> {
        for i in 0..5 {
            // SUPER dirty -> TODO get number of album to know how many chunks to get.
            self.get_my_albums_chunk(i * 50, final_result).unwrap();
        }
        Ok(())
    }
    /// Get 20 albums in the user's library with a given offset
    pub fn get_my_albums_chunk(
        &mut self,
        offset: u16,
        final_result: &mut Vec<model::album::SimplifiedAlbum>,
    ) -> Result<(), std::io::Error> {
        let mut result = String::new();
        let errno = self.command.get_my_albums(offset, &mut result);
        if errno.is_err() {
            return errno;
        }
        let v: Value = serde_json::from_str(result.as_str()).unwrap();
        // work for playlist, we should verify the JSON out for other types to get the right thing
        let size = v["items"].as_array().unwrap().len();
        for x in 0..size {
            final_result.push(serde_json::from_str(
                &serde_json::to_string(&v["items"][x]["album"]).unwrap(),
            )?);
        }
        Ok(())
    }
}
