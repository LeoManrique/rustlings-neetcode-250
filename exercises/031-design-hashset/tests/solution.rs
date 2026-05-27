include!("../src/lib.rs");

#[test]
fn empty_set_contains_nothing() {
    let s = MyHashSet::new();
    assert!(!s.contains(0));
    assert!(!s.contains(42));
}

#[test]
fn canonical_example() {
    // LC: add(1), add(2), contains(1)=true, contains(3)=false,
    //     add(2), contains(2)=true, remove(2), contains(2)=false
    let s = MyHashSet::new();
    s.add(1);
    s.add(2);
    assert!(s.contains(1));
    assert!(!s.contains(3));
    s.add(2);
    assert!(s.contains(2));
    s.remove(2);
    assert!(!s.contains(2));
}

#[test]
fn add_is_idempotent() {
    let s = MyHashSet::new();
    s.add(7);
    s.add(7);
    s.add(7);
    assert!(s.contains(7));
    s.remove(7);
    assert!(!s.contains(7));
}

#[test]
fn remove_missing_key_is_a_noop() {
    let s = MyHashSet::new();
    s.remove(123); // shouldn't panic
    assert!(!s.contains(123));
    s.add(1);
    s.remove(2); // different key
    assert!(s.contains(1));
}

#[test]
fn handles_keys_that_collide_on_buckets() {
    // 0 and BUCKETS-sized stride share a bucket; this exercises the chain logic.
    let s = MyHashSet::new();
    s.add(0);
    s.add(1024);
    s.add(2048);
    assert!(s.contains(0));
    assert!(s.contains(1024));
    assert!(s.contains(2048));
    s.remove(1024);
    assert!(s.contains(0));
    assert!(!s.contains(1024));
    assert!(s.contains(2048));
}

#[test]
fn supports_large_range_of_keys() {
    let s = MyHashSet::new();
    for k in 0..200 {
        s.add(k);
    }
    for k in 0..200 {
        assert!(s.contains(k));
    }
    for k in 200..220 {
        assert!(!s.contains(k));
    }
}
