/*struct Tokens {
    inner_vec: Vec<i32, i32>,
    curr_idx: i32,
}

impl Tokens {
    fn new(inner_vec: Vec<i32, i32>) -> Tokens {
        Tokens { inner_vec: inner_vec, curr_idx: 0 }
    }
}

impl Iterator for Tokens {
    type Item = Vec<i32, i32>;

    fn next(&mut self) -> Option<Vec<i32, i32>> {
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

    fn span_tokenize(&self, s: &str) -> Vec<(i32, i32)>;

    fn tokenize_sents<'a>(&'a self, strings: Vec<&'a str>) -> Vec<Vec<&str>> {
        let mut vec = Vec::new();
        for s in strings.iter() {
            vec.push(self.tokenize(s));
        }
        vec
    }

    // TODO: Unsure if we can use an iterator the way Python does...
    // so for now, just return a Vec
    fn span_tokenize_sents(&self, strings: Vec<&str>)  -> Vec<Vec<(i32, i32)>> {
        let mut result: Vec<Vec<(i32, i32)>> = Vec::new();
        for s in strings.iter() {
            let span: Vec<(i32, i32)> = self.span_tokenize(s);
            result.push(span);
        }
        result
    }
}

pub struct StringTokenizer { _string: &'static str }

impl TokenizerI for StringTokenizer {

    fn tokenize<'a>(&'a self, s: &'a str) -> Vec<&str> {
        let split_str: Vec<&str> = s.split(self._string).collect();
        return split_str;
    }

    fn span_tokenize(&self, s: &str) -> Vec<(i32, i32)> {
        let mut result: Vec<(i32, i32)> = Vec::new();
        match string_span_tokenize(s, self._string) {
            Ok(spans) => { 
                for span in spans.iter() {
                    result.push(span.to_owned());
                }
            },
            Err(err) => { print!("{:?}", err) }
        }
        result
    }
}

fn string_span_tokenize(s: &str, sep: &str) -> Result<Vec<(i32, i32)>, String> {
    if sep.len() == 0 {
        Err(String::from("Error! Separator has a length of 0!"))
    } else {
        // TODO: we'll likely want to do some error checking
        // to ensure s.len() and str.len() don't exceed i32::MAX
        let strlen: i32 = s.len() as i32;
        let seplen: i32 = sep.len() as i32;
        let mut result: Vec<(i32, i32)> = Vec::new();
        let mut left: i32 = 0;
        let mut r_idx: i32;
        loop {
            let right = s.find(sep);
            match right {
                Some(right_idx) => {
                    if right_idx != 0 {
                        result.push( (left, right_idx as i32) );
                    }
                    r_idx = right_idx as i32;
                },
                None => {
                    if left != strlen {
                        result.push( (left, strlen) );
                    }
                    break;
                }
            }
            left = r_idx + seplen;
        }
        Ok(result)
    }
}
