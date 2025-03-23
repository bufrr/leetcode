impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x < 2 {
            return x;
        }
        let mut left = 2;
        let mut right = x/2;
        let mut pivot = 2;
        while left <= right {
            pivot = left + (right - left) / 2;
            let num = pivot as u64 * pivot as u64;
            if num > x as u64{
                right = pivot - 1;
            } else if num < x as u64{
                left = pivot + 1;
            } else {
                return pivot;
            }
        }
        right
    }
}
