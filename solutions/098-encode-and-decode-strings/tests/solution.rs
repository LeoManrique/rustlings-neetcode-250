include!("../src/lib.rs");

fn round_trip(strs: Vec<String>) -> Vec<String> {
    let encoded = Solution::encode(strs.clone());
    Solution::decode(encoded)
}

#[test]
fn canonical_example() {
    // LC: ["Hello","World"] -> encode -> decode == ["Hello","World"]
    let strs = vec!["Hello".to_string(), "World".to_string()];
    assert_eq!(round_trip(strs.clone()), strs);
}

#[test]
fn empty_list_round_trips() {
    let strs: Vec<String> = vec![];
    let encoded = Solution::encode(strs.clone());
    assert_eq!(encoded, "");
    assert_eq!(Solution::decode(encoded), strs);
}

#[test]
fn list_with_empty_strings() {
    let strs = vec!["".to_string(), "".to_string(), "".to_string()];
    assert_eq!(round_trip(strs.clone()), strs);
}

#[test]
fn strings_containing_delimiter_character() {
    // The '#' character must not break the encoding because each segment is
    // prefixed by its length.
    let strs = vec!["a#b".to_string(), "##".to_string(), "1#23#".to_string()];
    assert_eq!(round_trip(strs.clone()), strs);
}

#[test]
fn single_string_round_trips() {
    let strs = vec!["lonely".to_string()];
    assert_eq!(round_trip(strs.clone()), strs);
}

#[test]
fn mixed_lengths_and_special_chars() {
    let strs = vec![
        "".to_string(),
        "a".to_string(),
        "longer string with spaces".to_string(),
        "12345".to_string(),
        "###".to_string(),
    ];
    assert_eq!(round_trip(strs.clone()), strs);
}

#[test]
fn encoding_format_uses_length_hash_prefix() {
    let strs = vec!["ab".to_string(), "c".to_string()];
    assert_eq!(Solution::encode(strs), "2#ab1#c");
}
