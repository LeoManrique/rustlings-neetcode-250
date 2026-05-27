// FIXME: tests 39 and 69 contradict the lower-bound semantics of LC #35 (same inputs expect different outputs than tests 16/14). Solution uses standard lower_bound, which satisfies the LeetCode definition.
pub struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        nums.partition_point(|&x| x < target) as i32
    }
}
