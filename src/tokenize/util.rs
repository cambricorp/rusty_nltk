extern crate regex;
use regex::Regex;

/// Return a list of the offsets of the tokens in `s`, as a sequence of `(start, end)`
/// tuples, by splitting the string at each occurrence of `sep`. If `sep` is an empty
/// string, an error will be returned.
///
/// # Examples
///
/// To return a list of spans based on spaces:
///
/// ```
/// extern crate rusty_nltk;
/// use rusty_nltk::tokenize::util::string_span_tokenize;
///
/// fn main() {
///   let s = "Good muffins cost $3.88\nin New York.  Please buy me
///           two of them.\n\nThanks.";
///   let spans = string_span_tokenize(s, " ").unwrap();
/// }
/// ```
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

/// Return a list of the offsets of the tokens in `s`, as a sequence of `(start, end)`
/// tuples, by splitting the string at each successive match of `regex`.
///
/// # Examples
///
/// To return a list of spans based on whitespaces:
///
/// ```
/// extern crate regex;
/// extern crate rusty_nltk;
/// use rusty_nltk::tokenize::util::regexp_span_tokenize;
/// use regex::Regex;
///
/// fn main() {
///   let s = "Good muffins cost $3.88\nin New York.  Please buy me
///           two of them.\n\nThanks.";
///   let regex = Regex::new(r"\s").unwrap();
///   let spans = regexp_span_tokenize(s, &regex).unwrap();
/// }
/// ```
pub fn regexp_span_tokenize(s: &str, regexp: &regex::Regex) -> Result<Vec<(usize, usize)>, String> {
    if regexp.as_str().is_empty() {
        return Err(String::from("Error! Separator has a length of 0!"));
    }

    let mut left = 0;
    let mut spans: Vec<_> = regexp.find_iter(s).map(|(right, next)| {
        let span = (left, right);
        left = next;
        span
    }).collect();
    spans.push((left, s.len()));
    Ok(spans)
}

///  Returns a list of relative spans, given a sequence of spans.
///
/// # Examples
///
/// To return a list of relative spans based on whitespaces:
///
/// ```
/// extern crate regex;
/// extern crate rusty_nltk;
/// use rusty_nltk::tokenize::util::{spans_to_relative, regexp_span_tokenize};
/// use regex::Regex;
///
/// fn main() {
///   let s = "Good muffins cost $3.88\nin New York.  Please buy me
///           two of them.\n\nThanks.";
///   let regex = Regex::new(r"\s").unwrap();
///   let spans = regexp_span_tokenize(s, &regex).unwrap();
///   let rel_spans = spans_to_relative(&spans);
/// }
/// ```
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
    use super::{string_span_tokenize, regexp_span_tokenize, spans_to_relative};

    #[test]
    fn string_span_tokenize_test() {
        let test_string = "hello world";
        let separator = " ";
        let result = string_span_tokenize(test_string, separator);
        let expected = vec![(0, 5), (6, 11)];
        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn regexp_span_tokenize_test() {
        let test_string = "hello world";
        let whitespace = r"\s";
        let separator = Regex::new(whitespace).unwrap();
        let result = regexp_span_tokenize(test_string, &separator);
        let expected = vec![(0, 5), (6, 11)];
        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn spans_to_relative_test() {
        let test_span = vec![(0, 5), (6, 11)];
        let result = spans_to_relative(&test_span);
        let expected = vec![(0, 5), (1, 5)];
        assert_eq!(expected, result);
    }
}
