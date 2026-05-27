pub struct Solution;

impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut record: Vec<i32> = Vec::with_capacity(operations.len());
        for op in &operations {
            match op.as_str() {
                "+" => {
                    let n = record.len();
                    record.push(record[n - 1] + record[n - 2]);
                }
                "D" => {
                    let last = *record.last().unwrap();
                    record.push(last * 2);
                }
                "C" => {
                    record.pop();
                }
                s => record.push(s.parse::<i32>().unwrap()),
            }
        }
        record.iter().sum()
    }
}
