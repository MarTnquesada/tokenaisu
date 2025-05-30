use regex::Regex;
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub enum Language {
    En,
    Es,
    Fi,
    Sv,
    Tdt,
    Ca,
    Fr,
    It,
    Ga,
    So,
}

pub fn tokenize(text: &str, language: Language) -> String {
    // ## This is an example of how the non-breaking prefix data structure looks, but it needs to be implemented ## /
    let mut nonbreaking_prefix: HashMap<String, i32> = HashMap::new();
    nonbreaking_prefix.insert("St".to_string(), 1);
    nonbreaking_prefix.insert("vs".to_string(), 1);
    // ##  ## /
    // Remove railing newline character
    let mut tokenized_text: String = String::from(text.trim_end_matches('\n'));
    // Add spaces at the beginning and end of the text
    tokenized_text = format!(" {tokenized_text} ");
    // Replace all sequences of whitespaces with a single whitespace
    tokenized_text = tokenized_text
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");
    // Remove ASCII characters 0-31 (always works because the first 128 ASCII chars match the first 128 unicode chars)
    tokenized_text = tokenized_text.chars().filter(|&ch| ch as u8 > 31).collect();
    // Capture protected patterns and replace them with unique substitution strings
    // TODO
    // Replace all sequences of whitespaces with a single whitespace (again)
    tokenized_text = tokenized_text
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");
    // Removes the starting and ending spaces added previously
    tokenized_text = tokenized_text.trim_matches(' ').to_owned();
    // Separate out all other special characters depending on the language
    if language == Language::Fi || language == Language::Sv {
        // In Finnish and Swedish, the colon can be used inside words as an apostrophe-like character:
        let re_general = Regex::new(r"([^\p{L}\p{N}\s\.\:\'\`\,\-])").unwrap();
        tokenized_text = re_general.replace_all(&tokenized_text, " $1 ").to_string();
        // If a colon is not immediately followed by lower-case characters, separate it out anyway
        let re_colon = Regex::new(r"(:)(?=$|[^\p{Ll}])").unwrap();
        tokenized_text = re_colon.replace_all(&tokenized_text, " $1 ").to_string();
    } else if language == Language::Tdt {
        // # In Tetun, the apostrophe can be used inside words as an apostrophe-like character:
        let re_general = Regex::new(r"([^\p{L}\p{N}\s\.\'\`\,\-])").unwrap();
        tokenized_text = re_general.replace_all(&tokenized_text, " $1 ").to_string();
        // If an apostrophe is not immediately followed by lower-case characters, separate it out anyway
        let re_apostrophe = Regex::new(r"(')(?=$|[^\p{Ll}])").unwrap();
        tokenized_text = re_apostrophe
            .replace_all(&tokenized_text, " $1 ")
            .to_string();
    } else if language == Language::Ca {
        // In Catalan, the middle dot can be used inside words:
        let re_general = Regex::new(r"([^\p{L}\p{N}\s\.\u{00B7}'\`\,\-])").unwrap();
        tokenized_text = re_general.replace_all(&tokenized_text, " $1 ").to_string();
        // If a middot is not immediately followed by lower-case characters, separate it out anywa
        let re_middot = Regex::new(r"(\u{00B7})(?=$|[^\p{Ll}])").unwrap();
        tokenized_text = re_middot.replace_all(&tokenized_text, " $1 ").to_string();
    } else {
        let re_general = Regex::new(r"([^\p{L}\p{N}\s\.\'\`\,\-])").unwrap();
        tokenized_text = re_general.replace_all(&tokenized_text, " $1 ").to_string();
    }
    // Optional aggressive hyphen splitting
    // TODO
    // Multi-dot tagging
    let re_new_multi_dot = Regex::new(r"\.([\.]+)").unwrap();
    tokenized_text = re_new_multi_dot
        .replace_all(&tokenized_text, " DOTMULTI$1")
        .to_string();
    let re_dotmulti_left = Regex::new(r"DOTMULTI\.").unwrap();
    let re_dotmulti_plus_nondot = Regex::new(r"DOTMULTI\.([^\.])").unwrap();
    let re_dotmulti_expand = Regex::new(r"DOTMULTI\.").unwrap();
    while re_dotmulti_left.is_match(&tokenized_text) {
        // Replace DOTMULTI. followed by non-dot with DOTDOTMULTI plus that character
        tokenized_text = re_dotmulti_plus_nondot
            .replace_all(&tokenized_text, "DOTDOTMULTI $1")
            .to_string();
        // Replace any remaining DOTMULTI. with DOTDOTMULTI
        tokenized_text = re_dotmulti_expand
            .replace_all(&tokenized_text, "DOTDOTMULTI")
            .to_string();
    }
    // Separate out "," except if within numbers (5,300)
    let re_comma_after_non_numeric = Regex::new(r"([^\p{N}]),").unwrap();
    tokenized_text = re_comma_after_non_numeric
        .replace_all(&tokenized_text, "$1 , ")
        .to_string();
    let re_comma_before_non_numeric = Regex::new(r",([^\p{N}])").unwrap();
    tokenized_text = re_comma_before_non_numeric
        .replace_all(&tokenized_text, " , $1")
        .to_string();
    // Separate "," after a number if it's the end of a sentence
    let re_comma_after_number_end_of_sentence = Regex::new(r"([\p{N}]),$").unwrap();
    tokenized_text = re_comma_after_number_end_of_sentence
        .replace_all(&tokenized_text, "$1 ,")
        .to_string();
    // Split contractions
    match language {
        Language::En => {
            // Split contractions right
            // Non-alpha + apostrophe + non-alpha -> add spaces around apostrophe
            let re_space_around_aphostrophe = Regex::new(r"([^\p{L}])[']([^\p{L}])").unwrap();
            tokenized_text = re_space_around_aphostrophe
                .replace_all(&tokenized_text, "$1 ' $2")
                .to_string();
            // Non-alpha/non-numeric + apostrophe + alpha -> space before apostrophe
            let re_space_before_aphostrophe = Regex::new(r"([^\p{L}\p{N}])[']([\p{L}])").unwrap();
            tokenized_text = re_space_before_aphostrophe
                .replace_all(&tokenized_text, "$1 ' $2")
                .to_string();
            // Alpha + apostrophe + non-alpha -> space after apostrophe
            let re_space_after_aphostrophe = Regex::new(r"([\p{L}])[']([^\p{L}])").unwrap();
            tokenized_text = re_space_after_aphostrophe
                .replace_all(&tokenized_text, "$1 ' $2")
                .to_string();
            // Alpha + apostrophe + alpha -> space before apostrophe (e.g., "don't" -> "don ' t")
            let re_space_before_aphostrophe_alpha = Regex::new(r"([\p{L}])[']([\p{L}])").unwrap();
            tokenized_text = re_space_before_aphostrophe_alpha
                .replace_all(&tokenized_text, "$1 '$2")
                .to_string();

            // Special case for "1990's" - numeric + apostrophe + 's'
            let re_numeric_apostrophe = Regex::new(r"([\p{N}])[']([s])").unwrap();
            tokenized_text = re_numeric_apostrophe
                .replace_all(&tokenized_text, "$1 '$2")
                .to_string();
        }
        Language::Fr | Language::It | Language::Ga | Language::Ca => {
            // Split contractions left
            // Non-alpha + apostrophe + non-alpha -> add spaces around apostrophe
            let re_space_around_apostrophe = Regex::new(r"([^\p{L}])[']([^\p{L}])").unwrap();
            tokenized_text = re_space_around_apostrophe
                .replace_all(&tokenized_text, "$1 ' $2")
                .to_string();
            // Non-alpha + apostrophe + alpha -> space before apostrophe
            let re_space_before_apostrophe = Regex::new(r"([^\p{L}])[']([\p{L}])").unwrap();
            tokenized_text = re_space_before_apostrophe
                .replace_all(&tokenized_text, "$1 ' $2")
                .to_string();
            // Alpha + apostrophe + non-alpha -> space after apostrophe
            let re_space_after_apostrophe = Regex::new(r"([\p{L}])[']([^\p{L}])").unwrap();
            tokenized_text = re_space_after_apostrophe
                .replace_all(&tokenized_text, "$1 ' $2")
                .to_string();
            // Alpha + apostrophe + alpha -> space after apostrophe (e.g., "l'eau" -> "l' eau")
            let re_space_before_aphostrophe_alpha = Regex::new(r"([\p{L}])[']([\p{L}])").unwrap();
            tokenized_text = re_space_before_aphostrophe_alpha
                .replace_all(&tokenized_text, "$1' $2")
                .to_string();
        }
        Language::So | Language::Tdt => {
            // Don't split glottals (no alpha + apostrophe + alpha rule)
            // Non-alpha + apostrophe + non-alpha -> add spaces around apostrophe
            let re_space_around_apostrophe = Regex::new(r"([^\p{L}])[']([^\p{L}])").unwrap();
            tokenized_text = re_space_around_apostrophe
                .replace_all(&tokenized_text, "$1 ' $2")
                .to_string();
            // Non-alpha + apostrophe + alpha -> space before apostrophe
            let re_space_before_apostrophe = Regex::new(r"([^\p{L}])[']([\p{L}])").unwrap();
            tokenized_text = re_space_before_apostrophe
                .replace_all(&tokenized_text, "$1 ' $2")
                .to_string();
            // Alpha + apostrophe + non-alpha -> space after apostrophe
            let re_space_afer_apostrophe = Regex::new(r"([\p{L}])[']([^\p{L}])").unwrap();
            tokenized_text = re_space_afer_apostrophe
                .replace_all(&tokenized_text, "$1 ' $2")
                .to_string();
        }
        _ => {
            // Default: add spaces around all apostrophes
            let re_apostrophe_space = Regex::new(r"'").unwrap();
            tokenized_text = re_apostrophe_space
                .replace_all(&tokenized_text, " ' ")
                .to_string();
        }
    }
    // Word tokenization
    let words: Vec<&str> = text.split_whitespace().collect();
    tokenized_text = String::new();
    let re_period = Regex::new(r"^(\S+)\.$").unwrap();
    for (i, word) in words.iter().enumerate() {
        let mut processed_word = word.to_string();
        if let Some(caps) = re_period.captures(word) {
            let pre = &caps[1];
            if i == words.len() - 1 {
                // Last word: split period
                processed_word = format!("{} .", pre);
            } else if (pre.contains('.') && pre.chars().any(|c| c.is_alphabetic()))
                || (nonbreaking_prefix.get(pre) == Some(&1))
                || (i < words.len() - 1
                    && words[i + 1]
                        .chars()
                        .next()
                        .map_or(false, |c| c.is_lowercase()))
            {
                // Keep period attached
            } else if nonbreaking_prefix.get(pre) == Some(&2)
                && i < words.len() - 1
                && words[i + 1]
                    .chars()
                    .next()
                    .map_or(false, |c| c.is_ascii_digit())
            {
                // Keep period attached for numbered items
            } else {
                // Split period
                processed_word = format!("{} .", pre);
            }
        }

        tokenized_text.push_str(&processed_word);
        tokenized_text.push(' ');
    }

    tokenized_text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn english_multiline() {
        let result = tokenize(
            "This sentence is really simple, so it should not be hard to detokenize.\nThis one is no more difficult, but, hey, it is on a new line.",
            Language::En,
        );
        assert_eq!(
            result,
            "This sentence is really simple , so it should not be hard to detokenize .\nThis one is no more difficult , but , hey , it is on a new line ."
        );
    }

    #[test]
    fn english_double_quotes() {
        let result = tokenize("This is a somewhat \"less simple\" test.", Language::En);
        assert_eq!(result, "This is a somewhat \" less simple \" test .");
    }
    // TODO expand further with examples from https://github.com/moses-smt/mosesdecoder/blob/master/regression-testing/run-test-detokenizer.perl
}
