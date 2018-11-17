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

use interface::util::event::{Event, Events};

pub struct App<'a> {
    pub size: Rect,
    pub items: Vec<&'a str>,
    pub  selected: Option<usize>,
    pub info_style: Style,
    pub warning_style: Style,
    pub error_style: Style,
    pub critical_style: Style,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            size: Rect::default(),
            items: vec![
                "Justice", "Michel Sardou", "Beethoven", "Michael Jackson", "Johnny Halliday"
            ],
            selected: None,
            info_style: Style::default().fg(Color::White),
            warning_style: Style::default().fg(Color::Yellow),
            error_style: Style::default().fg(Color::Magenta),
            critical_style: Style::default().fg(Color::Red),
        }
    }

    pub fn advance(&mut self) {
        
    }
}
