impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut ans = vec![0i32; n as usize + 1];
        for i in 1..=n {
            ans[i as usize] = ans[(i & (i - 1)) as usize] + 1;
        }
        ans
    }
}
