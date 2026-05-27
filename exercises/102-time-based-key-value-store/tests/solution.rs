include!("../src/lib.rs");

#[test]
fn canonical_example() {
    // LC: set("foo","bar",1); get("foo",1)="bar"; get("foo",3)="bar";
    //     set("foo","bar2",4); get("foo",4)="bar2"; get("foo",5)="bar2"
    let tm = TimeMap::new();
    tm.set("foo".into(), "bar".into(), 1);
    assert_eq!(tm.get("foo".into(), 1), "bar");
    assert_eq!(tm.get("foo".into(), 3), "bar");
    tm.set("foo".into(), "bar2".into(), 4);
    assert_eq!(tm.get("foo".into(), 4), "bar2");
    assert_eq!(tm.get("foo".into(), 5), "bar2");
}

#[test]
fn get_missing_key_returns_empty_string() {
    let tm = TimeMap::new();
    assert_eq!(tm.get("nope".into(), 1), "");
}

#[test]
fn get_before_first_timestamp_returns_empty_string() {
    let tm = TimeMap::new();
    tm.set("k".into(), "v".into(), 10);
    assert_eq!(tm.get("k".into(), 5), "");
    assert_eq!(tm.get("k".into(), 9), "");
    assert_eq!(tm.get("k".into(), 10), "v");
}

#[test]
fn get_returns_floor_timestamp_value() {
    let tm = TimeMap::new();
    tm.set("k".into(), "a".into(), 1);
    tm.set("k".into(), "b".into(), 5);
    tm.set("k".into(), "c".into(), 10);
    assert_eq!(tm.get("k".into(), 1), "a");
    assert_eq!(tm.get("k".into(), 4), "a");
    assert_eq!(tm.get("k".into(), 5), "b");
    assert_eq!(tm.get("k".into(), 7), "b");
    assert_eq!(tm.get("k".into(), 10), "c");
    assert_eq!(tm.get("k".into(), 100), "c");
}

#[test]
fn keys_are_independent() {
    let tm = TimeMap::new();
    tm.set("a".into(), "alpha".into(), 1);
    tm.set("b".into(), "beta".into(), 2);
    assert_eq!(tm.get("a".into(), 5), "alpha");
    assert_eq!(tm.get("b".into(), 5), "beta");
    assert_eq!(tm.get("c".into(), 5), "");
}

#[test]
fn supports_many_sets_for_one_key() {
    let tm = TimeMap::new();
    for ts in 1..=20 {
        tm.set("x".into(), format!("v{}", ts), ts);
    }
    assert_eq!(tm.get("x".into(), 1), "v1");
    assert_eq!(tm.get("x".into(), 13), "v13");
    assert_eq!(tm.get("x".into(), 20), "v20");
    assert_eq!(tm.get("x".into(), 0), "");
}
