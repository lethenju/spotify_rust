extern crate curl;
mod easy_api;

use easy_api::EasyAPI;

fn main() {
    let mut easy_api = EasyAPI::construct();
    
    easy_api.refresh();
    
    println!("Hello, world!");
}
