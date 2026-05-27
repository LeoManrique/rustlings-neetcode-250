include!("../src/lib.rs");

#[test]
fn test_canonical_example() {
    let trie = Trie::new();
    trie.insert("apple".to_string());
    assert!(trie.search("apple".to_string()));
    assert!(!trie.search("app".to_string()));
    assert!(trie.starts_with("app".to_string()));
    trie.insert("app".to_string());
    assert!(trie.search("app".to_string()));
}

#[test]
fn test_empty_trie_finds_nothing() {
    let trie = Trie::new();
    assert!(!trie.search("anything".to_string()));
    assert!(!trie.starts_with("a".to_string()));
}

#[test]
fn test_search_partial_word_is_false() {
    let trie = Trie::new();
    trie.insert("hello".to_string());
    assert!(!trie.search("hell".to_string()));
    assert!(!trie.search("hellos".to_string()));
    assert!(trie.starts_with("hell".to_string()));
}

#[test]
fn test_multiple_words_sharing_prefix() {
    let trie = Trie::new();
    trie.insert("car".to_string());
    trie.insert("card".to_string());
    trie.insert("care".to_string());
    assert!(trie.search("car".to_string()));
    assert!(trie.search("card".to_string()));
    assert!(trie.search("care".to_string()));
    assert!(trie.starts_with("ca".to_string()));
    assert!(!trie.search("ca".to_string()));
    assert!(!trie.search("cars".to_string()));
}

#[test]
fn test_starts_with_returns_false_for_unknown_prefix() {
    let trie = Trie::new();
    trie.insert("dog".to_string());
    assert!(!trie.starts_with("cat".to_string()));
    assert!(!trie.search("dogs".to_string()));
}

#[test]
fn test_reinsert_same_word_idempotent() {
    let trie = Trie::new();
    trie.insert("rust".to_string());
    trie.insert("rust".to_string());
    assert!(trie.search("rust".to_string()));
    assert!(trie.starts_with("rus".to_string()));
}

#[test]
fn test_single_character_words() {
    let trie = Trie::new();
    trie.insert("a".to_string());
    assert!(trie.search("a".to_string()));
    assert!(trie.starts_with("a".to_string()));
    assert!(!trie.search("b".to_string()));
}
