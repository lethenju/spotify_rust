extern crate spotify_api;
use spotify_api::EasyAPI;
#[cfg(test)]
#[test]
fn initialize_handle() {
    let mut handle = EasyAPI::new();
    handle.refresh();
}

#[test]
fn test_search_tracks() {
    let mut handle = EasyAPI::new();
    handle.refresh();
    let mut billie_jean_results = Vec::new();
    handle.search_track("billie jean", &mut billie_jean_results);
    assert!(billie_jean_results.len() > 0);
}

#[test]
fn test_search_albums() {
    let mut handle = EasyAPI::new();
    handle.refresh();
    let mut thriller_results = Vec::new();
    handle.search_album("thriller", &mut thriller_results);
    assert!(thriller_results.len() > 0);
}

#[test]
fn test_search_artists() {
    let mut handle = EasyAPI::new();
    handle.refresh();
    let mut mj_results = Vec::new();
    handle.search_album("Michael Jackson", &mut mj_results);
    assert!(mj_results.len() > 0);
}


#[test]
fn test_play_without_context() {
    let mut handle = EasyAPI::new();
    handle.refresh();
    let mut billie_jean_results = Vec::new();
    handle.search_track("billie jean", &mut billie_jean_results);
    handle.play_track(&billie_jean_results[0], None);
}
