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
        let _ = (target, mountain_arr);
        todo!()
    }
}
