/*  
    @Author Julien LE THENO
    @mod Tui : handles the Text user interface
*/
extern crate spotify_cli;

pub mod util;
use tui::layout::Rect;

pub struct Albums {
    pub size: Rect,
    pub album_name: Vec<String>,
    pub album_id: Vec<String>,
    pub selected: Option<usize>,
}

impl<'a> Albums {
    pub fn new(_album_name: Vec<String>, _album_id: Vec<String>) -> Albums {
        Albums {
            size: Rect::default(),
            album_name: _album_name,
            album_id: _album_id,
            selected: Some(0),
        }
    }
    pub fn add_albums(&mut self, _album_name: &mut Vec<String>, _album_id: &mut Vec<String>) {
        self.album_name.append(_album_name);
        self.album_id.append(_album_id);
    }

    pub fn advance(&mut self) {}
}

pub struct Tracks {
    pub size: Rect,
    pub tracks: Vec<spotify_cli::Track>,
    pub selected: Option<usize>,
}
impl<'a> Tracks {
    pub fn new() -> Tracks {
        Tracks {
            size: Rect::default(),
            tracks: Vec::new(),
            selected: None,
        }
    }

    pub fn add_tracks(&mut self, _tracks: &mut Vec<spotify_cli::Track>) {
        self.tracks.append(_tracks);
    }
    pub fn clear_tracks(&mut self) {
        //self.items.remove(self.items.len());
        self.tracks.clear();
    }

    pub fn advance(&mut self) {}
}
