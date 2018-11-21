extern crate spotify_rust;
#[cfg(test)]
mod tests {

    #[test]
    fn initializing_api() {
        let mut easy_api = spotify_rust::EasyAPI::construct();
        easy_api.refresh().unwrap();
    }

    #[test]
    fn getting_albums() {
        let mut easy_api = spotify_rust::EasyAPI::construct();
        easy_api.refresh().unwrap();
        let mut my_albums = Vec::new();
        easy_api.get_my_albums(&mut my_albums).unwrap();
        assert!(my_albums.len() > 0, true);
    }

}
