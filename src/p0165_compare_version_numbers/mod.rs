struct Solution;

impl Solution {
    fn revision_to_int(rev: &str) -> i32 {
        // trim all leading zeroes
        let valid = rev.trim_start_matches("0");
        if valid.len() == 0 {
            return 0;
        }
        valid.parse::<i32>().unwrap()
    }

    pub fn compare_version(version1: String, version2: String) -> i32 {
        let revisions_1 = version1.split(".").collect::<Vec<&str>>();
        let revisions_2 = version2.split(".").collect::<Vec<&str>>();

        let max_revs = revisions_1.len().max(revisions_2.len());

        for i in 0..max_revs {
            let mut rev_1 = 0;
            let mut rev_2 = 0;
            if i < revisions_1.len() {
                rev_1 = Solution::revision_to_int(revisions_1[i]);
            }

            if i < revisions_2.len() {
                rev_2 = Solution::revision_to_int(revisions_2[i]);
            }

            if rev_1 > rev_2 {
                return 1;
            }

            if rev_2 > rev_1 {
                return -1;
            }
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
