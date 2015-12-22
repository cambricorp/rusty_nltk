extern crate regex;

use regex::Regex;
use tokenize::api::Tokenizer;

/// `SExpressionTokenizerBuilder` builds s-expression tokenizers with varying strictness.
/// Unless grouping symbols are changed, they will be defaulted to `(` and `)`.
pub struct SExpressionTokenizerBuilder {
    /// _strict determines if non-matching parenthesis are allowed.
    _strict: bool,
    /// _open_paren is used as the opening grouping symbol to be matched.
    _open_paren: char,
    /// _close_paren is used as the closing grouping symbol to be matched.
    _close_paren: char,
}

impl SExpressionTokenizerBuilder {
    /// Constructs a new `SExpressionTokenizerBuilder` with strictness set to `strict`.
    ///
    /// # Examples
    ///
    /// To create a `SExpressionTokenizerBuilder` that builds a strict tokenizer:
    ///
    /// ```
    /// extern crate rusty_nltk;
    /// use rusty_nltk::tokenize::sexpr::SExpressionTokenizerBuilder;
    ///
    /// fn main() {
    ///   let sexpr_tokenizer_builder = SExpressionTokenizerBuilder::new(true);
    /// }
    /// ```
    ///
    /// To create an `SExpressionTokenizerBuilder` that builds a tokenizer that
    /// is not strict:
    ///
    /// ```
    /// extern crate rusty_nltk;
    /// use rusty_nltk::tokenize::sexpr::SExpressionTokenizerBuilder;
    ///
    /// fn main() {
    ///   let sexpr_tokenizer_builder = SExpressionTokenizerBuilder::new(false);
    /// }
    /// ```
    pub fn new(strict: bool) -> SExpressionTokenizerBuilder {
        SExpressionTokenizerBuilder {
            _strict: strict,
            _open_paren: '(',
            _close_paren: ')',
        }
    }

    /// Changes the opening and closing grouping symbols to `open` and `close`, respectively.
    ///
    /// # Examples
    ///
    /// To change the grouping symbols to `{` and `}` for a strict
    /// `SExpressionTokenizerBuilder`:
    ///
    /// ```
    /// extern crate rusty_nltk;
    /// use rusty_nltk::tokenize::sexpr::SExpressionTokenizerBuilder;
    /// fn main() {
    ///   let sexpr_tokenizer_builder = SExpressionTokenizerBuilder::new(true).open_close('{', '}');
    /// }
    /// ```
    ///
    /// To change the grouping symbols to `[` and `]` for an
    /// `SExpressionTokenizerBuilder` that is not strict:
    ///
    /// ```
    /// extern crate rusty_nltk;
    /// use rusty_nltk::tokenize::sexpr::SExpressionTokenizerBuilder;
    /// fn main() {
    ///   let sexpr_tokenizer_builder = SExpressionTokenizerBuilder::new(false).open_close('[', ']');
    /// }
    /// ```
    pub fn open_close(self, open: char, close: char) -> SExpressionTokenizerBuilder {
        SExpressionTokenizerBuilder {
            _open_paren: open,
            _close_paren: close,
            ..self
        }
    }

    /// Builds an `SExpressionTokenizer`
    ///
    /// # Examples
    ///
    /// To build a strict `SExpressionTokenizer` with `{` and `}` grouping symbols:
    ///
    /// ```
    /// extern crate rusty_nltk;
    /// use rusty_nltk::tokenize::sexpr::{SExpressionTokenizer, SExpressionTokenizerBuilder};
    ///
    /// fn main() {
    ///   let sexpr_tokenizer_builder = SExpressionTokenizerBuilder::new(true).open_close('{', '}');
    ///   let tokenizer = sexpr_tokenizer_builder.build();
    /// }
    /// ```
    ///
    /// To build an `SExpressionTokenizer` that is not strict with `[` and `]` grouping symbols:
    ///
    /// ```
    /// extern crate rusty_nltk;
    /// use rusty_nltk::tokenize::sexpr::{SExpressionTokenizer, SExpressionTokenizerBuilder};
    ///
    /// fn main() {
    ///   let sexpr_tokenizer_builder = SExpressionTokenizerBuilder::new(false).open_close('[', ']');
    ///   let tokenizer = sexpr_tokenizer_builder.build();
    /// }
    /// ```
    ///
    /// To build an `SExpressionTokenizer` that is not strict with default `(` and `)` grouping symbols:
    ///
    /// ```
    /// extern crate rusty_nltk;
    /// use rusty_nltk::tokenize::sexpr::{SExpressionTokenizer, SExpressionTokenizerBuilder};
    ///
    /// fn main() {
    ///   let tokenizer = SExpressionTokenizerBuilder::new(false).build();
    /// }
    /// ```
    pub fn build(self) -> SExpressionTokenizer {
        let paren_regexp = Regex::new(
            &format!("\\{}|\\{}", self._open_paren.to_string(), self._close_paren.to_string())
        ).unwrap();

        SExpressionTokenizer {
            strict: self._strict,
            open_paren: self._open_paren,
            close_paren: self._close_paren,
            paren_regexp: paren_regexp,
        }
    }
}

/// `SExpressionTokenizer` tokenizes s-expression based on parameters determined by
/// its builder.
pub struct SExpressionTokenizer {
    /// strict determines if non-matching parenthesis are allowed.
    strict: bool,
    /// open_paren is used as the opening grouping symbol to be matched.
    open_paren: char,
    /// close_paren is used as the closing grouping symbol to be matched.
    close_paren: char,
    /// paren_regex is the regex to find matches of grouping symbols.
    paren_regexp: regex::Regex
}

/// Tokenize `s` based on parameters set by `SExpressionTokenizerBuilder`
///
/// # Examples
///
/// To tokenize `"(a b (c d)) e f (g)"` strictly with default grouping symbols:
///
/// ```
/// extern crate rusty_nltk;
/// use rusty_nltk::tokenize::sexpr::{SExpressionTokenizer, SExpressionTokenizerBuilder};
/// use rusty_nltk::tokenize::api::Tokenizer;
///
/// fn main() {
///   let text = "(a b (c d)) e f (g)";
///   let tokenizer = SExpressionTokenizerBuilder::new(true).build();
///   let result = tokenizer.tokenize(text);
/// }
/// ```
///
/// To tokenize `"(a b (c d)) e f (g)"` not strictly with default grouping symbols:
///
/// ```
/// extern crate rusty_nltk;
/// use rusty_nltk::tokenize::sexpr::{SExpressionTokenizer, SExpressionTokenizerBuilder};
/// use rusty_nltk::tokenize::api::Tokenizer;
///
/// fn main() {
///   let text = "(a b (c d)) e f (g)";
///   let tokenizer = SExpressionTokenizerBuilder::new(false).build();
///   let result = tokenizer.tokenize(text);
/// }
/// ```
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

            if paren == self.open_paren.to_string() {
                depth += 1;
            } else if paren == self.close_paren.to_string() {
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

/// Tokenizes an s-expression with strictness set to `strict` and default `(` and `)`
/// grouping symbols.
///
/// # Examples
///
/// To tokenize `"(a b (c d)) e f (g)"` strictly:
///
/// ```
/// extern crate rusty_nltk;
/// use rusty_nltk::tokenize::sexpr::sexpression_tokenize;
///
/// fn main() {
///   let text = "(a b (c d)) e f (g)";
///   let result = sexpression_tokenize(text, true);
/// }
/// ```
pub fn sexpression_tokenize<'a>(s: &'a str, strict: bool) -> Result<Vec<&'a str>, String> {
    use tokenize::api::Tokenizer;
    use tokenize::sexpr::SExpressionTokenizerBuilder;
    let tokenizer = SExpressionTokenizerBuilder::new(strict).build();
    tokenizer.tokenize(s)
}

#[cfg(test)]
mod test_sexpr {
    use tokenize::api::Tokenizer;
    use super::{SExpressionTokenizerBuilder, sexpression_tokenize};

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
    fn passing_strict_parens_test_fn() {
        let strict = true;
        let text = "(a b (c d)) e f (g)";
        let expected = vec!["(a b (c d))", "e", "f", "(g)"];
        let result = sexpression_tokenize(text, strict);
        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn passing_strict_braces_test() {
        let strict = true;
        let tokenizer = SExpressionTokenizerBuilder::new(strict).open_close('{', '}').build();

        let text = "{a b {c d}} e f {g}";
        let expected = vec!["{a b {c d}}", "e", "f", "{g}"];
        let result = tokenizer.tokenize(text);
        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn failing_strict_braces_test() {
        let strict = true;
        let tokenizer = SExpressionTokenizerBuilder::new(strict).open_close('{', '}').build();

        let text = "{a b {c d}} e f {g} }";
        let result = tokenizer.tokenize(text);
        assert_eq!(Err("Unmatched open token at 20".to_string()), result)
    }
}
