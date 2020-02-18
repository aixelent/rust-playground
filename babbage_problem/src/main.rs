fn main() {
    let m: i32 = 1_000_000;
    let eq: i32 = 269_696;

    for s in (1..).find(|x| x * x % 1_000_000 == eq) {
        println!("{}", s);
    }

    //
    let mut x = 0;
    while x * x % m != eq {
        x += 1;
    }
    println!("{}", x);
}
