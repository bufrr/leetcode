impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (m, n) = (word1.len(), word2.len());
        let mut dp = vec![vec![0; n + 1]; m + 1];

        for i in 0..=m {
            dp[i][0] = i;
        }
        for j in 0..=n {
            dp[0][j] = j;
        }

        for i in 1..=m {
            for j in 1..=n {
                let cost = if word1.chars().nth(i - 1) == word2.chars().nth(j - 1) {
                    0
                } else {
                    1
                };
                dp[i][j] = (dp[i - 1][j - 1] + cost)
                    .min(dp[i - 1][j] + 1)
                    .min(dp[i][j - 1] + 1);
            }
        }

        dp[m][n] as i32
    }
}
