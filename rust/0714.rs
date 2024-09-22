impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let n = prices.len();
        let mut free = vec![0; n];
        let mut hold = vec![0; n];

        hold[0] = -prices[0];

        for i in 1..n {
            hold[i] = hold[i-1].max(free[i-1] - prices[i]);
            free[i] = free[i-1].max(hold[i-1]+ prices[i] - fee);
        }

        free[n-1]
    }
}
