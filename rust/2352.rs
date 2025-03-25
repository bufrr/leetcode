impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        let mut res = 0i32;
        let mut m_r = HashMap::new();

        for v in grid.iter() {
            let s = v.into_iter().map(|n| n.to_string()).collect::<Vec<String>>().join(",");
            *m_r.entry(s).or_insert(0) += 1;
        }
        
        for i in 0..grid.len() {
            let mut col = vec![0; grid.len()];
            for j in 0..grid.len() {
                col[j] = grid[j][i];
            }
            let key = col.into_iter().map(|n|n.to_string()).collect::<Vec<String>>().join(",");

            if let Some(count) = m_r.get(&key) {
                res += count;
            }
        }
        res
    }
}
