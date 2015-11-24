#[cfg(test)]
mod test_simple {
    
    use tokenize::api::TokenizerI;
    use tokenize::api::StringTokenizer;

    #[test]
    fn test_space_tokenize() {
        let s = "Good muffins cost $3.88\nin New York.  Please buy me\ntwo of them.\n\nThanks.";
        let space_tokenizer = StringTokenizer{ string: " " };
        let result = space_tokenizer.tokenize(s).unwrap();
        let expected = vec!["Good", "muffins", "cost", "$3.88\nin", "New", "York.", "",
        "Please", "buy", "me\ntwo", "of", "them.\n\nThanks."];
        assert_eq!(expected, result);
    }

    #[test]
    fn test_tab_tokenizer() {
        let s = "a\tb c\n\t d";
        let tab_tokenizer = StringTokenizer{ string: "\t" };
        let result = tab_tokenizer.tokenize(s).unwrap();
        let expected = vec!["a", "b c\n", " d"];
        assert_eq!(expected, result);
    }
}
