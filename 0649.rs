impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut radiant = Vec::new();
        let mut dire = Vec::new();
        let n = senate.len();
        for (i, c) in senate.chars().enumerate() {
            if c == 'R' {
                radiant.push(i);
            } else {
                dire.push(i);
            }
        }
        while !radiant.is_empty() && !dire.is_empty() {
            if radiant[0] < dire[0] {
                radiant.push(radiant[0] + n);
            } else {
                dire.push(dire[0] + n);
            }
            radiant.remove(0);
            dire.remove(0);
        }
        if radiant.is_empty() {
            "Dire".to_string()
        } else {
            "Radiant".to_string()
        }
    }
}
