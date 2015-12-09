extern crate regex;

use regex::Regex;
use tokenize::api::Tokenizer;

pub struct SExpressionTokenizerBuilder {
    _strict: bool,
    _open_paren: &'static str,
    _close_paren: &'static str,
}

impl SExpressionTokenizerBuilder {
    pub fn new(strict: bool) -> SExpressionTokenizerBuilder {
        SExpressionTokenizerBuilder {
            _strict: strict,
            _open_paren: "(",
            _close_paren: ")",
        }
    }

    pub fn open_close(self, open: &'static str, close: &'static str) -> SExpressionTokenizerBuilder {
        SExpressionTokenizerBuilder {
            _open_paren: open,
            _close_paren: close,
            ..self
        }
    }

    pub fn build(self) -> SExpressionTokenizer {
        let paren_regexp = Regex::new(
            &format!("\\{}|\\{}", self._open_paren, self._close_paren)
        ).unwrap();

        SExpressionTokenizer {
            strict: self._strict,
            open_paren: self._open_paren,
            close_paren: self._close_paren,
            paren_regexp: paren_regexp,
        }
    }
}

pub struct SExpressionTokenizer {
    strict: bool,
    open_paren: &'static str,
    close_paren: &'static str,
    paren_regexp: regex::Regex
}

impl Tokenizer for SExpressionTokenizer {
    fn tokenize<'a>(&self, s: &'a str) -> Result<Vec<&'a str>, String> {
        let mut result = Vec::new();
        let mut pos = 0;
        let mut depth = 0;

        for cap in self.paren_regexp.captures_iter(s) {
            let paren = cap.at(0).unwrap();

            let (start, end) = cap.pos(0).unwrap();

            if depth == 0 {
                let tokens =
                    s[pos..start]
                    .split(char::is_whitespace)
                    .filter(|s| !s.is_empty());

                result.extend(tokens);

                pos = start;
            }

            if paren == self.open_paren {
                depth += 1;
            } else if paren == self.close_paren {
                if self.strict && depth == 0 {
                    return Err(format!("Unmatched open token at {}", pos));
                }
                depth -= 1;
                if depth == 0 {
                    result.push(&s[pos..end]);
                    pos = end;
                }
            }
        }
        Ok(result)
    }
}

#[cfg(test)]
mod test_sexpr {
    use tokenize::api::Tokenizer;
    use super::SExpressionTokenizerBuilder;

    #[test]
    fn passing_strict_parens_test() {
        let strict = true;
        let tokenizer = SExpressionTokenizerBuilder::new(strict).build();

        let text = "(a b (c d)) e f (g)";
        let expected = vec!["(a b (c d))", "e", "f", "(g)"];
        let result = tokenizer.tokenize(text);
        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn passing_strict_braces_test() {
        let strict = true;
        let tokenizer = SExpressionTokenizerBuilder::new(strict).open_close("{", "}").build();

        let text = "{a b {c d}} e f {g}";
        let expected = vec!["{a b {c d}}", "e", "f", "{g}"];
        let result = tokenizer.tokenize(text);
        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn failing_strict_braces_test() {
        let strict = true;
        let tokenizer = SExpressionTokenizerBuilder::new(strict).open_close("{", "}").build();

        let text = "{a b {c d}} e f {g} }";
        let result = tokenizer.tokenize(text);
        assert_eq!(Err("Unmatched open token at 20".to_string()), result)
    }
}
