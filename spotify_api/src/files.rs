use std::fs::File;
use std::io::prelude::*;

/*  
    @Author Julien LE THENO
    @mod Files : handles the key loader
*/

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
