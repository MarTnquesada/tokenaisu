use tokenaisu::{Language, moses_tokenize_line};

fn main() {
    println!("Hello, world!");
    println!(
        "{}",
        moses_tokenize_line("Hello, world!", Language::En, true)
    );
}
