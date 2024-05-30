impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let n = rooms.len();
        let mut visit = vec![false; n];
        let mut stack = vec![0];

        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            visit[node] = true;
            for &next in &rooms[node] {
                if !visit[next as usize] {
                    stack.push(next as usize);
                }
            }
        }

        visit.iter().all(|&v| v)
    }
}
