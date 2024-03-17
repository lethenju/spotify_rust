/*  
    @Author Julien LE THENO
    @mod Files : handles the key loader
*/
use std::fs::File;
use std::io::prelude::*;

///
/// Loads the refresh token and the application credentials from files
///
/// Takes in parameter the references to the refresh token and the base64 of
/// the clientid:clientsecret, and thoses String will be filled during this funtion.
///
/// If the files doesnt exits, it returns
/// ```rust
/// std::io::Error::new(
///             std::io::ErrorKind::NotFound,
///             "No file :(",
///         ));
/// ```
///
/// # Example
///
/// ```rust
/// let mut refresh_token = String::new();
/// let mut base_64_secret = String::new();
///
/// load_keys(&mut refresh_token, &mut base_64_secret).unwrap();
///
/// println!("{}  -  {}",refresh_token.as_str(),base_64_secret.as_str());
/// ```
///
pub fn load_keys(
    refresh_token: &mut String,
    base_64_secret: &mut String,
) -> Result<(), std::io::Error> {
    println!("Loading keys..");

    match File::open("refresh_token") {
        Ok(file) => {
            let mut f = file;
            f.read_to_string(refresh_token)
                .expect("something went wrong reading the file");
        }
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No file :(",
            ));
        }
    }

    match File::open("base_64_secret") {
        Ok(file) => {
            let mut f = file;
            f.read_to_string(base_64_secret)
                .expect("something went wrong reading the file");
        }
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No file :(",
            ));
        }
    }
    println!("Keys loaded");
    Ok(())
}


pub fn read_library(
    json_library: &mut String,
) -> Result<(), std::io::Error> {
    println!("Loading library");

    match File::open("library") {
        Ok(file) => {
            let mut f = file;
            f.read_to_string(json_library)
                .expect("something went wrong reading the file");
        }
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No file :(",
            ));
        }
    }

    println!("Library loaded ! ");
    Ok(())
}


pub fn read_artists(
    json_artists: &mut String,
) -> Result<(), std::io::Error> {

    match File::open("artists") {
        Ok(file) => {
            let mut f = file;
            f.read_to_string(json_artists)
                .expect("something went wrong reading the file");
        }
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No file :(",
            ));
        }
    }
    Ok(())
}

/// Writes the library on disk for speeding up startup
pub fn write_library(library: Vec<crate::model::album::FullAlbum>) -> Result<(), std::io::Error> {
    let v = serde_json::to_string(&library);
    let mut buffer = File::create("library").unwrap();
    write!(buffer,"{}", v.unwrap())
}

/// Writes the artists library for caching const data
pub fn write_artists(artists: Vec<crate::model::artist::SimplifiedArtist>) -> Result<(), std::io::Error> 
{
    let v = serde_json::to_string(&artists);
    let mut buffer = File::create("artists").unwrap();
    write!(buffer,"{}", v.unwrap())
}
