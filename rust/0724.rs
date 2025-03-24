impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let length = nums.len();
        let mut left_sum = vec![0; length];
        let mut right_sum = vec![0; length];

        let mut l_sum = 0;
        let mut r_sum = 0;
        for (i, n) in nums.iter().enumerate() {
            l_sum += *n;
            left_sum[i] = l_sum;
        }
        for (i, n) in nums.iter().rev().enumerate() {
            r_sum += *n;
            right_sum[length - i - 1] = r_sum;
        }
        
        for i in 0..length {
            if left_sum[i] == right_sum[i] {
                return i as i32;
            }
        }
        -1
    }
}
