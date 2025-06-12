use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
use strum_macros;

#[derive(PartialEq, Debug, strum_macros::AsRefStr)]
#[strum(serialize_all = "lowercase")]
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

fn load_nonbreaking_prefixes(language: Language) -> HashMap<String, u8> {
    // TODO locate actual file from absolute path
    let mut prefixfile = format!(
        "nonbreaking_prefixes/nonbreaking_prefix.{}",
        language.as_ref()
    );
    print!("{}", language.as_ref());
    if !Path::new(&prefixfile).exists() {
        eprintln!(
            "WARNING: No known abbreviations for language '{}', attempting fall-back to English version...",
            language.as_ref()
        );
        prefixfile = format!("nonbreaking_prefixes/nonbreaking_prefix.en");
        if !Path::new(&prefixfile).exists() {
            // TODO exit
        }
    }
    //let file = File::open(&prefixfile)?;
    //let reader = io::BufReader::new(file);
    let mut nonbreaking_prefixes: HashMap<String, u8> = HashMap::new();
    // for line in reader.lines() {
    //     let line = line?;
    //     let trimmed = line.trim();
    //     if trimmed.is_empty() || trimmed.starts_with('#') {
    //         continue;
    //     }

    //     if let Some((key, _)) = trimmed.split_once(" #NUMERIC_ONLY#") {
    //         nonbreaking_prefixes.insert(key.to_string(), 2);
    //     } else {
    //         nonbreaking_prefixes.insert(trimmed.to_string(), 1);
    //     }
    // }

    let mut nonbreaking_prefixes: HashMap<String, u8> = HashMap::new();
    nonbreaking_prefixes.insert("St".to_string(), 1);
    nonbreaking_prefixes.insert("vs".to_string(), 1);
    nonbreaking_prefixes
}

pub fn moses_tokenize_line(
    text: &str,
    language: Language,
    no_escaping: bool,
    protected_patterns: Vec<&str>,
) -> String {
    let mut tokenized_text = text
        // Remove trailing newline character
        .trim_end_matches('\n')
        // Replace all sequences of whitespaces with a single whitespace while trimming text
        // This is done for any type of Unicode space (incl. tabs), so code executed after this is only concerned with strict ASCII spaces
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");

    // Add spaces at the beginning and end of the text
    tokenized_text.reserve(2);
    tokenized_text.insert(0, ' ');
    tokenized_text.push(' ');

    // Remove ASCII characters 0-31 (works because the first 128 ASCII chars match the first 128 unicode chars)
    tokenized_text = tokenized_text.chars().filter(|&ch| ch as u8 > 31).collect();

    // Capture protected patterns and replace them with unique substitution strings
    let mut found_protected_patterns: HashMap<String, String> = HashMap::new();
    for pattern in protected_patterns {
        let re_pattern = Regex::new(pattern).unwrap();
        tokenized_text = re_pattern
            .replace_all(&text, |caps: &regex::Captures| {
                let substitution = format!("THISISPROTECTED{:03}", found_protected_patterns.len());
                found_protected_patterns.insert(substitution.clone(), caps[0].to_owned());
                print!("{}, {}", &substitution, &caps[0]);
                substitution
            })
            .to_string();
    }
    print!("{}", &tokenized_text);
    // After substituting protected patterns, replace all sequences of whitespaces with a single whitespace and trim the text
    tokenized_text = tokenized_text
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");

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
    let words: Vec<&str> = tokenized_text.split_whitespace().collect();
    let mut word_tokenized_text = String::new();
    let re_period = Regex::new(r"^(\S+)\.$").unwrap();
    // ##  TODO This is an example of how the non-breaking prefix data structure looks, but it needs to be implemented ## /
    let nonbreaking_prefixes = load_nonbreaking_prefixes(language);
    // ##  ## /
    for (i, word) in words.iter().enumerate() {
        let mut processed_word = word.to_string();
        if let Some(caps) = re_period.captures(word) {
            let pre = &caps[1];
            if i == words.len() - 1 {
                // Last word: split period
                processed_word = format!("{} .", pre);
            } else if (pre.contains('.') && pre.chars().any(|c| c.is_alphabetic()))
                || (nonbreaking_prefixes.get(pre) == Some(&1))
                || (i < words.len() - 1
                    && words[i + 1]
                        .chars()
                        .next()
                        .map_or(false, |c| c.is_lowercase()))
            {
                // Keep period attached
            } else if nonbreaking_prefixes.get(pre) == Some(&2)
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
        word_tokenized_text.push_str(&processed_word);
        word_tokenized_text.push(' ');
    }
    tokenized_text = word_tokenized_text.clone();

    // Clean up extraneous spaces
    tokenized_text = tokenized_text
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");

    // .' at end of sentence is missed
    let re_period = Regex::new(r"\.\' ?$").unwrap();
    tokenized_text = re_period.replace(&tokenized_text, ". ' ").to_string();

    // Restore protected patterns
    // TODO could use some optimization
    for (substitution, pattern) in found_protected_patterns {
        tokenized_text = tokenized_text.replace(&substitution, &pattern);
    }

    // Restore multi-dots
    while tokenized_text.contains("DOTDOTMULTI") {
        tokenized_text = tokenized_text.replace("DOTDOTMULTI", "DOTMULTI.");
    }
    tokenized_text = tokenized_text.replace("DOTMULTI", ".");

    // Escape special characters
    if !no_escaping {
        tokenized_text = tokenized_text
            .replace("&", "&amp;") // escape escape
            .replace("|", "&#124;") // factor separator
            .replace("<", "&lt;") // xml
            .replace(">", "&gt;") // xml
            .replace("'", "&apos;") // xml
            .replace("\"", "&quot;") // xml
            .replace("[", "&#91;") // syntax non-terminal
            .replace("]", "&#93;"); // syntax non-terminal
    }

    // Ensure final line break
    if !tokenized_text.ends_with('\n') {
        tokenized_text.push('\n');
    }

    tokenized_text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn english_double_quotes() {
        let result = moses_tokenize_line(
            "This is a somewhat \"less simple\" test.",
            Language::En,
            true,
            vec![],
        );
        assert_eq!(result, "This is a somewhat \" less simple \" test .\n");
    }

    #[test]
    fn french_simple() {
        let result = moses_tokenize_line("Voici une phrase simple.", Language::Fr, true, vec![]);
        assert_eq!(result, "Voici une phrase simple .\n");
    }

    #[test]
    fn french_apostrophe() {
        let result = moses_tokenize_line("Moi, j'ai une apostrophe.", Language::Fr, true, vec![]);
        assert_eq!(result, "Moi , j' ai une apostrophe .\n");
    }

    #[test]
    fn french_apostrophe_penultimate() {
        let result = moses_tokenize_line(
            "de musique rap issus de l'immigration",
            Language::Fr,
            true,
            vec![],
        );
        assert_eq!(result, "de musique rap issus de l' immigration\n");
    }

    #[test]
    fn german_nonascii() {
        let result = moses_tokenize_line(
            "Ich hoffe, daß Sie schöne Ferien hatten.",
            Language::En,
            true,
            vec![],
        );
        assert_eq!(result, "Ich hoffe , daß Sie schöne Ferien hatten .\n");
    }

    // #[test]
    // fn chinese_simple() {
    //     let result = moses_tokenize_line("这是一个简单的的汉语句子。", Language::En, true, vec![]);
    //     assert_eq!(result, "这 是 一个 简单 的的 汉语 句子 。\n");
    // }

    // #[test]
    // fn japanese_simple() {
    //     let result = moses_tokenize_line("どうしょうかな。", Language::En, true, vec![]);
    //     assert_eq!(result, "どう しょ う か な 。\n");
    // }

    #[test]
    fn protected_patterns() {
        // In English, these would normally be contractions that are separated by default
        let text = "Some text containing the protected pattern $'$ and /'/.";

        let result_without_protected = moses_tokenize_line(text, Language::En, true, vec![]);
        assert_eq!(
            result_without_protected,
            "Some text containing the protected pattern $ ' $ and / ' / .\n"
        );

        let result_with_protected =
            moses_tokenize_line(text, Language::En, true, vec![r"([^\p{L}])[']([^\p{L}])"]);
        assert_eq!(
            result_with_protected,
            "Some text containing the protected pattern $'$ and /'/ .\n"
        );
    }

    // TODO expand further with examples from https://github.com/moses-smt/mosesdecoder/blob/master/regression-testing/run-test-detokenizer.perl
    // (but don't use examples with multi-lines since those are intended for end-to-end tests)
}
