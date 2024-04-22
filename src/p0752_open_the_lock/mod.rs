use std::collections::{HashSet, VecDeque};

struct Solution;

impl Solution {
    // Using BFS
    // starting from 0000
    // while queue not empty
    // pop from queue
    // if current is visited
    // continue
    // if current is a deadlock state
    // continue
    // if current is target
    // return path size
    // add all next states onto queue
    // return - 1
    pub fn get_next_states(state: &String) -> Vec<String> {
        state
            .chars()
            .enumerate()
            .flat_map(|(i, ch)| {
                let up = ((ch as u8 - b'0' + 1) % 10 + b'0') as char;
                let down = ((ch as u8 - b'0' + 9) % 10 + b'0') as char;
                vec![
                    state[..i].to_string() + &up.to_string() + &state[i + 1..],
                    state[..i].to_string() + &down.to_string() + &state[i + 1..],
                ]
            })
            .collect()
    }

    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let deadends: HashSet<String> = deadends.into_iter().collect();
        let mut visited: HashSet<String> = HashSet::new();
        let mut queue: VecDeque<(String, i32)> = VecDeque::new();

        queue.push_back(("0000".to_string(), 0));

        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            if visited.contains(&current.0) {
                continue;
            }
            if deadends.contains(&current.0) {
                continue;
            }
            if current.0 == target {
                return current.1;
            }
            visited.insert(current.0.clone());

            queue.extend(
                Solution::get_next_states(&current.0)
                    .into_iter()
                    .map(|v| (v, current.1 + 1)),
            );
        }

        -1
    }
}

pub fn solve() {
    // Implement the solution here
    let deadends = [
        "8887", "8889", "8878", "8898", "8788", "8988", "7888", "9888",
    ];
    let target = "8888";
    let deadends = vec!["0201", "0101", "0102", "1212", "2002"]
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let target = "0202".to_string();

    println!("Example 1: {}", Solution::open_lock(deadends, target)); // Output: 16
}
