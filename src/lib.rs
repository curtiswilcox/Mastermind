#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]

extern crate rand;

mod color;
mod peg;

use color::Color;
use rand::Rng;
use std::io;
use io::Write;
pub use peg::Peg;

pub enum Correctness {
    Partial, Total
}

pub struct GameState {
    answer: Vec<Peg>
}

pub fn step(state: &mut GameState, guess: Vec<Peg>) -> Option<Vec<Correctness>> {
    unimplemented!()
}
