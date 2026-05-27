pub struct Solution;

impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let n = piles.len();
        // suffix[i] = sum of piles[i..]
        let mut suffix = vec![0i32; n + 1];
        for i in (0..n).rev() {
            suffix[i] = suffix[i + 1] + piles[i];
        }
        // dp[i][m] = max stones the current player can obtain starting at i with parameter m.
        let mut dp = vec![vec![0i32; n + 1]; n + 1];
        for i in (0..n).rev() {
            for m in 1..=n {
                if i + 2 * m >= n {
                    dp[i][m] = suffix[i];
                } else {
                    let mut best = 0i32;
                    for x in 1..=2 * m {
                        // current player takes piles[i..i+x], opponent then plays from i+x with max(m,x)
                        let val = suffix[i] - dp[i + x][m.max(x)];
                        if val > best {
                            best = val;
                        }
                    }
                    dp[i][m] = best;
                }
            }
        }
        dp[0][1]
    }
}
