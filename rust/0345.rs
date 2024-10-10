impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        fn is_vowel(c: char) -> bool {
            "aeiouAEIOU".contains(c)
        }

        let mut res: Vec<char> = s.chars().collect();
        let mut left = 0;
        let mut right = res.len() - 1;

        while left < right {
            while left < res.len() && !is_vowel(res[left]) {
                left += 1;
            }
            while right > 0 && !is_vowel(res[right]) {
                right -= 1;
            }
            if left < right {
                res.swap(left, right);
                left += 1;
                right += 1;
            }
        }

        res.into_iter().collect()
    }
}
