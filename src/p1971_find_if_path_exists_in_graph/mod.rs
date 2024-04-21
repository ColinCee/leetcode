use std::collections::{HashMap, HashSet};

struct Solution;
impl Solution {
    // Starting from source, recursively dfs until destination is found
    // keep track of visited nodes
    // if visited ignore
    // return true on finding destination

    pub fn dfs(
        edge_map: &mut &HashMap<i32, HashSet<i32>>,
        visited: &mut HashSet<i32>,
        current: &i32,
        destination: &i32,
    ) -> bool {
        if visited.contains(&current) {
            return false;
        }
        visited.insert(current.clone());

        if current == destination {
            return true;
        }

        let connections = edge_map.get(&current);
        if connections.is_none() {
            return false;
        }

        let result = connections
            .unwrap()
            .iter()
            .any(|&conn| Solution::dfs(edge_map, visited, &conn, destination));

        result
    }
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut visited: HashSet<i32> = HashSet::new();
        let mut edge_map: HashMap<i32, HashSet<i32>> = HashMap::new();

        for edge in edges {
            edge_map
                .entry(edge[0])
                .or_insert_with(HashSet::new)
                .insert(edge[1]);
            edge_map
                .entry(edge[1])
                .or_insert_with(HashSet::new)
                .insert(edge[0]);
        }

        return Solution::dfs(&mut &edge_map, &mut visited, &source, &destination);
    }
}

pub fn solve() {
    // let edges = vec![vec![0, 1], vec![1, 2], vec![2, 0]];
    // println!("Example 1: {}", Solution::valid_path(3, edges, 0, 2)); // Output: 16
    let edges: Vec<Vec<i32>> = vec![vec![0, 1], vec![1, 2], vec![2, 0]];
    println!("Example 1: {}", Solution::valid_path(6, edges, 0, 5)); // Output: 16
}
