impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut flowerbed = flowerbed;
        let mut count = 0;
        let length = flowerbed.len();

        for i in 0..length {
            if flowerbed[i] == 0 {
                let prev = if i == 0 { 0 } else { flowerbed[i - 1] };
                let next = if i == length - 1 { 0 } else { flowerbed[i + 1] };

                if prev == 0 && next == 0 {
                    flowerbed[i] = 1;
                    count += 1;
                }
            }
        }
        count >= n
    }
}
