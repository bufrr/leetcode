impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();
        let mut visited = vec![false; n];
        let mut res = 0;
        for i in 0..n {
            if !visited[i] {
                res += 1;
                Self::dfs(i, &is_connected, &mut visited);
            }
        }
        res
    }

    fn dfs(i: usize, is_connected: &Vec<Vec<i32>>, visited: &mut Vec<bool>) {
        visited[i] = true;
        for j in 0..is_connected.len() {
            if is_connected[i][j] == 1 && !visited[j] {
                Self::dfs(j, is_connected, visited);
            }
        }
    }
}
