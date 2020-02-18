extern crate reqwest;

// http://rosettacode.org/wiki/HTTP

fn main() {
    let req = reqwest::get("https://google.com").expect("").text().expect("");
    println!("Response from google.com:\n{}", req);
}
