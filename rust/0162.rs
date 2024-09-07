impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let length = nums.len();
        Self::search(nums, 0, length as i32 - 1)
    }

    pub fn search(nums: Vec<i32>, left: i32, right: i32) -> i32 {
        if left == right {
            return left;
        }
        let mid = (left + right) / 2;
        if nums[mid as usize] > nums[mid as usize + 1] {
            return Self::search(nums, left, mid);
        }
        Self::search(nums, mid + 1, right)
    }
}

