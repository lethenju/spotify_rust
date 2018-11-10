use std::fs::File;
use std::io::prelude::*;

/*  
    @Author Julien LE THENO
    @mod Files : handles the key loader
*/

pub fn load_keys(refresh_token : &mut String, base_64_secret : &mut String) {
    println!("Loading keys..");
    let mut f = File::open("refresh_token").expect("file not found");
    f.read_to_string(refresh_token)
        .expect("something went wrong reading the file");

    f = File::open("base_64_secret").expect("file not found");
    f.read_to_string(base_64_secret)
        .expect("something went wrong reading the file");
    println!("Keys loaded");
}