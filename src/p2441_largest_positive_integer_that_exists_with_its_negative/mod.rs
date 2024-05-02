struct Solution;

impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let num_set: HashSet<i32> = nums.iter().cloned().collect();
        let mut max_k = -1;

        for &num in &nums {
            if num > 0 && num_set.contains(&-num) {
                max_k = max_k.max(num);
            }
        }

        max_k
    }
}
pub fn solve() {
    // Implement the solution here
    // let nums = vec![-1, 2, -3, 3];
    // let nums = vec![-1, 10, 6, 7, -7, 1];
    let nums = vec![
        -30, 34, 1, 32, 26, -9, -30, 22, -1, -8, 11, -37, 32, 40, 9, 15, -34, -34, -16, -5, 26,
        -44, -36, -13, -16, 10, 39, -17, -22, 17, -16,
    ];
    // let nums = vec![0];
    println!("{}", Solution::find_max_k(nums))
}
