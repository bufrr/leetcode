use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        let mut total_cost: i64 = 0;
        let n = costs.len();
        let candidates = candidates as usize;
        
        let mut heap = BinaryHeap::new();
        let mut left = 0;
        let mut right = n.saturating_sub(1);
        
        // Initialize heap with the first and last 'candidates' workers
        for i in 0..candidates.min((n + 1) / 2) {
            if left <= right {
                heap.push(Reverse((costs[left], left, true)));
                left += 1;
            }
            if left <= right {
                heap.push(Reverse((costs[right], right, false)));
                right -= 1;
            }
        }
        
        // Hire k workers
        for _ in 0..k {
            if let Some(Reverse((cost, index, is_left))) = heap.pop() {
                total_cost += cost as i64;
                if left <= right {
                    if is_left {
                        heap.push(Reverse((costs[left], left, true)));
                        left += 1;
                    } else {
                        heap.push(Reverse((costs[right], right, false)));
                        right -= 1;
                    }
                }
            }
        }
        
        total_cost
    }
}

