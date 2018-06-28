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
    pub fn new(n: u32) -> Color {
        use color::Color::*;
        match n {
            0 => Red,
            1 => Orange,
            2 => Yellow,
            3 => Green,
            4 => Blue,
            5 => Indigo,
            6 => Purple,
            7 => Black,
            _ => White
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
