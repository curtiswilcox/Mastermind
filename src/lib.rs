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
    answer: Vec<Peg>,
    current_turn: u32,
}


impl GameState {

    pub fn new(num_pegs: usize) -> GameState {
        let answer: Vec<Peg> = Vec::with_capacity(num_pegs)
            .into_iter()
            .map(|x: usize| Peg::new_random())
            .collect();

        GameState {
            answer,
            current_turn: 0,
        }
    }

    pub fn step(&mut self, guess: Vec<Peg>) -> Vec<Correctness> {

        self.current_turn += 1;

        self.make_guess_response(guess)

    }

    fn make_guess_response(&mut self, mut guess: Vec<Peg>) -> Vec<Correctness> {
        use Correctness::*;

        let mut answer = self.answer.clone();
        let mut result: Vec<Correctness> = Vec::new();

        for i in 0..guess.len() {
            if guess[i].color() == answer[i].color() && !answer[i].found() {
                result.push(Total);
                answer[i].find();
                guess[i].find();
            }
        }

        guess.retain(|p| !p.found() );
        answer.retain(|p| !p.found() );

        for g in guess.iter() {
            answer.iter()
            .position(|p| p.color() == g.color())
            .map(|index| {
                if !answer[index].found() {
                    result.push(Partial);
                    answer[index].find();
                }
            });
        }

        result
    }
}
