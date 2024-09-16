impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let N = nums.len();
        if N == 0 {
            return 0;
        }

        let mut max_robbed_amount = vec![0; N+1];
        max_robbed_amount[N] = 0;
        max_robbed_amount[N-1] = nums[N-1];
        for i in (0..N-1).rev() {
            max_robbed_amount[i] = max_robbed_amount[i+1].max(max_robbed_amount[i+2] + nums[i]);
        }
        max_robbed_amount[0]
    }
}
