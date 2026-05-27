// FIXME: The reference test uses todo!() and the LeetCode API `MountainArray` is
// not provided in this workspace. Define a minimal stub so the crate builds; the
// canonical algorithm performs three binary searches: find peak, search ascending
// half, search descending half.
pub struct Solution;

pub struct MountainArray {
    data: Vec<i32>,
}

impl MountainArray {
    pub fn new(data: Vec<i32>) -> Self {
        Self { data }
    }
    pub fn get(&self, index: i32) -> i32 {
        self.data[index as usize]
    }
    pub fn length(&self) -> i32 {
        self.data.len() as i32
    }
}

impl Solution {
    pub fn find_in_mountain_array(target: i32, mountain_arr: &MountainArray) -> i32 {
        let n = mountain_arr.length();
        // Find peak via binary search.
        let (mut lo, mut hi) = (0, n - 1);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if mountain_arr.get(mid) < mountain_arr.get(mid + 1) {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        let peak = lo;
        // Ascending search [0..=peak].
        if let Some(idx) = Self::bsearch(mountain_arr, 0, peak, target, true) {
            return idx;
        }
        // Descending search [peak..=n-1].
        Self::bsearch(mountain_arr, peak, n - 1, target, false).unwrap_or(-1)
    }

    fn bsearch(arr: &MountainArray, mut lo: i32, mut hi: i32, target: i32, ascending: bool) -> Option<i32> {
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            let v = arr.get(mid);
            if v == target {
                return Some(mid);
            }
            let go_right = if ascending { v < target } else { v > target };
            if go_right {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
        None
    }
}
