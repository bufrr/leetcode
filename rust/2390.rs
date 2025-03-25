impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut stack = Vec::new();
        for i in s.chars() {
            if i != '*' {
                stack.push(i);
            } else {
                stack.pop();
            }
        }
        stack.into_iter().collect()
    }
}
