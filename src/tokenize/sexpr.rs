extern crate regex;

use regex::Regex;

use tokenize::api::TokenizerI;

pub struct SExprTokenizer {
    _strict: bool,
    _open_paren: char,
    _close_paren: char,
    _paren_regexp: &'static regex::Regex
}

impl TokenizerI for SExprTokenizer {

    fn tokenize<'a>(&'a self, _s: &'a str) -> Vec<&str> {
        let result: Vec<&str> = Vec::new();
        return result;
        // TODO: Return a vector of s-expressions
    }
}
