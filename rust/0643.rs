impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut sum = 0;
        for i in 0..k {
            sum += nums[i as usize];
        }
        let mut res = sum;
        let length = nums.len();
        let k = k as usize;
        for i in (k as usize)..length {
            sum += nums[i] - nums[i-k];
            if sum > res {
                res = sum
            }
        }

        res as f64 / k as f64
    }
}
