/*  
    @Author Julien LE THENO
    @mod Tui : handles the Text user interface
*/

use std::io;
pub mod util;
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::Terminal;

use interface::util::{Event, Events};

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
    pub track_name: Vec<String>,
    pub track_id: Vec<String>,
    pub selected: Option<usize>,
}

impl<'a> Tracks {
    pub fn new() -> Tracks {
        Tracks {
            size: Rect::default(),
            track_name: Vec::new(),
            track_id: Vec::new(),
            selected: None,
        }
    }

    pub fn add_tracks(&mut self, _track_name: &mut Vec<String>, _track_id: &mut Vec<String>) {
        self.track_name.append(_track_name);
        self.track_id.append(_track_id);
    }
    pub fn clear_tracks(&mut self) {
        //self.items.remove(self.items.len());
        self.track_name.clear();
        self.track_id.clear();
    }

    pub fn advance(&mut self) {}
}
