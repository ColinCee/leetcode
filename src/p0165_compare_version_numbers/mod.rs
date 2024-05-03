struct Solution;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let v1: Vec<i32> = version1.split('.').map(|s| s.parse().unwrap()).collect();
        let v2: Vec<i32> = version2.split('.').map(|s| s.parse().unwrap()).collect();

        let mut i = 0;
        while i < v1.len() || i < v2.len() {
            let n1 = v1.get(i).unwrap_or(&0);
            let n2 = v2.get(i).unwrap_or(&0);

            if n1 < n2 {
                return -1;
            } else if n1 > n2 {
                return 1;
            }

            i += 1;
        }

        0
    }
}
pub fn solve() {
    // Implement the solution here
    let version1 = "1".to_string();
    let version2 = "11".to_string();
    println!("{}", Solution::compare_version(version1, version2))
}
