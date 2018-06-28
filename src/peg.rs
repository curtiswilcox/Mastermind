use rand::{self, Rng};
use color::{self, Color};
use std::cmp;
use std::fmt;

#[derive(Clone)]
pub struct Peg {
    color: Color,
    found: bool,
}

impl Peg {
    pub fn new(color: Color) -> Peg {
        Peg {
            color,
            found: false,
        }
    }

    pub fn new_random() -> Peg {
        Peg {
            color: Color::new(rand::thread_rng().gen_range(0, color::NUM_COLORS)),
            found: false,
        }
    }

    pub fn color(&self) -> Color {
        self.color.clone()
    }

    pub fn found(&self) -> bool {
        self.found
    }

    pub fn find(&mut self) {
        self.found = true;
    }
}

impl cmp::PartialEq for Peg {
    fn eq(&self, other: &Peg) -> bool {
        self.color == other.color
    }
}

impl fmt::Debug for Peg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.color, self.found)
    }
}

impl fmt::Display for Peg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.color)
    }
}

impl Into<Peg> for char {
    fn into(self) -> Peg {
        use Color::*;
        match self.to_lowercase().to_string().as_ref() {
            "r" => Peg::new(Red),
            "o" => Peg::new(Orange),
            "y" => Peg::new(Yellow),
            "g" => Peg::new(Green),
            "l" => Peg::new(Blue),
            "i" => Peg::new(Indigo),
            "p" => Peg::new(Purple),
            "b" => Peg::new(Black),
            "w" => Peg::new(White),
            _ => Peg::new(Black),
        }
    }
}

pub fn convert(guess: String) -> Vec<Peg> {
    let mut converted: Vec<Peg> = Vec::new();
    for c in guess.chars() {
        converted.push(c.into());
    }
    converted
}
