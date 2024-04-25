struct Solution;

impl Solution {
    pub fn longest_ideal_string(s: String, k: i32) -> i32 {
        let mut dp = vec![0; 26];

        for c in s.chars() {
            // map character from 0 to 25
            let idx = (c as u8 - b'a') as usize;
            let mut curr_len = 0;

            let start = (idx as i32 - k).max(0) as usize;
            let end = (idx as i32 + k).min(25) as usize;

            for i in start..=end {
                curr_len = curr_len.max(dp[i] + 1);
            }

            dp[idx] = curr_len;
        }

        dp.into_iter().max().unwrap()
    }
}

pub fn solve() {
    // Implement the solution here
    let string = "aaaaaaaaa";
    let k = 2;
    println!(
        "Example 1: {}",
        Solution::longest_ideal_string(string.to_string(), k)
    )
}
