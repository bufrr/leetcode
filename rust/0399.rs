impl Solution {
    pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
        let mut graph: HashMap<String, HashMap<String, f64>> = HashMap::new();

        // Build the graph
        for (i, eq) in equations.iter().enumerate() {
            let a = &eq[0];
            let b = &eq[1];
            let value = values[i];

            graph.entry(a.clone()).or_insert(HashMap::new()).insert(b.clone(), value);
            graph.entry(b.clone()).or_insert(HashMap::new()).insert(a.clone(), 1.0 / value);
        }

        // Process queries
        queries.iter().map(|query| {
            let start = &query[0];
            let end = &query[1];
            Self::dfs(start, end, &graph, &mut vec![])
        }).collect()
    }

    fn dfs(start: &str, end: &str, graph: &HashMap<String, HashMap<String, f64>>, visited: &mut Vec<String>) -> f64 {
        if !graph.contains_key(start) || !graph.contains_key(end) {
            return -1.0;
        }

        if start == end {
            return 1.0;
        }

        visited.push(start.to_string());

        if let Some(neighbors) = graph.get(start) {
            for (neighbor, &value) in neighbors {
                if !visited.contains(&neighbor.to_string()) {
                    let result = Self::dfs(neighbor, end, graph, visited);
                    if result != -1.0 {
                        visited.pop();
                        return result * value;
                    }
                }
            }
        }

        visited.pop();
        -1.0
    }
}
