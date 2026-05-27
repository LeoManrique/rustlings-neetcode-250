pub struct Solution;

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let (mut fives, mut tens) = (0i32, 0i32);
        for bill in bills {
            match bill {
                5 => fives += 1,
                10 => {
                    if fives == 0 {
                        return false;
                    }
                    fives -= 1;
                    tens += 1;
                }
                _ => {
                    if tens > 0 && fives > 0 {
                        tens -= 1;
                        fives -= 1;
                    } else if fives >= 3 {
                        fives -= 3;
                    } else {
                        return false;
                    }
                }
            }
        }
        true
    }
}
