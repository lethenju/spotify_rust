/*  
    @Author Julien LE THENO
    @mod Tui : handles the Text user interface
*/
extern crate spotify_rust;

pub mod util;
use tui::layout::Rect;

pub struct Albums {
    pub size: Rect,
    pub albums: Vec<spotify_rust::Album>,
    pub selected: Option<usize>,
}

impl<'a> Albums {
    pub fn new(_albums: Vec<spotify_rust::Album>) -> Albums {
        Albums {
            size: Rect::default(),
            albums: _albums,
            selected: Some(0),
        }
    }
    pub fn add_albums(&mut self,_albums:&mut Vec<spotify_rust::Album>) {
        self.albums.append(_albums);
    }

    pub fn advance(&mut self) {}
}

pub struct Tracks {
    pub size: Rect,
    pub tracks: Vec<spotify_rust::Track>,
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

    pub fn add_tracks(&mut self, _tracks: &mut Vec<spotify_rust::Track>) {
        self.tracks.append(_tracks);
    }
    pub fn clear_tracks(&mut self) {
        //self.items.remove(self.items.len());
        self.tracks.clear();
    }

    pub fn advance(&mut self) {}
}
