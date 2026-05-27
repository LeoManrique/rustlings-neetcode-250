include!("../src/lib.rs");

#[test]
fn empty_map_returns_negative_one() {
    let m = MyHashMap::new();
    assert_eq!(m.get(0), -1);
    assert_eq!(m.get(123), -1);
}

#[test]
fn canonical_example() {
    // LC: put(1,1), put(2,2), get(1)=1, get(3)=-1, put(2,1), get(2)=1,
    //     remove(2), get(2)=-1
    let mut m = MyHashMap::new();
    m.put(1, 1);
    m.put(2, 2);
    assert_eq!(m.get(1), 1);
    assert_eq!(m.get(3), -1);
    m.put(2, 1);
    assert_eq!(m.get(2), 1);
    m.remove(2);
    assert_eq!(m.get(2), -1);
}

#[test]
fn put_overwrites_existing_value() {
    let mut m = MyHashMap::new();
    m.put(5, 10);
    assert_eq!(m.get(5), 10);
    m.put(5, 20);
    assert_eq!(m.get(5), 20);
    m.put(5, 0);
    assert_eq!(m.get(5), 0);
}

#[test]
fn remove_missing_key_is_a_noop() {
    let mut m = MyHashMap::new();
    m.remove(42); // shouldn't panic
    m.put(1, 100);
    m.remove(2);
    assert_eq!(m.get(1), 100);
}

#[test]
fn handles_keys_that_collide_on_buckets() {
    let mut m = MyHashMap::new();
    m.put(0, 1);
    m.put(1024, 2);
    m.put(2048, 3);
    assert_eq!(m.get(0), 1);
    assert_eq!(m.get(1024), 2);
    assert_eq!(m.get(2048), 3);
    m.remove(1024);
    assert_eq!(m.get(0), 1);
    assert_eq!(m.get(1024), -1);
    assert_eq!(m.get(2048), 3);
}

#[test]
fn large_workload_round_trips() {
    let mut m = MyHashMap::new();
    for k in 0..200 {
        m.put(k, k * 10);
    }
    for k in 0..200 {
        assert_eq!(m.get(k), k * 10);
    }
    for k in 0..200 {
        m.remove(k);
    }
    for k in 0..200 {
        assert_eq!(m.get(k), -1);
    }
}
