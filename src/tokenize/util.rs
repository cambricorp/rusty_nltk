extern crate regex;
use regex::Regex;

pub fn string_span_tokenize(s: &str, sep: &str) -> Result<Vec<(usize, usize)>, String> {
    if sep.len() == 0 {
        Err(String::from("Error! Separator has a length of 0!"))
    } else {
        // TODO: we'll likely want to do some error checking
        // to ensure s.len() and str.len() don't exceed usize::MAX
        let strlen: usize = s.len();
        let seplen: usize = sep.len() as usize;
        let mut result: Vec<(usize, usize)> = Vec::new();
        let mut left: usize = 0;
        let mut r_idx: usize;
        loop {
            let right = s.find(sep);
            match right {
                Some(right_idx) => {
                    if right_idx != 0 {
                        result.push( (left, right_idx as usize) );
                    }
                    r_idx = right_idx as usize;
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

pub fn regexp_span_tokenize(s: &str, regexp: &regex::Regex) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    for pos in regexp.find_iter(s) {
        result.push(pos);
    }
    return result;
}

pub fn spans_to_relative(spans: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut prev = 0;
    let mut result: Vec<(usize, usize)> = Vec::new();
    for tuple in spans.iter() {
        let (left, right) = tuple.to_owned();
        result.push((left - prev, right - left));
        prev = right;
    }
    result
}
