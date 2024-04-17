use crate::trees::{build_tree, print_tree_level_order, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    fn i32_to_char(val: &i32) -> char {
        if *val < 0 || *val > 25 {
            panic!("Input must be between 0 and 25, inclusive");
        } else {
            (*val as u8 + b'a') as char
        }
    }

    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut smallest = String::new();
        Self::dfs(&root, &mut Vec::new(), &mut smallest);
        smallest
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, current: &mut Vec<char>, smallest: &mut String) {
        if let Some(node_ref) = node {
            current.push(Solution::i32_to_char(&node_ref.borrow().val));

            if node_ref.borrow().left.is_none() && node_ref.borrow().right.is_none() {
                let current_str: String = current.iter().rev().collect();
                if smallest.is_empty() || current_str < *smallest {
                    *smallest = current_str;
                }
            } else {
                Self::dfs(&node_ref.borrow().left, current, smallest);
                Self::dfs(&node_ref.borrow().right, current, smallest);
            }

            current.pop();
        }
    }
}

pub fn solve() {
    // Implement the solution here
    // let nodes = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
    // let nodes = vec![0, 1, 2, 3, 4, 3, 4];
    // let nodes = vec![25, 1, 3, 1, 3, 0, 2];
    let nodes = vec![
        Some(2),
        Some(2),
        Some(1),
        None,
        Some(1),
        Some(0),
        None,
        Some(0),
    ];
    // let nodes = nodes.iter().map(|&x| Some(x)).collect::<Vec<_>>();
    println!("{:?}", nodes);
    let root = build_tree(&nodes);
    print_tree_level_order(&root);
    println!();

    let result = Solution::smallest_from_leaf(root);
    println!("result is: {:?}", result);
}
