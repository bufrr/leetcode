impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut first_num = i32::MAX;
        let mut second_num = i32::MAX;
        for n in nums {
            if n <= first_num {
                first_num = n;
            } else if n <= second_num {
                second_num = n;
            } else {
                return true;
            }
        }
        false
    }
}
