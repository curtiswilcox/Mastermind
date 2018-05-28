extern crate rand;

mod color;
mod peg;

use color::Color;
use rand::{thread_rng, Rng};
use std::io;
use io::Write;
use peg::Peg;


fn main() {
    let answer = get_answer();
    let mut guess = String::new();
    print!("Enter your guess: ");
    io::stdout().flush().expect("Flush failed...");
    match io::stdin().read_line(&mut guess) {
        Ok(_) => {}
        Err(err) => println!("Uh oh! {}", err),
    }
    let guess_convert = peg::convert(guess);
    check_answer(&guess_convert, &mut answer.clone());
}

fn get_answer() -> Vec<Peg> {
    let mut pegs: Vec<Peg> = Vec::new();
    for i in 0..4 {
        pegs.push(Peg::new(Color::new(rand::thread_rng().gen_range(0, 8)).unwrap()));
        print!("{:?} ", &pegs[i]);
    }
    println!();
    pegs
}

fn check_answer(guess: &Vec<Peg>, answer: &mut Vec<Peg>) {
    let mut result: Vec<u32> = Vec::new();
    let mut subtract = 0;

    for i in 0..guess.len() {
        if guess[i].color() == answer[i].color() && answer[i].found() {
            result.push(1);
            answer[i].find()
        }
    }

    for i in 0..guess.len() {
        if answer.contains(&guess[i]) {
            let index = answer.iter().position(|c| {
                c.to_string() == guess[i].to_string()
            }).unwrap(); // TODO first check every index, then check for contains
            if i - subtract == index {
                result.push(1);
            } else {
                result.push(0);
            }
            answer.remove(index);
            subtract += 1;
        } else {
            result.push(2)
//            continue
        }
    }
//    thread_rng().shuffle(&mut result);
    for r in result {
        print!("{} ", r)
    }
}
