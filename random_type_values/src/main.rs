extern crate rand;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let rand_i32: i32 = rng.gen::<i32>();
    let rand_u8: u8 = rng.gen::<u8>();
    let rand_tuple = rng.gen::<(i32, char, u8)>();
    let rand_arr = rng.gen::<[u8;5]>();

    println!("Random tuple: {:?}", rand_tuple);
    println!("Five random array value: {:?}", rand_arr);
    println!("Random u8 value: {:?}", rand_u8);
    println!("Random i32 value: {:?}", rand_i32);
}
