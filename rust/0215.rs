impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = std::collections::BinaryHeap::from(nums);
        let mut result = 0;
        for _ in 0..k {
            result = heap.pop().unwrap();
        }
        result
    }
}
