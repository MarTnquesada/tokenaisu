# tokenaisu

A bundle of different tokenizer implementations written in Rust.

## Moses-like tokenizer

This tokenizer follows the original https://github.com/moses-smt/mosesdecoder/blob/master/scripts/tokenizer/tokenizer.perl fairly closely, but there are a few differences to consider:

- The number of language codes accepted is limited to only those explicitely supported (see the list in with `some command` or within `tokenaisu::Language`). That is, those for which there are specific branching paths in the tokenizer or that have specific non-breaking prefixes or protected patterns (just like in the original Moses). However, this implementation does not allow to select language as "undefined". If you want to tokenize for a language that is not listed as supported, choose instead a closely related language. Or even better, add support for it :smiley:.
- Parallelization is currently limited to one line per thread, and the number of threads matches the number of availables cores as per [Rayon](https://docs.rs/rayon/latest/rayon/)'s default behaviour.

#### Usage

Tokenizing the English text file `untokenized_text.txt` contained in the Tokenaisu repository would be done as follows:

```
tokenaisu --language en --input-file-path untokenized_text.txt --output-file-path my_tokenized_test.txt
```

## TBD
