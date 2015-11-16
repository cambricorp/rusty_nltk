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

    fn span_tokenize_sents(&self, strings: Vec<&str>)  -> Vec<Vec<(i32, i32)>> {
        let mut result: Vec<Vec<(i32, i32)>> = Vec::new();
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
