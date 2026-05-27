pub struct Solution;

impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let k = k as usize;
        let n = arr.len();
        // Binary search for the smallest left such that the window arr[left..left+k]
        // is at least as close to x as the window arr[left+1..left+k+1].
        let (mut lo, mut hi) = (0usize, n - k);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if x - arr[mid] > arr[mid + k] - x {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        arr[lo..lo + k].to_vec()
    }
}
