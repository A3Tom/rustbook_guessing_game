use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("Guess a number between 1 & 100");

    let correct_number = rand::thread_rng().gen_range(1..=100);

    println!("The right number is {:#} btw", correct_number);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Canny read what ye wrote mate");

    let guess: u32 = guess.trim()
        .parse()
        .expect("Haul, geez a number!");

    println!("You guessed: {:#}", guess);

    match guess.cmp(&correct_number) {
        Ordering::Less => println!("Too low"),
        Ordering::Equal => println!("Fuck yeah you win, well played you nutter... how long did this take you?"),
        Ordering::Greater => todo!("Too high"),
    }
}
