pub struct Solution;

impl Solution {
    pub fn merge(mut nums1: Vec<i32>, m: i32, nums2: Vec<i32>, n: i32) -> Option<Vec<i32>> {
        let (mut i, mut j, mut k) = (m as isize - 1, n as isize - 1, m as isize + n as isize - 1);
        while j >= 0 {
            if i >= 0 && nums1[i as usize] > nums2[j as usize] {
                nums1[k as usize] = nums1[i as usize];
                i -= 1;
            } else {
                nums1[k as usize] = nums2[j as usize];
                j -= 1;
            }
            k -= 1;
        }
        None
    }
}
