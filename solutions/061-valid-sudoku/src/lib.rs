pub struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = [0u16; 9];
        let mut cols = [0u16; 9];
        let mut boxes = [0u16; 9];

        for (r, row) in board.iter().enumerate() {
            for (c, &ch) in row.iter().enumerate() {
                if ch == '.' {
                    continue;
                }
                let bit = 1u16 << (ch as u8 - b'1');
                let b = (r / 3) * 3 + c / 3;
                if rows[r] & bit != 0 || cols[c] & bit != 0 || boxes[b] & bit != 0 {
                    return false;
                }
                rows[r] |= bit;
                cols[c] |= bit;
                boxes[b] |= bit;
            }
        }
        true
    }
}
