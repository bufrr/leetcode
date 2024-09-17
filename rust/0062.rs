impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut d: Vec<Vec<i32>> = vec![vec![1; n as usize]; m as usize];

        for col in 1..m {
            for row in 1..n {
                d[col as usize][row as usize] = d[col as usize - 1][row as usize]
                    + d[col as usize][row as usize - 1];
            }
        }
        d[m as usize - 1][n as usize - 1]
    }
}
