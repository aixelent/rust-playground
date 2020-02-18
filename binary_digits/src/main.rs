use std::fmt;

struct Length(i32);

impl fmt::Binary for Length {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.0;
        write!(f, "{:b}", val)
    }
}

fn main() {
    let a = Length(5);
    let b = Length(50);
    let c = Length(9000);

    println!("{:b}", a);        // 101
    println!("{:b}", b);        // 110010
    println!("{:b}", c);        // 10001100101000
    println!("");

    //
    // for i in 0..100 {
    //     println!("{:b}", i);
    // }

    let n = 5;
    match n {
        _ => println!("{:b}", n),
    }
}