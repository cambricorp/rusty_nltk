use tokenize::util::string_span_tokenize;

pub trait TokenizerI {

    fn tokenize<'a>(&'a self, s: &'a str) -> Result<Vec<&str>, String>;

    fn span_tokenize(&self, _s: &str) -> Result<Vec<(usize, usize)>, String> {
        return Err(String::from("Not implemented."));
    }

    fn tokenize_sents<'a>(&'a self, strings: &[&'a str]) -> Vec<Vec<&str>> {
        let vec = strings.iter().map(|s| {
            self.tokenize(s).unwrap()
        }).collect();
        vec
    }

    fn span_tokenize_sents(&self, strings: &[&str])  -> Vec<Vec<(usize, usize)>> {
        let result = strings.iter().map(|s| {
            self.span_tokenize(s).unwrap()
        }).collect();
        result
    }
}

pub struct StringTokenizer { pub string: &'static str }

impl TokenizerI for StringTokenizer {

    fn tokenize<'a>(&'a self, s: &'a str) -> Result<Vec<&str>, String> {
        let split_str = s.split(self.string).collect();
        Ok(split_str)
    }

    fn span_tokenize(&self, _s: &str) -> Result<Vec<(usize, usize)>, String> {
        let result = match string_span_tokenize(_s, self.string) {
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
    use super::TokenizerI;
    use super::StringTokenizer;

    #[test]
    fn tokenize_sents_test() {
        let test_strings: Vec<&str> = vec!["hello world", "foo bar"];

        let str_tok = StringTokenizer { string: " " };
        let result: Vec<Vec<&str>> = str_tok.tokenize_sents(&test_strings);

        let expected: Vec<Vec<&str>> = vec![vec!["hello", "world"], vec!["foo", "bar"]];
        assert_eq!(expected, result);
    }

    #[test]
    fn span_tokenize_sents_test() {
        let test_strings: Vec<&str> = vec!["hello world", "foo bar"];
        let str_tok = StringTokenizer { string: " " };
        let result: Vec<Vec<(usize, usize)>> = str_tok.span_tokenize_sents(&test_strings);

        let expected = vec![vec![(0, 5), (6, 11)], vec![(0, 3), (4, 7)]];
        assert_eq!(expected, result);
    }

    #[test]
    fn tokenize_test() {
        let test_string = "hello world";
        let str_tok = StringTokenizer { string: " " };
        let result: Vec<&str> = str_tok.tokenize(test_string).unwrap();

        let expected = vec!["hello", "world"];
        assert_eq!(expected, result);
    }

    #[test]
    fn  span_tokenize_test() {
        let test_string = "hello world";
        let str_tok = StringTokenizer { string: " " };
        let result: Vec<(usize, usize)> = str_tok.span_tokenize(test_string).unwrap();

        let expected = vec![(0, 5), (6, 11)];
        assert_eq!(expected, result);
    }

}
