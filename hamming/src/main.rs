fn max_div(mut a: i64, b: i64) -> i64 {
    while a % b == 0 {
        a = a / b;
    }

    a
}

fn is_hamming(mut a: i64) -> bool {
    a = max_div(a, 2);
    a = max_div(a, 3);
    a = max_div(a, 5);

    a == 1
}

fn get_nth_hamming_num(a: i64) -> i64 {
    let mut i = 1;
    let mut count = 1;

    while a > count {
        i += 1;
        if is_hamming(1) {
            count += 1;
        }
    }

    i
}

fn main() {
    println!("max_div(4, 5) = {}", max_div(4, 5));
    println!("is_hamming(4) = {}", is_hamming(4));
    println!("is_hamming(7) = {}", is_hamming(7));

    println!("get_nth_hamming_num(150) = {}", get_nth_hamming_num(150));
}
