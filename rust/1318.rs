impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        let mut answer = 0;
        let mut a = a;
        let mut b = b;
        let mut c = c;
        while (a!=0) | (b!=0) | (c!=0) {
            if (c & 1) == 1 {
                if (a & 1) == 0 && (b & 1) == 0 {
                    answer+=1;
                }
            } else {
                answer += (a & 1) + (b & 1);
            }
            a >>= 1;
            b >>= 1;
            c >>= 1;
        }
        answer
    }
}
