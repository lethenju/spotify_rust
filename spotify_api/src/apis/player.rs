use super::super::model;
use super::super::EasyAPI;
use serde_json::Value;

impl EasyAPI {
    /// Pauses the playback.
    /// Not implemented yet
    pub fn pause(&mut self) -> Result<(), std::io::Error> {
        unimplemented!();
    }
    /// Returns the next song in the active context
    /// Not implemented yet
    pub fn get_next(&mut self) -> Result<model::track::FullTrack, std::io::Error> {
        unimplemented!();
    }
    /// Returns the previous track
    /// Not implemented yet
    pub fn get_previous(&mut self) -> Result<model::track::FullTrack, std::io::Error> {
        unimplemented!();
    }

    /// Gets the currently playing artist on the final_result argument
    /// final_result setted to "" if no track is playing
    pub fn get_currently_playing_artist(
        &mut self,
    ) -> Result<Option<model::artist::SimplifiedArtist>, std::io::Error> {
        let mut result = String::new();
        let errno = self.command.get_currently_playing(&mut result);
        match errno {
            Err(error) => return Err(error),
            _ => {}
        }

        if result.len() != 0 {
            let v: Value = serde_json::from_str(result.as_str()).unwrap();
            return Ok(Some(
                serde_json::from_str(&serde_json::to_string(&v["item"]["artist"][0]).unwrap())
                    .unwrap(),
            ));
        }
        Ok(None)
    }

    /// Gets the currently playing track on the final_result argument
    /// final_result setted to "" if no track is playing
    pub fn get_currently_playing_track(
        &mut self,
    ) -> Result<Option<model::track::FullTrack>, std::io::Error> {
        let mut result = String::new();
        let errno = self.command.get_currently_playing(&mut result);
        match errno {
            Err(error) => return Err(error),
            _ => {}
        }
        if result.len() != 0 {
            let v: Value = serde_json::from_str(result.as_str()).unwrap();
            return Ok(Some(
                serde_json::from_str(&serde_json::to_string(&v["item"]).unwrap()).unwrap(),
            ));
        }
        Ok(None)
    }
    /// Plays a track in a context ( for now just Album..)
    pub fn play_track(
        &mut self,
        track: &model::track::SimplifiedTrack,
        context: Option<&model::album::SimplifiedAlbum>,
    ) -> Result<(), std::io::Error> {
        let error = {
            match context {
                Some(context) => {
                    self.command
                        .play(track.id.as_str(), "track", context.id.as_str(), "album")
                }
                None => self.command.play(track.id.as_str(), "track", "", ""),
            }
        };
        match error {
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    println!("No devices connected for the playback!")
                }
            }
            _ => return error,
        }

        Ok(())
    }
}
