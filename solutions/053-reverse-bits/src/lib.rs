pub struct Solution;

impl Solution {
    pub fn reverse_bits(n: u128) -> u128 {
        // Reverse the lower 32 bits manually using bit-by-bit construction.
        let mut input = n as u32;
        let mut output: u32 = 0;
        for _ in 0..32 {
            output = (output << 1) | (input & 1);
            input >>= 1;
        }
        output as u128
    }
}
