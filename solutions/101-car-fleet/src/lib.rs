pub struct Solution;

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        // Sort cars by position descending (closest to target first). Compute
        // each car's solo arrival time. Iterating from front to back, a car
        // only forms a new fleet if its arrival time exceeds the slowest fleet
        // in front of it.
        let target = target as f64;
        let mut cars: Vec<(i32, i32)> = position.into_iter().zip(speed).collect();
        cars.sort_by(|a, b| b.0.cmp(&a.0));

        let mut fleets = 0i32;
        let mut slowest_time = 0.0f64;
        for (pos, spd) in cars {
            let time = (target - pos as f64) / spd as f64;
            if time > slowest_time {
                fleets += 1;
                slowest_time = time;
            }
        }
        fleets
    }
}
