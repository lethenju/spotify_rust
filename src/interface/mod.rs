/*  
    @Author Julien LE THENO
    @mod Tui : handles the Text user interface
*/
extern crate spotify_api;

pub mod util;
use tui::layout::Rect;

pub struct Albums {
    pub size: Rect,
    pub albums: Vec<spotify_api::Album>,
    pub selected: usize,
    pub active: bool
}

impl<'a> Albums {
    pub fn new(_albums: Vec<spotify_api::Album>) -> Albums {
        Albums {
            size: Rect::default(),
            albums: _albums,
            selected: 0,
            active: true,
        }
    }
    pub fn add_albums(&mut self,_albums:&mut Vec<spotify_api::Album>) {
        self.albums.append(_albums);
    }

    pub fn get_selected(&self) -> Option<usize> {
        if self.active{
            Some(self.selected)
        } else {
            None
        }
    }
    pub fn get_selected_album(&self) -> Option<&spotify_api::Album> {
        Some(&self.albums[self.selected])
    }
    pub fn advance(&mut self) {}
}

pub struct Tracks {
    pub size: Rect,
    pub tracks: Vec<spotify_api::Track>,
    pub selected: usize,
    pub active: bool,
}
impl<'a> Tracks {
    pub fn new() -> Tracks {
        Tracks {
            size: Rect::default(),
            tracks: Vec::new(),
            selected: 0,
            active: false,
        }
    }

    pub fn add_tracks(&mut self, _tracks: &mut Vec<spotify_api::Track>) {
        self.tracks.append(_tracks);
    }
    pub fn clear_tracks(&mut self) {
        //self.items.remove(self.items.len());
        self.tracks.clear();
    }

    pub fn get_selected(&self) -> Option<usize> {
        if self.active{
            Some(self.selected)
        } else {
            None
        }
    }
    pub fn get_selected_track(&self) -> Option<&spotify_api::Track> {
        if self.active{
            Some(&self.tracks[self.selected])
        } else {
            None
        }
    }


    pub fn advance(&mut self) {}
}
