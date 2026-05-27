pub struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        // Patience sorting: `tails[i]` is the smallest possible tail of any
        // increasing subsequence of length `i + 1`.
        let mut tails: Vec<i32> = Vec::with_capacity(nums.len());
        for n in nums {
            // Manual binary search for the leftmost index with tails[i] >= n.
            let (mut lo, mut hi) = (0usize, tails.len());
            while lo < hi {
                let mid = lo + (hi - lo) / 2;
                if tails[mid] < n {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
            if lo == tails.len() {
                tails.push(n);
            } else {
                tails[lo] = n;
            }
        }
        tails.len() as i32
    }
}
