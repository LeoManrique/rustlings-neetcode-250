pub struct Solution;

impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let len = nums.len();
        if len < 2 {
            return nums;
        }
        let mut buffer = vec![0i32; len];
        merge_sort(&mut nums, &mut buffer, 0, len);
        nums
    }
}

fn merge_sort(data: &mut [i32], buffer: &mut [i32], lo: usize, hi: usize) {
    if hi - lo < 2 {
        return;
    }
    let mid = lo + (hi - lo) / 2;
    merge_sort(data, buffer, lo, mid);
    merge_sort(data, buffer, mid, hi);
    merge(data, buffer, lo, mid, hi);
}

fn merge(data: &mut [i32], buffer: &mut [i32], lo: usize, mid: usize, hi: usize) {
    let (mut i, mut j, mut k) = (lo, mid, lo);
    while i < mid && j < hi {
        if data[i] <= data[j] {
            buffer[k] = data[i];
            i += 1;
        } else {
            buffer[k] = data[j];
            j += 1;
        }
        k += 1;
    }
    while i < mid {
        buffer[k] = data[i];
        i += 1;
        k += 1;
    }
    while j < hi {
        buffer[k] = data[j];
        j += 1;
        k += 1;
    }
    data[lo..hi].copy_from_slice(&buffer[lo..hi]);
}
