include!("../src/lib.rs");

// In this test environment, the forward-declared `guess(num)` LeetCode hook is
// stubbed to always return 0 (meaning "every guess matches"). The binary search
// therefore returns on the very first probe `mid = lo + (hi - lo) / 2` with
// lo = 1 and hi = n. These tests assert that exact deterministic behavior.

#[test]
#[allow(non_snake_case)]
fn returns_one_when_n_is_one() {
    let got = unsafe { Solution::guessNumber(1) };
    assert_eq!(got, 1);
}

#[test]
#[allow(non_snake_case)]
fn picks_first_midpoint_for_small_n() {
    // n=2 -> lo=1, hi=2, mid = 1 + (2-1)/2 = 1
    let got = unsafe { Solution::guessNumber(2) };
    assert_eq!(got, 1);
}

#[test]
#[allow(non_snake_case)]
fn picks_first_midpoint_for_canonical_example() {
    // LC example uses n=10, pick=6. With stubbed guess returning 0, the search
    // returns the first probed midpoint: 1 + (10-1)/2 = 5.
    let got = unsafe { Solution::guessNumber(10) };
    assert_eq!(got, 5);
}

#[test]
#[allow(non_snake_case)]
fn returns_value_in_range_for_large_n() {
    let n = 2_147_483_647i32;
    let got = unsafe { Solution::guessNumber(n) };
    assert!(got >= 1 && got <= n, "expected guess in [1, {}], got {}", n, got);
}

#[test]
#[allow(non_snake_case)]
fn returns_first_midpoint_for_n_three() {
    // n=3 -> mid = 1 + (3-1)/2 = 2
    let got = unsafe { Solution::guessNumber(3) };
    assert_eq!(got, 2);
}

#[test]
#[allow(non_snake_case)]
fn returns_first_midpoint_for_n_one_hundred() {
    // n=100 -> mid = 1 + (100-1)/2 = 50
    let got = unsafe { Solution::guessNumber(100) };
    assert_eq!(got, 50);
}
