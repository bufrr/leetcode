impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut graph = vec![vec![]; n as usize];
        let mut edges = std::collections::HashSet::new();

        for connection in &connections {
            let from = connection[0] as usize;
            let to = connection[1] as usize;
            graph[from].push(to);
            graph[to].push(from);
            edges.insert((from, to));
        }

        let mut visited = vec![false; n as usize];
        let mut res = 0;
        Self::dfs(&graph, 0, &mut visited, &mut res, &edges);
        res
    }

    fn dfs(graph: &[Vec<usize>], node: usize, visited: &mut [bool], res: &mut i32, edges: &std::collections::HashSet<(usize, usize)>) {
        visited[node] = true;
        for &next in &graph[node] {
            if !visited[next] {
                if edges.contains(&(node, next)) {
                    *res += 1;
                }
                Self::dfs(graph, next, visited, res, edges);
            }
        }
    }
}
