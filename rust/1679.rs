impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut res = 0;

        while left < right {
            let sum = nums[left] + nums[right];
            if sum == k {
                res += 1;
                left += 1;
                right -= 1;
            } else if sum > k {
                right -= 1;
            } else {
                left += 1;
            }
        }
        res
    }
}
