use color::Color;
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

pub fn convert(guess: String) -> Vec<Peg> {
    let mut converted: Vec<Peg> = Vec::new();
    for c in guess.chars() {
        match c.to_lowercase().to_string().as_ref() {
            "r" => converted.push(Peg::new(Color::Red)),
            "o" => converted.push(Peg::new(Color::Orange)),
            "y" => converted.push(Peg::new(Color::Yellow)),
            "g" => converted.push(Peg::new(Color::Green)),
            "l" => converted.push(Peg::new(Color::Blue)),
            "i" => converted.push(Peg::new(Color::Indigo)),
            "p" => converted.push(Peg::new(Color::Purple)),
            "b" => converted.push(Peg::new(Color::Black)),
            "w" => converted.push(Peg::new(Color::White)),
            _ => {},
        }
    }
    converted
}