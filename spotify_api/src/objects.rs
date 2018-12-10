/// Objects data model returned by the Spotify web API


/// Track structure that represents a spotify track, with its name, Artist and Spotify ID.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub name: String,
    pub id: String,
    pub artist: Option<Artist>,
}
/// Album structure that represents a spotify album, with its name, Spotify ID, and a Vector of Tracks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Album_full {
    pub album_type : String,
    pub artists : Vec<Artist_simp>,
    pub available_markets : Vec<String>,
    pub copyrights : Vec<Copyright>,
    pub external_ids : Vec<Ext_ID>,
    pub external_urls : Vec<Ext_URL>,
    pub genres : Vec<String>,
    pub href : String,
    pub id: String,
    pub images : Vec<Image>,
    pub label : String,
    pub name: String,
    pub popularity : u8,
    pub release_date: String,
    pub release_date_precision : String,
    pub restrictions : Restriction
    pub tracks: Paging<Tracks>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artist_full {
    pub external_urls : Vec<Ext_URL>,
    pub followers : Followers,
    pub genres : Vec<String>,
    pub href : String,
    pub id : String,
    pub images : Vec<Image>,
    pub name : String,
    pub popularity : u8,
    pub type: String,
    pub uri : String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artist_simp {
    pub external_urls : Vec<Ext_URL>,
    pub href : String,
    pub id : String,
    pub name : String,
    pub type: String,
    pub uri : String,
}

/// Album structure that represents a spotify album, with its name, Spotify ID, and a Vector of Tracks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Album_simp {
    pub album_type : String,
    pub artists : Vec<Artist>
    pub name: String,
    pub id: String,
    pub tracks: Option<Vec<Track>>,
}
/// Artist structure that represents an Artist, with its name, Spotify ID, and a Vector of Albums
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artist {
    pub name: String,
    pub id: String,
    pub albums: Option<Vec<Album>>,
}
/// Playlist structure that represents a Playlist, with its name, Spotify ID, and a Vector of Tracks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub name: String,
    pub id: String,
    pub tracks: Option<Vec<Track>>,
}