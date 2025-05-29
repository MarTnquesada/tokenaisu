# tokenaisu
A bundle of different tokenizer implementations (namely Moses) in Rust.

## Moses tokenizer
The Moses-like tokenizer follows the original https://github.com/moses-smt/mosesdecoder/blob/master/scripts/tokenizer/tokenizer.perl fairly closely, but there are a few differences to consider:
- Wherever possible, steps that were only applicable to ASCII text have been expanded to work over the same UTF-8 domain. I.e., prefering `trim()` over `trim_ascii()`, therefore trimming all unicode whitespace-like characters rather than only ASCII ones.
- ...
 
### Usage