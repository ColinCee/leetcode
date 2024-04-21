struct Solution;

impl Solution {
    // for all rows 0..grid len
    // for all cols 0..grid[row] len
    // if cell == 1 mark start
    // recursive dfs to find last cell
    //      change cell to 0 after visiting
    //      if right and bottom of cell is 0
    //         return location

    pub fn dfs(land: &mut Vec<Vec<i32>>, row: usize, col: usize) -> Option<(usize, usize)> {
        if row >= land.len() || col >= land[row].len() {
            return None;
        }
        let cell = land[row][col];
        if cell == 0 {
            return None;
        }

        land[row][col] = 0;

        if row + 1 == land.len() || land[row + 1][col] == 0 {
            if col + 1 == land[row].len() || land[row][col + 1] == 0 {
                return Some((row, col));
            }
        }

        let right = Solution::dfs(land, row, col + 1);
        let down = Solution::dfs(land, row + 1, col);

        right.or(down)
    }
    pub fn find_farmland(land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut land = land.clone();
        let mut output: Vec<Vec<i32>> = vec![];

        for row in 0..land.len() {
            for col in 0..land[row].len() {
                let cell = land[row][col];
                if cell == 0 {
                    continue;
                }

                let mut result = vec![row as i32, col as i32];
                let end = Solution::dfs(&mut land, row, col).unwrap();
                result.extend(vec![end.0 as i32, end.1 as i32]);

                output.push(result);
            }
        }
        output
    }
}

pub fn solve() {
    // let land: Vec<Vec<i32>> = vec![vec![1, 0, 0], vec![0, 1, 1], vec![0, 1, 1]];
    let land: Vec<Vec<i32>> = vec![vec![1, 0], vec![1, 0]];
    println!("Example 1: {:?}", Solution::find_farmland(land));
}
