use tokenize::util::string_span_tokenize;

pub trait TokenizerI {

    fn tokenize<'a>(&'a self, s: &'a str) -> Vec<&str>;

    fn span_tokenize(&self, s: &str) -> Vec<(usize, usize)>;

    fn tokenize_sents<'a>(&'a self, strings: Vec<&'a str>) -> Vec<Vec<&str>> {
        let mut vec = Vec::new();
        for s in strings.iter() {
            vec.push(self.tokenize(s));
        }
        return vec;
    }

    fn span_tokenize_sents(&self, strings: Vec<&str>)  -> Vec<Vec<(usize, usize)>> {
        let mut result: Vec<Vec<(usize, usize)>> = Vec::new();
        for s in strings.iter() {
            let span: Vec<(usize, usize)> = self.span_tokenize(s);
            result.push(span);
        }
        return result;
    }
}

pub struct StringTokenizer { _string: &'static str }

impl TokenizerI for StringTokenizer {

    fn tokenize<'a>(&'a self, s: &'a str) -> Vec<&str> {
        let split_str: Vec<&str> = s.split(self._string).collect();
        return split_str;
    }

    fn span_tokenize(&self, s: &str) -> Vec<(usize, usize)> {
        let mut result: Vec<(usize, usize)> = Vec::new();
        match string_span_tokenize(s, self._string) {
            Ok(spans) => {
                for span in spans.iter() {
                    result.push(span.to_owned());
                }
            },
            Err(err) => { print!("{:?}", err) }
        }
        return result;
    }
}


#[cfg(test)]
mod test_api {
    use super::TokenizerI;
    use super::StringTokenizer;

    #[test]
    fn tokenize_sents_test() {
        let test_strings: Vec<&str> = vec!["hello world", "foo bar"];

        let str_tok = StringTokenizer { _string: " " };
        let result: Vec<Vec<&str>> = str_tok.tokenize_sents(test_strings);

        let expected: Vec<Vec<&str>> = vec![vec!["hello", "world"], vec!["foo", "bar"]];
        assert_eq!(expected, result);
    }
/*
    #[test]
    fn span_tokenize_sents_test() {
        let test_strings: Vec<&str> = vec!["hello world", "foo bar"];
        let str_tok = StringTokenizer { _string: " " };
        let result: Vec<Vec<(usize, usize)>> = str_tok.span_tokenize_sents(test_strings);

        let expected = vec![vec![(0, 5), (6, 11)], vec![(0, 3), (4, 7)]];
        assert_eq!(expected, result);
    }
*/
    #[test]
    fn tokenize_test() {
        let test_string = "hello world";
        let str_tok = StringTokenizer { _string: " " };
        let result: Vec<&str> = str_tok.tokenize(test_string);

        let expected = vec!["hello", "world"];
        assert_eq!(expected, result);
    }
/*
    #[test]
    fn  span_tokenize_test() {
        let test_string = "hello world";
        let str_tok = StringTokenizer { _string: " " };
        let result: Vec<(usize, usize)> = str_tok.span_tokenize(test_string);

        let expected = vec![(0, 5), (6, 11)];
        assert_eq!(expected, result);
    }
*/
}
