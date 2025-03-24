impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;

        let mut m1: HashSet<i32> = nums1.clone().into_iter().collect();
        let mut m2: HashSet<i32> = nums2.clone().into_iter().collect();
        let mut v1 = Vec::new();
        let mut v2 = Vec::new();

        for n in nums1.iter() {
            if !m2.contains(n) {
                v1.push(*n);
                m2.insert(*n);
            }
        }
        for n in nums2.iter() {
            if !m1.contains(n) {
                v2.push(*n);
                m1.insert(*n);
            }
        }

        vec![v1, v2]
    }
}
