use super::super::model;
use super::super::EasyAPI;
use super::super::files;
use serde_json::Value;
use std::fs::File;
use std::io::Write;

impl EasyAPI {

    /// Get 'page_size' albums in the user's library with a given offset
    pub fn get_my_albums_chunk(
        &mut self,
        offset: u16,
        page_size : u16,
        final_result: &mut Vec<model::album::FullAlbum>,
    ) -> Result<(), std::io::Error> {
        let mut result = String::new();
        let errno = self.command.get_my_albums(offset, page_size, &mut result);
        if errno.is_err() {
            return errno;
        }
        let v: Value = serde_json::from_str(result.as_str()).unwrap();
        // Total number of albums is known in the request 
        // work for playlist, we should verify the JSON out for other types to get the right thing
        let size = v["items"].as_array().unwrap().len();
        if size != usize::from(page_size) {
            println!("Warning get_my_albums_chunk : Page size {} different from available size {} ", page_size, size)
        }
        for x in 0..size {
            let mut alb : model::album::FullAlbum;
            alb = serde_json::from_str(&serde_json::to_string(&v["items"][x]["album"]).unwrap(),)?;
            match self.get_genres_from_artist(&alb.artists[0].id)
            {
                Ok(result_genres) => {
                    if !result_genres.is_empty()
                    {
                        alb.genres = result_genres;
                        println!("Genre for this album {} : {}", alb.name, alb.genres[0]);
                    }
                }
                Err(e) => {println!("Genre not found for this album : {} : {}", alb.name, e.to_string())}
            }
            alb.available_markets.clear();
            final_result.push(alb);
        }
        Ok(())
    }

    pub fn get_my_albums_count(&mut self) -> Result<u32, std::io::Error>
    {
    
        let mut result = String::new();
        let errno = self.command.get_my_albums(0, 1, &mut result);
        match errno {
            Ok(()) => (),
            Err(e) => return Err(e) 
        }
        let v: Value = serde_json::from_str(result.as_str()).unwrap();
        // Total number of albums is known in the request 
        println!("Total number of albums  = {}", v["total"]);
        
        let total_count : u64 = v["total"].as_u64().unwrap();
        Ok(total_count as u32)
    }

    /// Load library
    pub fn read_library(&mut self,
          final_result: &mut Vec<model::album::FullAlbum>) -> Result<(), std::io::Error> {
        let mut result = String::new();
        match files::read_library(&mut result) {
            Ok(()) => {

            }
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "No file :(",
                ));
            }
        }
        let v: Value = serde_json::from_str(result.as_str()).unwrap();

        // work for playlist, we should verify the JSON out for other types to get the right thing
        let size = v.as_array().unwrap().len();
        for x in 0..size {
            final_result.push(serde_json::from_str(
                &serde_json::to_string(&v[x]).unwrap(),
            )?);
        }

        Ok(())
    }
    /// Write library
    pub fn write_library(&mut self, library:  Vec<model::album::FullAlbum>)-> Result<(), std::io::Error>{

            let v = serde_json::to_string(&library);
            let mut buffer = File::create("library").unwrap();
            write!(buffer,"{}", v.unwrap())
    }
}
