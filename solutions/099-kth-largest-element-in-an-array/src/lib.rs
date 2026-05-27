pub struct Solution;

impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        // kth largest = element at index (n - k) when sorted ascending.
        let n = nums.len();
        let target = n - k as usize;
        quickselect(&mut nums, 0, n - 1, target)
    }
}

fn quickselect(nums: &mut [i32], mut lo: usize, mut hi: usize, target: usize) -> i32 {
    loop {
        if lo >= hi {
            return nums[lo];
        }
        // Median-of-three pivot to avoid worst-case on sorted input.
        let mid = lo + (hi - lo) / 2;
        if nums[lo] > nums[mid] {
            nums.swap(lo, mid);
        }
        if nums[lo] > nums[hi] {
            nums.swap(lo, hi);
        }
        if nums[mid] > nums[hi] {
            nums.swap(mid, hi);
        }
        let pivot = nums[mid];
        // 3-way partition (Dutch National Flag) on [lo..=hi].
        let mut lt = lo;
        let mut gt = hi;
        let mut i = lo;
        while i <= gt {
            if nums[i] < pivot {
                nums.swap(lt, i);
                lt += 1;
                i += 1;
            } else if nums[i] > pivot {
                nums.swap(i, gt);
                if gt == 0 {
                    break;
                }
                gt -= 1;
            } else {
                i += 1;
            }
        }
        // After partition: nums[lo..lt] < pivot, nums[lt..=gt] == pivot, nums[gt+1..=hi] > pivot.
        if target < lt {
            hi = lt - 1;
        } else if target <= gt {
            return pivot;
        } else {
            lo = gt + 1;
        }
    }
}
