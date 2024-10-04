use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn divide_players(skill: Vec<i32>) -> i64 {
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        let mut num_count: HashMap<i32, i32> = HashMap::new();
        let target;

        for i in skill {
            if i < min {
                min = i
            }
            if i > max {
                max = i
            }
            *num_count.entry(i).or_insert(0) += 1
        }
        target = max + min;
        // println!("target {}", target);
        let values: Vec<i32> = num_count.keys().cloned().collect();
        let mut sum: i64 = 0;

        for &key in &values {
            let count = num_count.get(&key).unwrap();
            let complement = target - key;

            let comp_count = match num_count.get(&complement) {
                Some(&value) => value,
                None => return -1,
            };

            if *count != comp_count {
                return -1;
            }
            if *count == 0 {
                continue;
            }
            println!("key {} complement {}", key, complement);
            let mut score = i64::from(key) * i64::from(complement);

            match key == complement {
                true => score *= i64::from(*count / 2),
                false => score *= i64::from(*count),
            }
            sum += score;
            num_count.insert(key, 0);
            num_count.insert(complement, 0);
        }
        return sum;
    }
}

pub fn solve() {
    let skill = vec![1, 1, 5, 4, 5, 2];

    let result = Solution::divide_players(skill);

    println!("{}", result);
}
