pub struct Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let len = grid.len();
        let mut result = 0;
        let mut reads = vec![vec![false; grid[0].len()]; len];
        for (i, line) in grid.iter().enumerate() {
            for (j, c) in line.iter().enumerate() {
                if !reads[i][j] {
                    reads[i][j] = true;
                    if c == &'1' {
                        result += 1;
                        Solution::dfs(&grid, i + 1, j, &mut reads);
                        Solution::dfs(&grid, i - 1, j, &mut reads);
                        Solution::dfs(&grid, i, j + 1, &mut reads);
                        Solution::dfs(&grid, i, j - 1, &mut reads);
                    }
                }
            }
        }

        result
    }
    fn dfs(grid: &Vec<Vec<char>>, i: usize, j: usize, reads: &mut Vec<Vec<bool>>) {
        if i >= grid.len() || j >= grid[0].len() {
            return;
        }
        if reads[i][j] {
            return;
        }
        reads[i][j] = true;
        if grid[i][j] == '0' {
            return;
        }
        Solution::dfs(grid, i + 1, j, reads);
        Solution::dfs(grid, i - 1, j, reads);
        Solution::dfs(grid, i, j + 1, reads);
        Solution::dfs(grid, i, j - 1, reads);
    }
}

fn main() {
    let grid = vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
    ];
    assert_eq!(Solution::num_islands(grid), 1);
}
