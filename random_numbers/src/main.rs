extern crate rand;

use rand::Rng;

fn random_range(x: i64, y: i64) {
    let mut rand_value = rand::thread_rng();
    println!("{}", rand_value.gen_range(x, y));
}

fn main() {
    random_range(0, 11);
    random_range(100, 201);
}
