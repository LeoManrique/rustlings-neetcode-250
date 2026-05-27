pub struct Solution;

/// Wrapper around the resulting list of triplets so that test assertions of the
/// form `assert_eq!(..., vec![])` can infer the inner type from this side.
#[derive(Debug, Clone)]
pub struct Triplets(pub Vec<Vec<i32>>);

impl PartialEq<Vec<Vec<i32>>> for Triplets {
    fn eq(&self, other: &Vec<Vec<i32>>) -> bool {
        self.0 == *other
    }
}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Triplets {
        let mut nums = nums;
        nums.sort_unstable();
        let n = nums.len();
        let mut result: Vec<Vec<i32>> = Vec::new();
        for i in 0..n.saturating_sub(2) {
            if nums[i] > 0 {
                break;
            }
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let (mut l, mut r) = (i + 1, n - 1);
            while l < r {
                let sum = nums[i] + nums[l] + nums[r];
                if sum < 0 {
                    l += 1;
                } else if sum > 0 {
                    r -= 1;
                } else {
                    result.push(vec![nums[i], nums[l], nums[r]]);
                    l += 1;
                    r -= 1;
                    while l < r && nums[l] == nums[l - 1] {
                        l += 1;
                    }
                    while l < r && nums[r] == nums[r + 1] {
                        r -= 1;
                    }
                }
            }
        }
        Triplets(result)
    }
}
