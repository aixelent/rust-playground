#[warn(non_snake_case)]
fn ab(a: i64, b: i64) -> i64 {
    a + b
}

fn main() {
    println!("{}", ab(4, 5));
    println!("{}", ab(0, 0));
    println!("{}", ab(1, 0));

}

#[test]
fn ab_test() {
    assert_eq!(ab(3, 4), 7);
    assert_eq!(ab(1, 2), 3);
    assert_eq!(ab(7, 4), 11);
    assert_eq!(ab(0, 9), 9);
    assert_eq!(ab(13, 0), 13);
    assert_eq!(ab(0, 0), 0);
}