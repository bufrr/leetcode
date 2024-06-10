impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        if n == 2 {
            return 1;
        }
        let n = n as usize;
        let mut dp = vec![0, 1, 1];
        for i in 3..n {
            dp.push(dp[i-1]+dp[i-2]+dp[i-3]);
        }
        dp[n-1]+dp[n-2]+dp[n-3]
    }
}
