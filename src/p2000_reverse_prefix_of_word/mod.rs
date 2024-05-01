struct Solution;

impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let char_index = word.find(ch);

        if char_index.is_none() {
            return word;
        }

        let mut left = 0;
        let mut right = char_index.unwrap();
        let mut chars: Vec<char> = word.chars().collect();

        while left < right {
            chars.swap(left, right);
            left += 1;
            right -= 1;
        }

        chars.into_iter().collect()
    }
}

pub fn solve() {
    // Implement the solution here
    let word = "abcd".to_string();
    let ch = 'z';

    println!("{}", Solution::reverse_prefix(word, ch))
}
