struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut total_matches = 0;
        let mut t_chars = t.chars();

        for char_s in s.chars() {
            while let Some(char_t) = t_chars.next() {
                if char_s == char_t {
                    total_matches += 1;
                    break;
                }
            }
        }

        return total_matches == s.len()
    }
}

pub fn solve() {
    let result = Solution::is_subsequence("abc".to_string(), "ahbgdc".to_string());

    println!("{}",result)
}
