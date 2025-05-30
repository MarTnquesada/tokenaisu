use tokenaisu::{Language, tokenize};

fn main() {
    println!("Hello, world!");
    println!("{}", tokenize("Hello, world!", Language::En, false));
}
