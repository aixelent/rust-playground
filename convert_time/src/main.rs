extern crate chrono;

use chrono::*;

fn main() {
    let local_time = Local::now();
    println!("{}", local_time);    

    let local_to_utc = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
    println!("{}", local_to_utc);

    let dt = DateTime::<Utc>::date(&local_to_utc);
    println!("{}", dt);
}
