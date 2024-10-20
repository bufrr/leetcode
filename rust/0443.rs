impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut index = 0;
        let mut res = 0;
        while index < chars.len() {
            let mut group_length = 1;
            while index + group_length < chars.len() && chars[index + group_length] == chars[index] {
                group_length += 1;
            }
            chars[res] = chars[index];
            res += 1;
            if group_length > 1 {
                for c in group_length.to_string().chars() {
                    chars[res] = c;
                    res += 1;
                }
            }
            index += group_length;
        }
        res as i32
    }
}
