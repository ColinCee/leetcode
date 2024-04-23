use std::collections::{HashSet, VecDeque};
struct Solution;

impl Solution {
    // Create an adjacency list of connections
    // Find all leaf nodes and add onto queue
    // Set remaining nodes = n
    // While num remaining nodes > 2
    // remaining nodes -= 1
    // While current leaf queue is not empty
    // pop leaf from queue
    // get neighbour of current
    // remove connection of current from neighbour
    // if neighbour now only has one connection left
    // add neighbour to new leaf queue
    // set leaf queue to new leaf queue
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n == 1 {
            return vec![0];
        }

        let n = n as usize;
        let mut adjacency_list = vec![HashSet::new(); n];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            adjacency_list[u].insert(v);
            adjacency_list[v].insert(u);
        }

        let mut leaves = VecDeque::new();
        for i in 0..n {
            if adjacency_list[i].len() == 1 {
                leaves.push_back(i);
            }
        }

        let mut remaining_nodes = n;
        while remaining_nodes > 2 {
            remaining_nodes -= leaves.len();

            let mut next_leaves = VecDeque::new();
            while let Some(leaf) = leaves.pop_front() {
                let neighbor = *adjacency_list[leaf].iter().next().unwrap();
                adjacency_list[neighbor].remove(&leaf);

                if adjacency_list[neighbor].len() == 1 {
                    next_leaves.push_back(neighbor);
                }
            }

            leaves = next_leaves;
        }

        leaves.into_iter().map(|x| x as i32).collect()
    }
}

pub fn solve() {
    let n = 6;
    let edges = vec![vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 4], vec![5, 4]];
    println!("Example 1: {:?}", Solution::find_min_height_trees(n, edges)); // Output: 16
}
