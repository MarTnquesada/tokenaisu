use tokenaisu::{tokenize, Language};

fn main() {
    println!("Hello, world!");
    println!("{}", tokenize("Hello, world!", Language::En));
}
