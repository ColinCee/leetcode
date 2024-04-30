struct Solution;

impl Solution {
    // Use a bit array to represent all possible different strings of length 10
    // It is 2^10 = 1024, max 10 characters long, each either being 0 or 1
    // For each character in the word, it can have at most 10 characters
    // using XOR flip the bit at the ith position in the mask
    // increment result by the number of times we've seen this string
    // do the same for the other 9 bitmasks, XOR the current mask with (1 << i)
    pub fn wonderful_substrings(word: String) -> i64 {
        let mut count = vec![0; 1024];
        let mut result = 0;
        let mut mask = 0;

        count[0] = 1;

        for c in word.chars() {
            let index = (c as u8 - b'a') as usize;
            mask ^= 1 << index;

            result += count[mask];

            for i in 0..10 {
                result += count[mask ^ (1 << i)];
            }

            count[mask] += 1;
        }

        result
    }
}
pub fn solve() {
    // Implement the solution here
    let example = "aabb";
    println!("{}", Solution::wonderful_substrings(example.to_string()));
}
