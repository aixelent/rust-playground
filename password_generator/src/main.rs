extern crate rand;

use rand::Rng;

fn gen() {
    const CHARS: &[u8] = b"abcdefghijklmnopqrstuwxyzABCDEFGHIJKLMNOPQRSTUWXYZ1234567890!@#$%^&*()_+{}|:<>?-=[];',./";
    const LEN: usize = 30;
    
    let mut rng = rand::thread_rng();
    let pass: String = (0..LEN).map(|_| { let range = rng.gen_range(0, CHARS.len()); CHARS[range] as char }).collect();

    println!("{:?}", pass);
}
fn main() {
    gen();
}
