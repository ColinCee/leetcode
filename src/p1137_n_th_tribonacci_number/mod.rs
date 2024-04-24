struct Solution;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut output = vec![0, 1, 1];
        if n < 3 {
            return output[n as usize];
        }
        for i in 3..=n {
            let index = i as usize;
            let new_val = output[index - 1] + output[index - 2] + output[index - 3];
            output.push(new_val)
        }
        println!("{:?}", output);
        output[n as usize]
    }
}

pub fn solve() {
    let n = 6;
    println!("n: {}, result {}", n, Solution::tribonacci(n))
}
