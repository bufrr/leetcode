use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let n = nums1.len();
        let k = k as usize;

        // Create a vector of pairs (nums2[i], nums1[i])
        let mut pairs: Vec<(i32, i32)> = nums2.into_iter().zip(nums1.into_iter()).collect();

        // Sort pairs in descending order based on nums2 values
        pairs.sort_unstable_by(|a, b| b.0.cmp(&a.0));

        let mut sum: i64 = 0;
        let mut min_heap = BinaryHeap::new();
        let mut max_score: i64 = 0;

        for (num2, num1) in pairs {
            sum += num1 as i64;
            min_heap.push(Reverse(num1));

            if min_heap.len() > k {
                sum -= min_heap.pop().unwrap().0 as i64;
            }

            if min_heap.len() == k {
                max_score = max_score.max(sum * num2 as i64);
            }
        }

        max_score
    }
}
