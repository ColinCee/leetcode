use std::{collections::HashMap, vec};
struct Solution;
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for num in nums1 {
            *map.entry(num).or_insert(0) += 1;
        }

        let mut result = Vec::new();
        // Check nums2 against the count
        for num in nums2 {
            if let Some(cnt) = map.get_mut(&num) {
                if *cnt > 0 {
                    result.push(num);
                    *cnt -= 1;
                }
            }
        }

        result
    }
}

pub fn solve() {
    // Implement the solution here
    let nums1 = vec![4, 9, 5];
    let nums2 = vec![9, 4, 9, 8, 4];
    let solution = Solution::intersect(nums1, nums2);

    println!("{:?}", solution)
}
