use tokenize::util::string_span_tokenize;

/// `Tokenizer` is a trait that performs basic tokenizing operations
/// on text such as tokenizing and span tokenizing.
pub trait Tokenizer {
    fn tokenize<'a>(&self, s: &'a str) -> Result<Vec<&'a str>, String>;

    fn span_tokenize(&self, _s: &str) -> Result<Vec<(usize, usize)>, String> {
        return Err(String::from("Not implemented."));
    }

    fn tokenize_sents<'a>(&self, strings: &[&'a str]) -> Vec<Vec<&'a str>> {
        strings.iter().map(|s| {
            self.tokenize(s).unwrap()
        }).collect()
    }

    fn span_tokenize_sents(&self, strings: &[&str])  -> Vec<Vec<(usize, usize)>> {
        strings.iter().map(|s| {
            self.span_tokenize(s).unwrap()
        }).collect()
    }
}

/// `StringTokenizerBuilder` builds string tokenizers that will
/// tokenize on each instance of `_string`.
pub struct StringTokenizerBuilder {
    _string: &'static str,
}

impl StringTokenizerBuilder {
    /// Constructs a new `StringTokenizerBuilder` which tokenizes on `string`.
    ///
    /// # Examples
    ///
    /// To create a `StringTokenizerBuilder` that builds
    /// a space tokenizer:
    ///
    /// ```
    /// extern crate rusty_nltk;
    /// use rusty_nltk::tokenize::api::StringTokenizerBuilder;
    ///
    /// fn main() {
    ///   let space_tokenizer = StringTokenizerBuilder::new(" ");
    /// }
    /// ```
    ///
    /// To create a `StringTokenizerBuilder` that builds
    /// a newline tokenizer:
    ///
    /// ```
    /// extern crate rusty_nltk;
    /// use rusty_nltk::tokenize::api::StringTokenizerBuilder;
    ///
    /// fn main() {
    ///   let newline_tokenizer = StringTokenizerBuilder::new("\n");
    /// }
    /// ```
    pub fn new(string: &'static str) -> StringTokenizerBuilder {
        StringTokenizerBuilder {
            _string: string,
        }
    }

    /// Builds an `StringTokenizer`
    ///
    /// # Examples
    ///
    /// To build a `StringTokenizer` which tokenizes on spaces:
    ///
    /// ```
    /// extern crate rusty_nltk;
    /// use rusty_nltk::tokenize::api::{Tokenizer, StringTokenizer, StringTokenizerBuilder};
    ///
    /// fn main() {
    ///   let space_tokenizer = StringTokenizerBuilder::new(" ").build();
    /// }
    /// ```
    ///
    /// To build a `StringTokenizer` which tokenizes on newlines:
    ///
    /// ```
    /// extern crate rusty_nltk;
    /// use rusty_nltk::tokenize::api::{Tokenizer, StringTokenizer, StringTokenizerBuilder};
    ///
    /// fn main() {
    ///   let newline_tokenizer = StringTokenizerBuilder::new("\n").build();
    /// }
    /// ```
    pub fn build(self) -> StringTokenizer {
        StringTokenizer {
            string: self._string,
        }
    }
}

/// `StringTokenizer` tokenizes text based on parameters determined by
/// its builder.
pub struct StringTokenizer {
    pub string: &'static str
}

/// Tokenize `s` based on parameters set by `StringTokenizerBuilder`
///
/// # Examples
///
/// To tokenize `"Hello, World!"` on spaces:
///
/// ```
/// extern crate rusty_nltk;
/// use rusty_nltk::tokenize::api::{Tokenizer, StringTokenizer, StringTokenizerBuilder};
///
/// fn main() {
///   let space_tokenizer = StringTokenizerBuilder::new(" ").build();
///   let result = space_tokenizer.tokenize("Hello, World!");
/// }
/// ```
///
/// To tokenize `"Hello,\nWorld!"` on spaces:
///
/// ```
/// extern crate rusty_nltk;
/// use rusty_nltk::tokenize::api::{Tokenizer, StringTokenizer, StringTokenizerBuilder};
///
/// fn main() {
///   let space_tokenizer = StringTokenizerBuilder::new("\n").build();
///   let result = space_tokenizer.tokenize("Hello,\nWorld!");
/// }
/// ```
impl Tokenizer for StringTokenizer {
    fn tokenize<'a>(&self, s: &'a str) -> Result<Vec<&'a str>, String> {
        let split_str = s.split(self.string).collect();
        Ok(split_str)
    }

    fn span_tokenize(&self, s: &str) -> Result<Vec<(usize, usize)>, String> {
        let result = match string_span_tokenize(s, self.string) {
            Ok(spans) => {
                spans.iter().map(|span| {
                    span.to_owned()
                }).collect()
            },
            Err(err) => {
                return Err(String::from(format!("Error: {}", err)));
            }
        };
        Ok(result)
    }
}

#[cfg(test)]
mod test_api {
    use super::{Tokenizer, StringTokenizerBuilder};

    #[test]
    fn tokenize_sents_test() {
        let tokenizer = StringTokenizerBuilder::new(" ").build();
        let test_strings = vec!["hello world", "foo bar"];
        let result = tokenizer.tokenize_sents(&test_strings);
        let expected = vec![vec!["hello", "world"], vec!["foo", "bar"]];
        assert_eq!(expected, result);
    }

    #[test]
    fn span_tokenize_sents_test() {
        let tokenizer = StringTokenizerBuilder::new(" ").build();
        let test_strings = vec!["hello world", "foo bar"];
        let result = tokenizer.span_tokenize_sents(&test_strings);
        let expected = vec![vec![(0, 5), (6, 11)], vec![(0, 3), (4, 7)]];
        assert_eq!(expected, result);
    }

    #[test]
    fn tokenize_test() {
        let tokenizer = StringTokenizerBuilder::new(" ").build();
        let test_string = "hello world";
        let result = tokenizer.tokenize(test_string);
        let expected = vec!["hello", "world"];
        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn  span_tokenize_test() {
        let tokenizer = StringTokenizerBuilder::new(" ").build();
        let test_string = "hello world";
        let result = tokenizer.span_tokenize(test_string);
        let expected = vec![(0, 5), (6, 11)];
        assert_eq!(Ok(expected), result);
    }
}
