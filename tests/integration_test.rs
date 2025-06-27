use std::fs;
use tokenaisu::moses::{Language, moses_tokenize_file};

#[test]
fn tokenize_file() {
    moses_tokenize_file(
        "tests/untokenized_text.txt",
        "tests/tokenized_text_test.txt",
        Language::En,
        true,
        false,
        &[],
    )
    .unwrap();
    let text_data = fs::read_to_string("tests/tokenized_text_test.txt").unwrap();
    let ground_truth = fs::read_to_string("tests/tokenized_text.txt").unwrap();
    fs::remove_file("tests/tokenized_text_test.txt").unwrap();
    assert_eq!(text_data, ground_truth);
}
