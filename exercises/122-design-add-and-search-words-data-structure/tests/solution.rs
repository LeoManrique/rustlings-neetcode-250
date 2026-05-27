include!("../src/lib.rs");

#[test]
fn test_canonical_example() {
    let d = WordDictionary::new();
    d.add_word("bad".to_string());
    d.add_word("dad".to_string());
    d.add_word("mad".to_string());
    assert!(!d.search("pad".to_string()));
    assert!(d.search("bad".to_string()));
    assert!(d.search(".ad".to_string()));
    assert!(d.search("b..".to_string()));
}

#[test]
fn test_empty_dict_finds_nothing() {
    let d = WordDictionary::new();
    assert!(!d.search("a".to_string()));
    assert!(!d.search(".".to_string()));
}

#[test]
fn test_exact_match_only() {
    let d = WordDictionary::new();
    d.add_word("hello".to_string());
    assert!(d.search("hello".to_string()));
    assert!(!d.search("hell".to_string()));
    assert!(!d.search("helloo".to_string()));
}

#[test]
fn test_wildcard_matches_single_char() {
    let d = WordDictionary::new();
    d.add_word("at".to_string());
    assert!(d.search("..".to_string()));
    assert!(!d.search("...".to_string())); // wildcard requires same length
}

#[test]
fn test_all_wildcards() {
    let d = WordDictionary::new();
    d.add_word("abc".to_string());
    d.add_word("def".to_string());
    assert!(d.search("...".to_string()));
    assert!(!d.search("....".to_string()));
}

#[test]
fn test_mixed_wildcards_no_match() {
    let d = WordDictionary::new();
    d.add_word("cat".to_string());
    d.add_word("dog".to_string());
    assert!(!d.search("c..t".to_string()));
    assert!(d.search("c.t".to_string()));
}

#[test]
fn test_duplicate_adds_are_idempotent() {
    let d = WordDictionary::new();
    d.add_word("foo".to_string());
    d.add_word("foo".to_string());
    assert!(d.search("foo".to_string()));
    assert!(d.search("f.o".to_string()));
}
