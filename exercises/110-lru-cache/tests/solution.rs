include!("../src/lib.rs");

#[test]
fn test_canonical_example() {
    let mut cache = LRUCache::new(2);
    cache.put(1, 1);
    cache.put(2, 2);
    assert_eq!(cache.get(1), 1);
    cache.put(3, 3); // evicts key 2
    assert_eq!(cache.get(2), -1);
    cache.put(4, 4); // evicts key 1
    assert_eq!(cache.get(1), -1);
    assert_eq!(cache.get(3), 3);
    assert_eq!(cache.get(4), 4);
}

#[test]
fn test_get_missing_returns_negative_one() {
    let mut cache = LRUCache::new(3);
    assert_eq!(cache.get(42), -1);
}

#[test]
fn test_update_existing_key() {
    let mut cache = LRUCache::new(2);
    cache.put(1, 10);
    cache.put(1, 20);
    assert_eq!(cache.get(1), 20);
}

#[test]
fn test_capacity_one() {
    let mut cache = LRUCache::new(1);
    cache.put(1, 1);
    assert_eq!(cache.get(1), 1);
    cache.put(2, 2);
    assert_eq!(cache.get(1), -1);
    assert_eq!(cache.get(2), 2);
}

#[test]
fn test_get_refreshes_recency() {
    let mut cache = LRUCache::new(2);
    cache.put(1, 1);
    cache.put(2, 2);
    assert_eq!(cache.get(1), 1);
    cache.put(3, 3);
    assert_eq!(cache.get(2), -1);
    assert_eq!(cache.get(1), 1);
    assert_eq!(cache.get(3), 3);
}

#[test]
fn test_update_does_not_evict() {
    let mut cache = LRUCache::new(2);
    cache.put(1, 1);
    cache.put(2, 2);
    cache.put(1, 100);
    assert_eq!(cache.get(2), 2);
    assert_eq!(cache.get(1), 100);
}
