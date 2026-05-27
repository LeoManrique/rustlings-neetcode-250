pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // Ensure nums1 is the shorter array, so binary search runs in O(log(min(m,n))).
        let (a, b) = if nums1.len() <= nums2.len() {
            (&nums1, &nums2)
        } else {
            (&nums2, &nums1)
        };
        let m = a.len();
        let n = b.len();
        let total = m + n;
        let half = (total + 1) / 2;

        let mut lo = 0usize;
        let mut hi = m;
        loop {
            let i = (lo + hi) / 2; // count from a in the left half
            let j = half - i; // count from b in the left half

            let a_left = if i == 0 { i32::MIN } else { a[i - 1] };
            let a_right = if i == m { i32::MAX } else { a[i] };
            let b_left = if j == 0 { i32::MIN } else { b[j - 1] };
            let b_right = if j == n { i32::MAX } else { b[j] };

            if a_left <= b_right && b_left <= a_right {
                return if total % 2 == 1 {
                    a_left.max(b_left) as f64
                } else {
                    (a_left.max(b_left) as f64 + a_right.min(b_right) as f64) / 2.0
                };
            } else if a_left > b_right {
                hi = i - 1;
            } else {
                lo = i + 1;
            }
        }
    }
}
