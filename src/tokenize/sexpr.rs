extern crate regex;

use regex::Regex;
use std::cmp;

use tokenize::api::TokenizerI;

pub struct SExprTokenizer {
    strict: bool,
    open_paren: &'static str,
    close_paren: &'static str,
    paren_regexp: regex::Regex
}

impl TokenizerI for SExprTokenizer {

    fn tokenize<'a>(&'a self, _s: &'a str) -> Result<Vec<&str>, String> {
        let mut result: Vec<&str> = Vec::new();
        let mut pos: usize = 0;
        let mut depth: usize = 0;

        for cap in self.paren_regexp.captures_iter(_s) {
            let paren = cap.at(0).unwrap();

            let (start, end) = cap.pos(0).unwrap();

            if depth == 0 {
                for token in _s[pos..start].split(|c: char| c.is_whitespace())
                                           .filter(|s| !s.is_empty()) {
                        result.push(token);
                }
                pos = start;
            }
            if paren == self.open_paren {
                depth = depth + 1;
            }
            if paren == self.close_paren {
                if self.strict && depth == 0 {
                    return Err(format!("Unmatched open token at {}", pos));
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
    fn passing_strict_parens_test() {
        let strict =  true;
        let open_paren = "(";
        let close_paren = ")";
        let paren_regexp = Regex::new(
            &format!("\\{}|\\{}", open_paren, close_paren)
        ).unwrap();

        let tokenizer = SExprTokenizer {
            strict: strict,
            open_paren: open_paren,
            close_paren: close_paren,
            paren_regexp: paren_regexp
        };

        let text = "(a b (c d)) e f (g)";
        let expected = vec!["(a b (c d))", "e", "f", "(g)"];
        let result = tokenizer.tokenize(text).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn passing_strict_braces_test() {
        let strict =  true;
        let open_paren = "{";
        let close_paren = "}";
        let paren_regexp = Regex::new(
            &format!("\\{}|\\{}", open_paren, close_paren)
        ).unwrap();

        let tokenizer = SExprTokenizer {
            strict: strict,
            open_paren: open_paren,
            close_paren: close_paren,
            paren_regexp: paren_regexp
        };

        let text = "{a b {c d}} e f {g}";
        let expected = vec!["{a b {c d}}", "e", "f", "{g}"];
        let result = tokenizer.tokenize(text).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    #[should_panic(expected = "Unmatched open token at 20")]
    fn failing_strict_braces_test() {
        let strict =  true;
        let open_paren = "{";
        let close_paren = "}";
        let paren_regexp = Regex::new(
            &format!("\\{}|\\{}", open_paren, close_paren)
        ).unwrap();

        let tokenizer = SExprTokenizer {
            strict: strict,
            open_paren: open_paren,
            close_paren: close_paren,
            paren_regexp: paren_regexp
        };

        let text = "{a b {c d}} e f {g} }";
        let _result = tokenizer.tokenize(text).unwrap();
        // Tests expectedly fails - no need to assert.
    }
}
