extern crate rand;

use rand::Rng;
use rand::distributions::{Distribution, Standard};

#[derive(Debug)]
struct Rar {
    x: i32,
    y: char,
    z: u8,
    o: bool,
    e: (u8, i8, u8),
}

impl Distribution<Rar> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Rar {
        let (rand_x, rand_y, rand_z, rand_o, rand_e) = rng.gen();
        Rar {
            x: rand_x,
            y: rand_y,
            z: rand_z,
            o: rand_o,
            e: rand_e, 
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let rand_rar: Rar = rng.gen(); 
    println!("Rand Rar:\ni32: {:?}\nchar: {:?}\nu8: {:?}\nbool: {:?}\n(u8, i8, u8): {:?}", rand_rar.x, rand_rar.y, rand_rar.z, rand_rar.o, rand_rar.e);
}