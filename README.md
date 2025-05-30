# tokenaisu

A bundle of different tokenizer implementations (namely Moses) written in Rust.

## Moses tokenizer

The Moses-like tokenizer follows the original https://github.com/moses-smt/mosesdecoder/blob/master/scripts/tokenizer/tokenizer.perl fairly closely, but there are a few differences to consider:

- The number of language codes accepted is limited to only those explicitely supported (see the list in with `some command` or within `tokenaisu::Language`). That is, those for which there are specific branching paths in the tokenizer or that have specific non-breaking prefixes or protected patterns. If you want to tokenize for a language that is not listed as supported, choose instead a closely related language. Or even better, add support for it :smiley:.
- Wherever possible, steps that were only applicable to ASCII text have been expanded to work over the same UTF-8 domain. I.e., prefering `trim()` over `trim_ascii()`, therefore trimming all unicode whitespace-like characters rather than only ASCII ones.
- ...

### Usage
