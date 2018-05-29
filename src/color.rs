use std::cmp;
use std::fmt;

pub const NUM_COLORS: u32 = 9;

#[derive(Clone, Debug)]
pub enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Indigo,
    Purple,
    Black,
    White,
}

impl Color {
    pub fn new(n: u32) -> Option<Color> {
        match n {
            0 => Some(Color::Red),
            1 => Some(Color::Orange),
            2 => Some(Color::Yellow),
            3 => Some(Color::Green),
            4 => Some(Color::Blue),
            5 => Some(Color::Indigo),
            6 => Some(Color::Purple),
            7 => Some(Color::Black),
            8 => Some(Color::White),
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