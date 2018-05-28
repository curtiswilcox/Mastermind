use std::cmp;
use std::fmt;

#[derive(Clone, Debug)]
pub enum Color {
    Pink,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Black,
    White,
    Found, // for checking purposes
}

impl Color {
    pub fn new(n: u32) -> Option<Color> {
        match n {
            0 => Some(Color::Pink),
            1 => Some(Color::Orange),
            2 => Some(Color::Yellow),
            3 => Some(Color::Green),
            4 => Some(Color::Blue),
            5 => Some(Color::Purple),
            6 => Some(Color::Black),
            7 => Some(Color::White),
            _ => None
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl cmp::PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        if self.to_string() == other.to_string() {
            true
        } else {
            false
        }
    }
}