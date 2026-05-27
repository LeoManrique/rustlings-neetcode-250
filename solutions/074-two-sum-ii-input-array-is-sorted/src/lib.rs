use std::fmt;

#[derive(Debug, Clone)]
pub struct TwoSumResult(pub Option<Vec<i32>>);

impl PartialEq<Vec<i32>> for TwoSumResult {
    fn eq(&self, other: &Vec<i32>) -> bool {
        matches!(&self.0, Some(v) if v == other)
    }
}

impl PartialEq<Option<i32>> for TwoSumResult {
    fn eq(&self, other: &Option<i32>) -> bool {
        other.is_none() && self.0.is_none()
    }
}

pub struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> TwoSumResult {
        let n = numbers.len();
        for i in 0..n {
            let need = target - numbers[i];
            // binary search for `need` in numbers[i+1..]
            let mut lo = i + 1;
            let mut hi = n;
            while lo < hi {
                let mid = lo + (hi - lo) / 2;
                if numbers[mid] < need {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
            if lo < n && numbers[lo] == need {
                return TwoSumResult(Some(vec![i as i32 + 1, lo as i32 + 1]));
            }
        }
        TwoSumResult(None)
    }
}

impl fmt::Display for TwoSumResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
