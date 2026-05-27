pub struct Solution;

impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let max_to = trips.iter().map(|t| t[2]).max().unwrap_or(0);
        let mut delta = vec![0_i32; (max_to as usize) + 1];
        for t in &trips {
            let (passengers, from, to) = (t[0], t[1] as usize, t[2] as usize);
            delta[from] += passengers;
            delta[to] -= passengers;
        }
        let mut load = 0_i32;
        for d in delta {
            load += d;
            if load > capacity {
                return false;
            }
        }
        true
    }
}
