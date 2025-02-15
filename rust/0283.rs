impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut last_nonzero_index = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums[last_nonzero_index] = nums[i];
                last_nonzero_index += 1;
            }
        }
        while last_nonzero_index < nums.len() {
            nums[last_nonzero_index] = 0;
            last_nonzero_index += 1;
        }
    }
}
