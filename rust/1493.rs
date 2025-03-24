impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = 0;
        let length = nums.len();
        let mut k = 1;

        while right < length {
            if nums[right] == 0 {
                k -= 1;
            }
            if k < 0 {
                if nums[left] == 0 {
                    k += 1;
                }
                left += 1;
            }
            right += 1;
        }
        (right - left - 1) as i32
    }
}
