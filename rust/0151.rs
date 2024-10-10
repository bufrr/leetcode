impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut s = s.split_whitespace().collect::<Vec<_>>();
        s.reverse();
        s.join(" ")
    }
}
