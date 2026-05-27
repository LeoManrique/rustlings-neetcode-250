include!("../src/lib.rs");

#[test]
fn test_canonical_example() {
    // LeetCode example with capacity 2.
    let mut c = LFUCache::new(2);
    c.put(1, 1);
    c.put(2, 2);
    assert_eq!(c.get(1), 1); // key 1 freq=2
    c.put(3, 3); // evict key 2 (freq=1, LRU among min-freq)
    assert_eq!(c.get(2), -1);
    assert_eq!(c.get(3), 3); // freq of 3 -> 2
    c.put(4, 4); // evict key 1 (freq=2, LRU since 3 is more recent)
    assert_eq!(c.get(1), -1);
    assert_eq!(c.get(3), 3);
    assert_eq!(c.get(4), 4);
}

#[test]
fn test_get_missing_returns_negative_one() {
    let mut c = LFUCache::new(2);
    assert_eq!(c.get(99), -1);
}

#[test]
fn test_zero_capacity_is_no_op() {
    let mut c = LFUCache::new(0);
    c.put(1, 1);
    assert_eq!(c.get(1), -1);
}

#[test]
fn test_update_existing_key_updates_value() {
    let mut c = LFUCache::new(2);
    c.put(1, 10);
    c.put(1, 100);
    assert_eq!(c.get(1), 100);
}

#[test]
fn test_capacity_one_evicts() {
    let mut c = LFUCache::new(1);
    c.put(1, 1);
    assert_eq!(c.get(1), 1);
    c.put(2, 2);
    assert_eq!(c.get(1), -1);
    assert_eq!(c.get(2), 2);
}

#[test]
fn test_lfu_evicts_least_frequent() {
    let mut c = LFUCache::new(2);
    c.put(1, 1);
    c.put(2, 2);
    // Access 1 a few times: freq(1)=4, freq(2)=1
    c.get(1);
    c.get(1);
    c.get(1);
    // Insert new key -> should evict 2 (lower freq).
    c.put(3, 3);
    assert_eq!(c.get(2), -1);
    assert_eq!(c.get(1), 1);
    assert_eq!(c.get(3), 3);
}

#[test]
fn test_tie_breaks_by_lru() {
    let mut c = LFUCache::new(3);
    c.put(1, 1);
    c.put(2, 2);
    c.put(3, 3);
    // All have freq=1. Inserting a fourth evicts the oldest -> key 1.
    c.put(4, 4);
    assert_eq!(c.get(1), -1);
    assert_eq!(c.get(2), 2);
    assert_eq!(c.get(3), 3);
    assert_eq!(c.get(4), 4);
}
