extern crate regex;
use regex::Regex;

pub fn string_span_tokenize(s: &str, sep: &str) -> Result<Vec<(usize, usize)>, String> {
    if sep.is_empty() {
        return Err(String::from("Error! Separator has a length of 0!"));
    }

    let mut left = 0;
    let spans = s.split(sep).map(|piece| {
        let right = left + piece.len();
        let span = (left, right);
        left = right + sep.len();
        span
    }).collect();
    Ok(spans)
}

pub fn regexp_span_tokenize(s: &str, regexp: &regex::Regex) -> Vec<(usize, usize)> {
    let mut left = 0;

    let mut spans: Vec<_> = regexp.find_iter(s).map(|(right, next)| {
        let span = (left, right);
        left = next;
        span
    }).collect();

    spans.push((left, s.len()));

    spans
}

pub fn spans_to_relative(spans: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut prev = 0;

    spans.iter().map(|&(left, right)| {
        let span = (left - prev, right - left);
        prev = right;
        span
    }).collect()
}

#[cfg(test)]
mod test_util {
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

        let expected = vec![(0, 5), (6, 11)];
        assert_eq!(expected, result);
    }

    #[test]
    fn spans_to_relative_test() {
        let test_span = vec![(0, 5), (6, 11)];
        let result = spans_to_relative(&test_span);

        let expected = vec![(0, 5), (1, 5)];
        assert_eq!(expected, result);
    }
}
