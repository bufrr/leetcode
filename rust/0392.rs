impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let left_bound = s.len();
        let right_bound = t.len();
        pub fn rec_is_subsequence(s: String, t: String, mut left: usize, mut right: usize) -> bool {
            if left == s.len() {
                return true;
            }
            if right == t.len() {
                return false;
            }
            if s.as_bytes()[left] == t.as_bytes()[right] {
                left += 1;
            }
            right += 1;

            rec_is_subsequence(s, t, left, right)
        }

        rec_is_subsequence(s, t, 0, 0)
    }
}

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let left_bound = s.len();
        let right_bound = t.len();

        let mut left = 0;
        let mut right = 0;

        while left < left_bound && right < right_bound {
            if s.as_bytes()[left] == t.as_bytes()[right] {
                left += 1;
            }
            right += 1;
        }

        return left == left_bound;
    }
}
