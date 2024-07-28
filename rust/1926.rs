use std::collections::VecDeque;

impl Solution {
    pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let (rows, cols) = (maze.len() as i32, maze[0].len() as i32);
        let (start_row, start_col) = (entrance[0], entrance[1]);
        
        // Define directions: up, down, left, right
        let directions = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        
        // Queue for BFS
        let mut queue = VecDeque::new();
        queue.push_back((start_row, start_col, 0)); // (row, col, steps)
        
        // Visited set to keep track of explored cells
        let mut visited = vec![vec![false; cols as usize]; rows as usize];
        visited[start_row as usize][start_col as usize] = true;
        
        while let Some((row, col, steps)) = queue.pop_front() {
            // Check if we've reached an exit
            if (row == 0 || row == rows - 1 || col == 0 || col == cols - 1) && 
               (row != start_row || col != start_col) {
                return steps;
            }
            
            // Explore neighbors
            for (dx, dy) in &directions {
                let new_row = row + dx;
                let new_col = col + dy;
                
                if new_row >= 0 && new_row < rows && new_col >= 0 && new_col < cols &&
                   !visited[new_row as usize][new_col as usize] && 
                   maze[new_row as usize][new_col as usize] == '.' {
                    visited[new_row as usize][new_col as usize] = true;
                    queue.push_back((new_row, new_col, steps + 1));
                }
            }
        }
        
        -1 // No exit found
    }
}

