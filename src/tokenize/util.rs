extern crate regex;
use regex::Regex;

pub fn string_span_tokenize(s: &str, sep: &str) -> Result<Vec<(usize, usize)>, String> {
    if sep.len() == 0 {
        Err(String::from("Error! Separator has a length of 0!"))
    } else {
        // TODO: we'll likely want to do some error checking
        // to ensure s.len() and str.len() don't exceed usize::MAX
        let strlen = s.len();
        let seplen = sep.len();
        let mut result: Vec<(usize, usize)> = Vec::new();
        let mut left = 0;
        let mut r_idx;
        loop {
            let right = s[left..].find(sep); // TODO: Will this work on unicode?
            match right {
                Some(right_idx) => {
                    if right_idx != 0 {
                        result.push((left, right_idx));
                    }
                    r_idx = right_idx;
                },
                None => {
                    if left != strlen {
                        result.push((left, strlen));
                    }
                    break;
                }
            }
            left = r_idx + seplen;
        }
        return Ok(result);
    }
}

pub fn regexp_span_tokenize(s: &str, regexp: &regex::Regex) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    let strlen = s.len();
    let mut left = 0;

    for (right, next) in regexp.find_iter(s) {
        if right != 0 {
            result.push((left, right));
        }
        left = next
    }
    result.push((left, strlen));

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
    return result;
}

#[cfg(test)]
mod util_tests {
    use regex::Regex;

    use super::string_span_tokenize;
    use super::regexp_span_tokenize;
    use super::spans_to_relative;

    #[test]
    fn string_span_tokenize_test() {
        let test_string = "hello world";
        let separator = " ";
        let result: Vec<(usize, usize)> = string_span_tokenize(test_string, separator).unwrap();

        let expected = vec![(0, 5), (6, 11)];
        assert_eq!(expected, result);
    }

    #[test]
    fn regexp_span_tokenize_test() {
        let test_string = "hello world";
        let separator = Regex::new(r"\s").unwrap();
        let result = regexp_span_tokenize(test_string, &separator);

        assert_eq!(vec![(0, 5), (6, 11)], result);
    }

    #[test]
    fn spans_to_relative_test() {
        let test_span = vec![(0, 5), (6, 11)];
        let result = spans_to_relative(test_span);

        assert_eq!(vec![(0, 5), (1, 5)], result);
    }
}
