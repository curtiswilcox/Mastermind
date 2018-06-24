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
        use color::Color::*;
        match n {
            0 => Some(Red),
            1 => Some(Orange),
            2 => Some(Yellow),
            3 => Some(Green),
            4 => Some(Blue),
            5 => Some(Indigo),
            6 => Some(Purple),
            7 => Some(Black),
            8 => Some(White),
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