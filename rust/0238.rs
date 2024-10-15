impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![0; nums.len()];
        let mut left = vec![0; nums.len()];
        let mut right = vec![0; nums.len()];

        left[0] = 1;
        for i in 1..nums.len() {
            left[i] = left[i - 1] * nums[i - 1]
        }

        right[nums.len() - 1] = 1;
        for i in (0..nums.len()-1).rev() {
            right[i] = right[i + 1] * nums[i + 1];
        }

        for i in 0..nums.len() {
            res[i] = left[i] * right[i]
        }

        res
    }
}
