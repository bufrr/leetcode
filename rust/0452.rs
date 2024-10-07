impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        if points.len() == 0 {
            return 0
        }
        points.sort_by(|a,b| a[1].cmp(&b[1]));
        let mut res = 1;
        let mut x_start = points[0][1];
        let mut x_end = points[0][1];
        let mut first_end = points[0][1];
        for p in points.iter() {
            x_start = p[0];
            x_end = p[1];
            
            if first_end < x_start {
                res += 1;
                first_end = x_end;
            }
        }

        res
    }
}
