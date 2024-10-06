impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_by(|a,b| a[1].cmp(&b[1]));
        let mut res = 0;
        let mut k = i32::MIN;

        for i in 0..intervals.len() {
            let x = intervals[i][0];
            let y = intervals[i][1];

            if x >= k {
                k = y;
            } else {
                res += 1;
            }
        }
        res
    }
}
