use clap::Parser;
use std::process;
use tokenaisu::moses::{Language, moses_tokenize_file};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    language: Language,

    #[arg(short, long)]
    input_file_path: String,

    #[arg(short, long)]
    output_file_path: String,
}

fn main() {
    let args = Args::parse();

    if let Err(e) = moses_tokenize_file(
        &args.input_file_path,
        &args.output_file_path,
        Language::En,
        true,
        false,
        &[],
    ) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
