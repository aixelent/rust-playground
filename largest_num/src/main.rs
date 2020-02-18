fn largest_num<T: PartialOrd + Copy>(x: &[T]) -> T {
    let mut largest = x[0];

    for &item in x.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let val_items = [1, 0, 3_933_001, 99, 118, 09, 111];
    println!("Largest value: {}", largest_num(&val_items));

    let char_items = ['a', 'b', '0', 'x', 'z', 'ż'];
    println!("Largest value: {}", largest_num(&char_items));
}
