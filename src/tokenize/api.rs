use tokenize::util::string_span_tokenize;

// TODO: determine if we should implement an iterator
/*struct Tokens {
    inner_vec: Vec<usize, usize>,
    curr_idx: usize,
}

impl Tokens {
    fn new(inner_vec: Vec<usize, usize>) -> Tokens {
        Tokens { inner_vec: inner_vec, curr_idx: 0 }
    }
}

impl Iterator for Tokens {
    type Item = Vec<usize, usize>;

    fn next(&mut self) -> Option<Vec<usize, usize>> {
        if curr_idx >= inner_vec.len() {
            None
        } else {
            let result = Some(self.inner_vec[self.curr_idx]);
            self.curr_idx += 1;
            result
        }
    }
}*/

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

    // TODO: Unsure if we can use an iterator the way Python does...
    // so for now, just return a Vec
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
