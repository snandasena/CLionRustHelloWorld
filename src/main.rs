extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the secreat number");

    let secreat_number = rand::thread_rng().gen_range(1, 101);

    println!("The secreate number {}", secreat_number);

    println!("Please input ur guess");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Faild to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number");

    println!("You guessed : {}", guess);

    match guess.cmp(&secreat_number) {
        Ordering::Less => println!("Too Small"),
        Ordering::Equal => println!("U win"),
        Ordering::Greater => println!("Too Big"),
    }
}

