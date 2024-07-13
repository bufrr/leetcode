impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut stack = Vec::new();
        let mut num = 0;
        let mut str = String::new();
        for c in s.chars() {
            match c {
                '0'..='9' => {
                    num = num * 10 + c.to_digit(10).unwrap() as usize;
                }
                '[' => {
                    stack.push((num, str.clone()));
                    num = 0;
                    str.clear();
                }
                ']' => {
                    let (count, prev) = stack.pop().unwrap();
                    str = prev + &*str.repeat(count);
                }
                _ => {
                    str.push(c);
                }
            }
        }
        str
    }
}
