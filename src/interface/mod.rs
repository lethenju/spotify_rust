/*  
    @Author Julien LE THENO
    @mod Tui : handles the Text user interface
*/

use std::io;
pub mod util;
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use tui::layout::{ Rect};
use tui::style::{Color, Style};
use tui::Terminal;

use interface::util::{Event, Events};

pub struct App<'a> {
    pub size: Rect,
    pub items: Vec<&'a str>,
    pub selected: Option<usize>,
}

impl<'a> App<'a> {
    pub fn new(_items :Vec<&'a str>) -> App<'a> {
        App {
            size: Rect::default(),
            items: _items,
            selected: Some(0),
        }
    }
    pub fn advance(&mut self) {
    }
}

pub struct Tracks<'a> {
    pub size: Rect,
    pub items: Vec<&'a str>,
    pub selected: Option<usize>,
}

impl<'a> Tracks<'a> {
    pub fn new() -> Tracks<'a> {
        Tracks {
            size: Rect::default(),
            items: Vec::new(),
            selected: Some(0),
        }
    }
    pub fn advance(&mut self) {
    }
}

