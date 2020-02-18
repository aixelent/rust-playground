fn factorial(x: i64) -> i64 {
    match x {
        0 => 1,
        _ => x * factorial(x - 1),
    }
} 

fn main() {
    println!("{:}", factorial(4));
    println!("{:}", factorial(5));
    println!("{:}", factorial(10));
}

#[test]
fn factorial_test() {
    assert_eq!(factorial(0), 1);
    assert_eq!(factorial(3), 6);
    assert_eq!(factorial(5), 120);
}