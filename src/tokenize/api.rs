use tokenize::util::string_span_tokenize;

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

pub struct StringTokenizerBuilder {
    _string: &'static str,
}

impl StringTokenizerBuilder {
    pub fn new(string: &'static str) -> StringTokenizerBuilder {
        StringTokenizerBuilder {
            _string: string,
        }
    }

    pub fn build(self) -> StringTokenizer {
        StringTokenizer {
            string: self._string,
        }
    }
}

pub struct StringTokenizer {
    pub string: &'static str
}

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
