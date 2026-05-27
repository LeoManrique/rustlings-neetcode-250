pub struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() as i32;
        let mut c1: i32 = 0;
        let mut c2: i32 = 0;
        let mut n1: i32 = 0;
        let mut n2: i32 = 0;
        for &x in &nums {
            if n1 > 0 && x == c1 {
                n1 += 1;
            } else if n2 > 0 && x == c2 {
                n2 += 1;
            } else if n1 == 0 {
                c1 = x;
                n1 = 1;
            } else if n2 == 0 {
                c2 = x;
                n2 = 1;
            } else {
                n1 -= 1;
                n2 -= 1;
            }
        }
        let mut cnt1: i32 = 0;
        let mut cnt2: i32 = 0;
        for &x in &nums {
            if n1 > 0 && x == c1 {
                cnt1 += 1;
            } else if n2 > 0 && x == c2 {
                cnt2 += 1;
            }
        }
        let threshold = n / 3;
        let q1 = n1 > 0 && cnt1 > threshold;
        let q2 = n2 > 0 && cnt2 > threshold;
        let mut res: Vec<i32> = Vec::new();
        match (q1, q2) {
            (true, true) => {
                if cnt1 >= cnt2 {
                    res.push(c1);
                    res.push(c2);
                } else {
                    res.push(c2);
                    res.push(c1);
                }
            }
            (true, false) => res.push(c1),
            (false, true) => res.push(c2),
            (false, false) => {}
        }
        res
    }
}
