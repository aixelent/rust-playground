fn fib(n: i64) -> i64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n-2) + fib(n-1)
    }
}

fn main() {
    println!("fib(7) = {}", fib(10));
    println!("fib_nth(0)  = {}", fib(0));
    println!("fib_nth(1)  = {}", fib(1));
    println!("fib_nth(12) = {}", fib(12));
    println!("fib_nth(15) = {}", fib(15));
    println!("fib_nth(19) = {}", fib(19));
    println!("fib_nth(25) = {}", fib(25));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fib_nth() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(5), 5);
        assert_eq!(fib(7), 13);
        assert_eq!(fib(9), 34);
        assert_eq!(fib(10), 55);
    }
}
