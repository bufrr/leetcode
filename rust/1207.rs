impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        use std::collections::HashMap;
        use std::collections::HashSet;

        let mut m = HashMap::new();
        let mut res = HashSet::new();

        for num in arr.iter() {
            *m.entry(num).or_insert(0) += 1
        }

        for (_, count) in &m {
            res.insert(count);
        } 

        m.len() == res.len()
    }
}
