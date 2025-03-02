impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let mut max:i32 = 0;
        let mut sum:i32 = 0;
        fn is_vowels(c: char) -> bool {
            let c_lower = c.to_lowercase().next().unwrap();
            matches!(c_lower, 'a' | 'e' | 'i' | 'o' | 'u')
        }
        let b = s.as_bytes();
        for i in 0..b.len() {
            if is_vowels(b[i] as char) {
                sum += 1;
            }
            if i >= k as usize && is_vowels(b[i-k as usize] as char){
                sum -= 1;
            }
            if sum > max {
                max = sum
            }
            if max >= k as i32 {
                return k
            }
        }
        max
    }
}
