struct Solution;
impl Solution {
    fn print_2d_vec(vec: &Vec<Vec<i32>>) {
        for row in vec {
            for item in row {
                print!("{} ", item);
            }
            println!();
        }
        println!("-----")
    }

    pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        // init dp array of same size as grid
        let mut dp: Vec<Vec<i32>> = vec![vec![i32::MAX; grid[0].len()]; grid.len()];
        dp[0] = grid[0].clone();

        for row in 1..grid.len() {
            for col in 0..grid[row].len() {
                // for each cell calculate what the current min is so far
                let mut current_min = dp[row][col];
                for col_2 in 0..grid[row].len() {
                    if col_2 == col {
                        continue;
                    }
                    let prev_row_cell = dp[row - 1][col_2];
                    current_min = current_min.min(prev_row_cell + grid[row][col])
                }

                dp[row][col] = current_min;
                Solution::print_2d_vec(&dp);
            }
        }

        *dp[dp.len() - 1].iter().min().unwrap()
    }
}
pub fn solve() {
    // Implement the solution here
    // let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let grid = vec![vec![7]];
    println!("Output {}", Solution::min_falling_path_sum(grid));
}
