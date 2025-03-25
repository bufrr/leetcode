impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }
        use std::collections::HashMap;
        let mut m1 = HashMap::new();
        let mut m2 = HashMap::new();
        for c in word1.chars() {
            *m1.entry(c).or_insert(0) += 1;
        }
        for c in word2.chars() {
            *m2.entry(c).or_insert(0) += 1;
        }

        let mut v1: Vec<_> = m1.values().collect();
        let mut v2: Vec<_> = m2.values().collect();

        let mut k1: Vec<_> = m1.keys().collect();
        let mut k2: Vec<_> = m2.keys().collect();
        k1.sort();
        k2.sort();
        if k1 != k2 {
            return false;
        }

        v1.sort();
        v2.sort();

        v1 == v2 
    }
}
