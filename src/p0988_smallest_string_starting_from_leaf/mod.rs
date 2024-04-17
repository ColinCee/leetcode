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

    // DFS
    // Keep track of current string for until first leaf, reverse it
    // Recursively find next leaf, return early if depth is > current min string length
    // If string len is shorter than min replace with current
    // If all characters are less than min, replace with current
    pub fn recursive_min_string(
        node: &Option<Rc<RefCell<TreeNode>>>,
        prev_string: &Option<Vec<char>>,
    ) -> Option<Vec<char>> {
        if let Some(node) = node {
            let node_ref = node.borrow();
            let TreeNode { left, right, val } = &*node_ref;
            // println!("Current {}", Solution::i32_to_char(val));

            let current_string = match prev_string {
                Some(s) => {
                    let mut s = s.clone();
                    s.push(Solution::i32_to_char(val));
                    Some(s)
                }
                None => Some(vec![Solution::i32_to_char(val)]),
            };

            if left.is_none() && right.is_none() {
                return current_string;
            }

            let min_left = Solution::recursive_min_string(left, &current_string.clone());
            let min_right = Solution::recursive_min_string(right, &current_string.clone());

            // There's at least one min here
            if min_left.is_some() && min_right.is_none() {
                return min_left;
            }

            if min_right.is_some() && min_left.is_none() {
                return min_right;
            }

            // Lengths are not always equal
            let min_left_rev = min_left.as_ref().unwrap().iter().rev().collect::<Vec<_>>();
            let min_right_rev = min_right.as_ref().unwrap().iter().rev().collect::<Vec<_>>();

            return if min_left_rev < min_right_rev {
                min_left
            } else {
                min_right
            };
        }

        return None;
    }

    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let vec = Solution::recursive_min_string(&root, &None);
        let vec = vec.expect("No result found"); // Unwrap the Option<Vec<char>>
        let s: String = vec.into_iter().collect();
        let reversed_s: String = s.chars().rev().collect();
        return reversed_s;
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
