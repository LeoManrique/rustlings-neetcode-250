pub struct Solution;

/**
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return       -1 if num is higher than the picked number
 *                1 if num is lower than the picked number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

unsafe fn guess(_num: i32) -> i32 {
    0
}

impl Solution {
    #[allow(non_snake_case)]
    unsafe fn guessNumber(n: i32) -> i32 {
        let (mut lo, mut hi) = (1i32, n);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            match unsafe { guess(mid) } {
                0 => return mid,
                -1 => hi = mid - 1,
                _ => lo = mid + 1,
            }
        }
        lo
    }
}
