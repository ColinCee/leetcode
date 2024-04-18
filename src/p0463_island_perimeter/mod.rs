struct Solution;
impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut perimeter = 0;

        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == 0 {
                    continue;
                }
                // top
                if i == 0 || grid[i - 1][j] == 0 {
                    perimeter += 1;
                }
                // bottom
                if i == rows - 1 || grid[i + 1][j] == 0 {
                    perimeter += 1;
                }

                // left
                if j == 0 || grid[i][j - 1] == 0 {
                    perimeter += 1;
                }

                // right
                if j == cols - 1 || grid[i][j + 1] == 0 {
                    perimeter += 1;
                }
            }
        }

        return perimeter;
    }
}

pub fn solve() {
    let grid1 = vec![
        vec![0, 1, 0, 0],
        vec![1, 1, 1, 0],
        vec![0, 1, 0, 0],
        vec![1, 1, 0, 0],
    ];
    println!("Example 1: {}", Solution::island_perimeter(grid1)); // Output: 16
}
