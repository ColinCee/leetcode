struct Solution;
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut grid = grid;
        let mut count = 0;
        let rows = grid.len();
        let cols = grid[0].len();

        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == '1' {
                    count += 1;
                    Solution::dfs(&mut grid, i, j);
                }
            }
        }

        count
    }

    fn dfs(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
        let rows = grid.len();
        let cols = grid[0].len();

        if i >= rows || j >= cols || grid[i][j] != '1' {
            return;
        }

        grid[i][j] = '0';

        Solution::dfs(grid, i + 1, j);
        Solution::dfs(grid, i, j + 1);
        Solution::dfs(grid, i.saturating_sub(1), j);
        Solution::dfs(grid, i, j.saturating_sub(1));
    }
}
pub fn solve() {
    let grid: Vec<Vec<char>> = vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
    ];
    println!("Example 1: {}", Solution::num_islands(grid));
}
