impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut dp_grid = vec![vec![0; text2.len() + 1]; text1.len() + 1];

        for col in (0..text2.len()).rev() {
            for row in (0..text1.len()).rev() {
                if text1.as_bytes()[row] == text2.as_bytes()[col] {
                    dp_grid[row][col] = 1 + dp_grid[row + 1][col + 1];
                } else {
                    dp_grid[row][col] = dp_grid[row + 1][col].max(dp_grid[row][col + 1]);
                }
            }
        }
        dp_grid[0][0]
    }
}
