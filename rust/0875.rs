use std::cmp::max;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let right = 1;
        let mut left = 1;
        let m = piles.iter().max().unwrap().clone();
        let mut right = max(right, m);

        while left < right {
            let mid = (left + right) / 2;
            let mut hours = 0;

            hours += piles.iter().map(|&p| (p + mid - 1) / mid).sum::<i32>();
            if hours > h {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        right
    }
}
