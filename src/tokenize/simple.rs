use tokenize::api::StringTokenizer;

pub struct SpaceTokenizerBuilder {
    _string: &'static str,
}

impl SpaceTokenizerBuilder {
    pub fn new() -> SpaceTokenizerBuilder {
        SpaceTokenizerBuilder {
            _string: " ",
        }
    }

    pub fn build(self) -> StringTokenizer {
        StringTokenizer {
            string: self._string,
        }
    }
}

pub struct TabTokenizerBuilder {
    _string: &'static str,
}

impl TabTokenizerBuilder {
    pub fn new() -> TabTokenizerBuilder {
        TabTokenizerBuilder {
            _string: "\t",
        }
    }

    pub fn build(self) -> StringTokenizer {
        StringTokenizer {
            string: self._string,
        }
    }
}

#[cfg(test)]
mod test_util {
    use tokenize::api::Tokenizer;
    use super::{SpaceTokenizerBuilder, TabTokenizerBuilder};

    #[test]
    fn test_space_tokenize() {
        let s = "Good muffins cost $3.88\nin New York.  Please buy me\ntwo of them.\n\nThanks.";
        let tokenizer = SpaceTokenizerBuilder::new().build();
        let result = tokenizer.tokenize(s).unwrap();
        let expected = vec!["Good", "muffins", "cost", "$3.88\nin", "New", "York.", "",
        "Please", "buy", "me\ntwo", "of", "them.\n\nThanks."];
        assert_eq!(expected, result);
    }

    #[test]
    fn test_tab_tokenizer() {
        let s = "a\tb c\n\t d";
        let tokenizer = TabTokenizerBuilder::new().build();
        let result = tokenizer.tokenize(s).unwrap();
        let expected = vec!["a", "b c\n", " d"];
        assert_eq!(expected, result);
    }
}
