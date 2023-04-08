/*  
    @Author Julien LE THENO
    @mod Data Manager : centralizes the data around the application by giving API to store and get data around
*/

use std::collections::HashMap;

use std::sync::Mutex;

use spotify_api::model::album::FullAlbum;
use spotify_api::model::album::SimplifiedAlbum;
use spotify_api::model::album::SimplifiedAlbumWithTracks;
use spotify_api::model::artist::SimplifiedArtistWithAlbums;
use spotify_api::model::track::SimplifiedTrack;
use spotify_api::model::context::FullPlayingContext;
use spotify_api::model::context::FullPlayingContextTimeStamped;

struct DataStore<T: Clone> {
    data: Mutex<HashMap<String, T>>,
    subscribers : Mutex<HashMap<String, Vec<Box<dyn Fn(&T) + Send + 'static>>>>,
}

pub struct DataManager {
    pub album_lists : DataStore<Vec<FullAlbum>>,
    pub albums  :  DataStore<FullAlbum>,
}


impl<T: Clone> DataStore<T> {
    pub fn new() -> Self {
        DataStore {
            data: Mutex::new(HashMap::new()),
            subscribers: Mutex::new(HashMap::new()),
        }
    }

    pub fn write(&mut self, key: &str, value: T) {
        let mut data = self.data.lock().unwrap();
        let mut subscribers = self.subscribers.lock().unwrap();

        if let Some(subscribers) = subscribers.get_mut(key) {
            for callback in subscribers.iter() {
                callback(&value);
            }
        }
        data.insert(key.to_owned(), value);
    }

    pub fn read(&self, key: &str) -> Option<T> {
        let data = self.data.lock().unwrap();
        data.get(key).cloned()
    }

    pub  fn subscribe<F> (&mut self, key: &str, callback: F)
    where 
        F: Fn(&T) + Send + 'static,
    {
        let mut subscribers = self.subscribers.lock().unwrap();
        subscribers.entry(key.to_owned()).or_insert_with(Vec::new).push(Box::new(callback));
    }
}

impl DataManager {
    pub fn new() -> Self {
        DataManager {
            album_lists : DataStore::new(),
            albums      : DataStore::new(),
        }
    }

}