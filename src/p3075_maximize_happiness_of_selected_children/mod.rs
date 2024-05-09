use std::cmp::max;

struct Solution;

impl Solution {
    pub fn maximum_happiness_sum(happiness: Vec<i32>, k: i32) -> i64 {
        let mut sorted = happiness.clone();
        sorted.sort_unstable_by(|a, b| b.cmp(a));

        let mut sum: i64 = 0;
        for i in 0..k as usize {
            sum += max((sorted[i] as i64) - i as i64, 0);
        }
        sum
    }
}

pub fn solve() {
    // Implement the solution here
    let happiness = vec![1, 2, 3];
    let k = 2;

    let result = Solution::maximum_happiness_sum(happiness, k);

    println!("{}", result);
}
