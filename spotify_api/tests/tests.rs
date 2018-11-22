extern crate spotify_api;
use spotify_api::EasyAPI;
#[cfg(test)]
#[test]
fn initialize_handle() {
    let mut handle = EasyAPI::new();
    handle.refresh();
}

#[test]
fn search_tracks() {
    let mut handle = EasyAPI::new();
    handle.refresh();
    let mut billie_jean_results = Vec::new();
    handle.search_track("billie jean", &mut billie_jean_results).unwrap();
    assert!(billie_jean_results.len() > 0);
    for track in billie_jean_results {
        println!("TRACKS {} : {}", track.id, track.name);
    }
}

#[test]
fn search_albums() {
    let mut handle = EasyAPI::new();
    handle.refresh();
    let mut thriller_results = Vec::new();
    handle.search_album("thriller", &mut thriller_results).unwrap();
    assert!(thriller_results.len() > 0);
    for album in thriller_results {
        println!("ALBUMS {} : {}", album.id, album.name);
    }
}

#[test]
fn search_artists() {
    let mut handle = EasyAPI::new();
    handle.refresh();
    let mut mj_results = Vec::new();
    handle.search_artist("Michael Jackson", &mut mj_results).unwrap();
    assert!(mj_results.len() > 0);
    for artist in mj_results {
        println!("ARTIST {} : {}", artist.id, artist.name);
    }
}

#[test]
fn search_playlists() {
    let mut handle = EasyAPI::new();
    handle.refresh();
    let mut mj_results = Vec::new();
    handle.search_playlist("Michael Jackson", &mut mj_results).unwrap();
    assert!(mj_results.len() > 0);
    for playlist in mj_results {
        println!("PLAYLIST {} : {}", playlist.id, playlist.name);
    }
}

#[test]
fn play_without_context() {
    let mut handle = EasyAPI::new();
    handle.refresh();
    let mut billie_jean_results = Vec::new();
    handle.search_track("billie jean", &mut billie_jean_results).unwrap();
    handle.play_track(&billie_jean_results[0], None).unwrap();
}
