extern crate regex;

use regex::Regex;
use std::cmp;

use tokenize::api::TokenizerI;

pub struct SExprTokenizer {
    _strict: bool,
    _open_paren: &'static str,
    _close_paren: &'static str,
    _paren_regexp: regex::Regex
}

impl TokenizerI for SExprTokenizer {

    fn tokenize<'a>(&'a self, _s: &'a str) -> Result<Vec<&str>, String> {
        let mut result: Vec<&str> = Vec::new();
        let mut pos: usize = 0;
        let mut depth: usize = 0;

        for cap in self._paren_regexp.captures_iter(_s) {
            let paren = cap.at(0).unwrap();

            let (start, end) = cap.pos(0).unwrap();

            if depth == 0 {
                for token in _s[pos..start].split_whitespace() {
                    result.push(token);
                }
                pos = start;
            }
            if paren == self._open_paren {
                depth = depth + 1;
            }
            if paren == self._close_paren {
                if self._strict && depth == 0 {
                    return Err(format!("Unmatched open paren at {}", pos));
                }
                depth = cmp::max(0, depth-1);
                if depth == 0 {
                    result.push(&_s[pos..end]);
                    pos = end;
                }
            }
        }
        return Ok(result);
    }
}

#[cfg(test)]
mod test_sexpr {
    use regex::Regex;

    use tokenize::api::TokenizerI;
    use super::SExprTokenizer;

    #[test]
    fn strict_parens_test() {
        let _strict =  true;
        let _open_paren = "(";
        let _close_paren = ")";
        let _paren_regexp = Regex::new(
            &format!("\\{}|\\{}", _open_paren, _close_paren)
        ).unwrap();

        let tokenizer = SExprTokenizer{_strict: _strict, _open_paren: _open_paren,
            _close_paren: _close_paren, _paren_regexp: _paren_regexp};

        let text = "(a b (c d)) e f (g)";
        let expected = vec!["(a b (c d))", "e", "f", "(g)"];
        let result = tokenizer.tokenize(text).unwrap();
        assert_eq!(expected, result);
    }
}
