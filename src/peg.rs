use color::Color;
//use std::fmt;

//#[derive(Clone, Debug)]
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
        self.color
    }

    pub fn found(&self) -> bool {
        self.found
    }

    pub fn find(&self) {
        self.found = true;
    }
}

pub fn convert(guess: String) -> Vec<Peg> {
    let mut converted: Vec<Peg> = Vec::new();
    for c in guess.chars() {
        match c.to_lowercase().to_string().as_ref() {
            "p" => converted.push(Peg::new(Color::Pink)),
            "o" => converted.push(Peg::new(Color::Orange)),
            "y" => converted.push(Peg::new(Color::Yellow)),
            "g" => converted.push(Peg::new(Color::Green)),
            "l" => converted.push(Peg::new(Color::Blue)),
            "u" => converted.push(Peg::new(Color::Purple)),
            "b" => converted.push(Peg::new(Color::Black)),
            "w" => converted.push(Peg::new(Color::White)),
            _ => {},
        }
    }
    converted
}