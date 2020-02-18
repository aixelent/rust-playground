extern crate rand;

use std::io;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::process;

fn main() {
    // generating random num
    let mut rng = thread_rng();
    let secret_value = rng.gen_range(0, 10);

    loop {
        println!("Guess number: ");

        // reading input
        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess).expect("Incorrect value");

        let guess: u16 = guess.trim().parse().expect("Number needed");
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_value) {
            Ordering::Equal => { println!("Well guessed!\n"); process::exit(1); },
            Ordering::Greater => println!("Too big\n"),
            Ordering::Less => println!("Too small\n")
        }
    }
}
  