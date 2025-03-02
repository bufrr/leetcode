impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut left = 0;
        let mut right = height.len() - 1;

        while left < right {
            let width = right - left;
            let mut h = height[left];
            if height[right] < height[left] {
                h = height[right];
                right -=1;
            } else {
                left += 1;
            }
            let area = width * h as usize;
            if area > max {
                max = area
            }
        }
        max as i32
    }
}
