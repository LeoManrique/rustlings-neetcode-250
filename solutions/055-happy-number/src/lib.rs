pub struct Solution;

impl Solution {
    pub fn is_happy(n: i64) -> bool {
        let mut slow = n;
        let mut fast = Self::next(n);
        while fast != 1 && slow != fast {
            slow = Self::next(slow);
            fast = Self::next(Self::next(fast));
        }
        fast == 1
    }

    fn next(mut n: i64) -> i64 {
        let mut sum = 0i64;
        while n > 0 {
            let d = n % 10;
            sum += d * d;
            n /= 10;
        }
        sum
    }
}
