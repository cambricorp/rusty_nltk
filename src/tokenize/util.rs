pub fn string_span_tokenize(s: &str, sep: &str) -> Result<Vec<(i32, i32)>, String> {
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

// TODO: Look up regexp API so we can finish this method...
/*pub fn regexp_span_tokenize(s: &str, regexp: &) -> Vec<(i32, i32)> {
    let mut left = 0;
    let mut result: Vec<(i32, i32)> = Vec::new();
    for m in finditr(regexp, s) {
        right, next = m.span();
        if right != 0 {
            result.push(left, right);
        }
        left = next;
    }
    result.push(left, strlen);
    result
}*/

pub fn spans_to_relative(spans: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut prev = 0;
    let mut result: Vec<(i32, i32)> = Vec::new();
    for tuple in spans.iter() {
        let (left, right) = tuple.to_owned();
        result.push( (left - prev, right - left) );
        prev = right;
    }
    result
}
