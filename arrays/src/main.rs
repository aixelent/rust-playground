fn main() {
    let arr = [1, 3, 5, 7, 9, 11, 13, 15, 17, 18, 21];
    let arr2 = [1, 3, 5, 7, 9, 11, 13, 15, 17, 18, 21];
    println!("Arr length: {}\n", arr.len());

    for i in 0..arr.len() {
        print!("{} : {}\n", i, arr[i]);
    }
    println!();

    // or
    for i in arr2.iter() {
        print!("{}, ", i);
    }
    println!();

    // dynamic array Vec
    let mut v = vec![3, 6];
    v.push(9);

    for i in v.iter() {
        print!("{}, ", i);
    }
}
