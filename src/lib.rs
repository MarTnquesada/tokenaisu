use regex::Regex;

#[derive(PartialEq, Debug  )]
pub enum Language {
    En, Es, Fi, Sv, Tdt, Ca
}

pub fn tokenize(text: &str, language: Language) -> String {
    // Remove railing newline character
    let mut tokenized_text: String = String::from(text.trim_end_matches('\n'));
    // Add spaces at the beginning and end of the text 
    tokenized_text = format!(" {tokenized_text} ");
    // Replace all sequences of whitespaces with a single whitespace
    tokenized_text = tokenized_text.split_whitespace().collect::<Vec<&str>>().join(" ");
    // Remove ASCII characters 0-31 (always works because the first 128 ASCII chars match the first 128 unicode chars)
    tokenized_text = tokenized_text
        .chars()
        .filter(|&ch| ch as u8 > 31)
        .collect();
    // Capture protected patterns and replace them with unique substitution strings
    // TODO 
    // Replace all sequences of whitespaces with a single whitespace (again)
    tokenized_text = tokenized_text.split_ascii_whitespace().collect::<Vec<&str>>().join(" ");
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
    }
    else if language == Language::Tdt {
        // # In Tetun, the apostrophe can be used inside words as an apostrophe-like character:
        let re_general = Regex::new(r"([^\p{L}\p{N}\s\.\'\`\,\-])").unwrap();
        tokenized_text = re_general.replace_all(&tokenized_text, " $1 ").to_string();
        // If an apostrophe is not immediately followed by lower-case characters, separate it out anyway
        let re_apostrophe = Regex::new(r"(')(?=$|[^\p{Ll}])").unwrap();
        tokenized_text = re_apostrophe.replace_all(&tokenized_text, " $1 ").to_string();
    }
    else if language == Language::Ca {
        // In Catalan, the middle dot can be used inside words:
        let re_general = Regex::new(r"([^\p{L}\p{N}\s\.\u{00B7}'\`\,\-])").unwrap();
        tokenized_text = re_general.replace_all(&tokenized_text, " $1 ").to_string();
        // If a middot is not immediately followed by lower-case characters, separate it out anywa
        let re_middot = Regex::new(r"(\u{00B7})(?=$|[^\p{Ll}])").unwrap();
        tokenized_text = re_middot.replace_all(&tokenized_text, " $1 ").to_string();
    }
    else {
        let re_general = Regex::new(r"([^\p{L}\p{N}\s\.\'\`\,\-])").unwrap();
        tokenized_text = re_general.replace_all(&tokenized_text, " $1 ").to_string();
    }
    // Optional aggressive hyphen splitting
    // TODO
    // Multi-dot tagging
    let re_new_multi_dot = Regex::new(r"\.([\.]+)").unwrap();
    tokenized_text = re_new_multi_dot.replace_all(&tokenized_text, " DOTMULTI$1").to_string();
    let re_dotmulti_left = Regex::new(r"DOTMULTI\.").unwrap();
    let re_dotmulti_plus_nondot: std::ops::Range<Result<Regex, regex::Error>> = Regex::new(r"DOTMULTI\.([^\.])").unwrap();
    let re_dotmulti_expand = Regex::new(r"DOTMULTI\.").unwrap();
    while re_dotmulti_left.is_match(&tokenized_text) {
        // Replace DOTMULTI. followed by non-dot with DOTDOTMULTI plus that character
        tokenized_text = re_dotmulti_plus_nondot.replace_all(&tokenized_text, "DOTDOTMULTI $1").to_string();
        // Replace any remaining DOTMULTI. with DOTDOTMULTI
        tokenized_text = re_dotmulti_expand.replace_all(&tokenized_text, "DOTDOTMULTI").to_string();
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
            Language::En);
        assert_eq!(result, "This sentence is really simple , so it should not be hard to detokenize .\nThis one is no more difficult , but , hey , it is on a new line .");
    }

    #[test]
    fn english_double_quotes() {
        let result = tokenize("This is a somewhat \"less simple\" test.", Language::En);
        assert_eq!(result, "This is a somewhat \" less simple \" test .");
    }
    // TODO expand further with examples from https://github.com/moses-smt/mosesdecoder/blob/master/regression-testing/run-test-detokenizer.perl and 
}