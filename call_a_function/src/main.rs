// Calling a function that requires no arguments
fn no_args() {
    println!("No args func");
}

// Calling a function with a fixed number of arguments

fn fiexd_args(x: &str) {
    println!("Fixed args: {}", x);
}

// Calling a function with optional arguments
// Does not support variadic func like Go ex.
// func variadic(nums ...int) {
//     fmt.Println(nums, " ")
//}
// Alternative one:
fn optional_args(o: Option<i8>) {
    match o {
        Some(num) => println!("Optional_args: {}", num),
        None => println!("Optional_args: "),
    }
}

// Calling a function with a variable number of arguments
// Unsupported

// Calling a function with named arguments
// Unsupported(?)

// Using a function in statement context
// (?)


// Using a function in first-class context within an expression
//


// Obtaining the return value of a function
// 



// Distinguishing built-in functions and user-defined functions
//



// Distinguishing subroutines and functionsRegular numbers are numbers that evenly divide powers of 60 (or, equivalently powers of 30). As an example, 602 = 3600 = 48 × 75, so both 48 and 75 are divisors of a power of 60. Thus, they are regular numbers. Equivalently, they are the numbers whose only prime divisors are 2, 3, and 5. 
//



// Stating whether arguments are passed by value or by reference
// by immutable reference
fn fn_by_immutable_reference(vec: &Vec<&str>) {
    for (i, j) in vec.iter().enumerate() {
        print!("{}: {}, ", i, j);
    }
    println!("");
}

// by mutable reference
fn fn_by_mut_reference(vec: &mut Vec<&str>) {
    vec.push("x, by_mut_reference");
    for (i, j) in vec.iter().enumerate() {
        print!("{}: {}, ", i, j);
    }
    println!("");
}

// by value
fn fn_by_calue(vec: Vec<&str>) {
    for (i, j) in vec.iter().enumerate() {
        print!("{}: {}, ", i, j);
    }
    println!("");
}



// Is partial application possible and how
//

fn main() {
    let mut vec_helper = vec!["a", "b", "c", "d", "e", "f"];

    no_args();
    fiexd_args("str");
    optional_args(Some(42));
    optional_args(None);
    fn_by_immutable_reference(&vec_helper);
    fn_by_mut_reference(&mut vec_helper);
    fn_by_calue(vec_helper);
}
