pub struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() {
            return false;
        }
        // Locate the row whose range may contain target via binary search on first column.
        let (mut lo, mut hi) = (0usize, matrix.len());
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if matrix[mid][0] <= target {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        if lo == 0 {
            return false;
        }
        let row = &matrix[lo - 1];
        // Binary search within that row.
        let (mut l, mut r) = (0usize, row.len());
        while l < r {
            let mid = l + (r - l) / 2;
            match row[mid].cmp(&target) {
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Less => l = mid + 1,
                std::cmp::Ordering::Greater => r = mid,
            }
        }
        false
    }
}
