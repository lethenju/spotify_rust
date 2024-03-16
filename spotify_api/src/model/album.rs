//! All objects related to album defined by Spotify API
use std::collections::HashMap;
use chrono::prelude::*;

use super::senum::{Type, AlbumType};
use super::track::SimplifiedTrack;
use super::artist::SimplifiedArtist;
use super::image::Image;
use super::page::Page;
///[link to album object simplified](https://developer.spotify.com/web-api/object-model/#album-object-simplified)
/// Simplified Album Object
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimplifiedAlbum {
    pub artists: Vec<SimplifiedArtist>,
    pub album_type: String,
    pub available_markets: Vec<String>,
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    #[serde(rename = "type")]
    pub _type: Type,
    pub uri: String,
    pub release_date: String,
}


///[link to album object full](https://developer.spotify.com/web-api/object-model/#album-object-full)
/// Full Album Object
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FullAlbum {
    pub artists: Vec<SimplifiedArtist>,
    pub album_type: AlbumType,
    pub available_markets: Vec<String>,
    //pub copyrights: Vec<HashMap<String, String>>,
    //pub external_ids: HashMap<String, String>,
    pub external_urls: HashMap<String, String>,
    pub genres: Vec<String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    //pub popularity: u32,
    pub release_date: String,
    pub release_date_precision: String,
    pub tracks: Page<SimplifiedTrack>,
    #[serde(rename = "type")]
    pub _type: Type,
    pub uri: String,
}


impl FullAlbum {
    ///Transformation to a SimplifiedAlbum
    pub fn to_simplified(
        &mut self
    ) -> SimplifiedAlbum {
        SimplifiedAlbum{
            artists : self.artists.clone(),
            _type : self._type.clone(),
            available_markets : self.available_markets.clone(),
            name : self.name.clone(),
            id : self.id.clone(),
            album_type : String::from(self.album_type.as_str()),
            external_urls : self.external_urls.clone(),
            href : self.href.clone(),
            images : self.images.clone(),
            uri : self.uri.clone(),
            release_date : self.release_date.clone(),
        }
    }
}

/// Full Albums
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FullAlbums {
    pub albums: Vec<FullAlbum>,
}

///[link to get list new releases](https://developer.spotify.com/web-api/get-list-new-releases/)
/// Simplified Albums wrapped by Page object
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PageSimpliedAlbums {
    pub albums: Page<SimplifiedAlbum>,
}

///[link to save album object](https://developer.spotify.com/web-api/object-model/#save-album-object)
/// Saved Album object
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SavedAlbum {
    pub added_at: DateTime<Utc>,
    pub album: FullAlbum,
}


#[derive(Clone, Debug)]
pub struct SimplifiedAlbumWithTracks {
    pub data: SimplifiedAlbum,
    pub tracks: Vec<SimplifiedTrack>,
}