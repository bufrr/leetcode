use std::collections::VecDeque;

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut fresh = 0;
        let mut queue = VecDeque::new();
        let mut grid = grid;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 2 {
                    queue.push_back((i, j));
                } else if grid[i][j] == 1 {
                    fresh += 1;
                }
            }
        }
        let mut minutes = 0;
        let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        while !queue.is_empty() {
            let mut new_queue = VecDeque::new();
            while let Some((i, j)) = queue.pop_front() {
                for (di, dj) in &directions {
                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;
                    if ni >= 0 && ni < grid.len() as i32 && nj >= 0 && nj < grid[0].len() as i32 {
                        let ni = ni as usize;
                        let nj = nj as usize;
                        if grid[ni][nj] == 1 {
                            grid[ni][nj] = 2;
                            fresh -= 1;
                            new_queue.push_back((ni, nj));
                        }
                    }
                }
            }
            if !new_queue.is_empty() {
                minutes += 1;
            }
            queue = new_queue;
        }
        if fresh == 0 {
            minutes
        } else {
            -1
        }
    }
}
