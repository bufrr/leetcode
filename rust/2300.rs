impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        potions.sort_unstable();
        let m = potions.len();

        spells.into_iter().map(|spell| {
            let spell = spell as i64;
            let target = (success + spell - 1) / spell; // Ceiling division

            if target > *potions.last().unwrap() as i64 {
                0
            } else {
                (m - potions.partition_point(|&p| (p as i64 * spell) < success)) as i32
            }
        }).collect()
    }
}
