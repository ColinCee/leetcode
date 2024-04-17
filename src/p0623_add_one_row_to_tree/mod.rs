use std::cell::RefCell;
use std::rc::Rc;

use crate::trees::{build_tree, print_tree_level_order, TreeNode};

struct Solution;

impl Solution {
    pub fn dfs_add_row(
        node: &mut Option<Rc<RefCell<TreeNode>>>,
        val: i32,
        depth: i32,
        current_depth: i32,
    ) {
        if let Some(node) = node {
            let mut node = node.borrow_mut();
            println!("{}-{}", node.val, current_depth);
            if current_depth < depth - 1 {
                Solution::dfs_add_row(&mut node.left, val, depth, current_depth + 1);
                Solution::dfs_add_row(&mut node.right, val, depth, current_depth + 1);
                return;
            }
            // TODO replace this with returning all nodes that need to be updated and update all at once
            if current_depth == depth - 1 {
                // Create a new node
                let mut new_node_left = TreeNode::new(val);
                new_node_left.left = node.left.clone();
                let mut new_node_right = TreeNode::new(val);
                new_node_right.right = node.right.clone();

                node.left = Some(Rc::new(RefCell::new(new_node_left)));
                node.right = Some(Rc::new(RefCell::new(new_node_right)));

                println!("{:?}", &node);
                return;
            }
        }

        return;
    }
    pub fn add_one_row(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
        depth: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if depth == 1 {
            let mut new_root = TreeNode::new(val);
            new_root.left = root.clone();
            return Some(Rc::new(RefCell::new(new_root)));
        }
        let mut new_root = root.clone();
        Solution::dfs_add_row(&mut new_root, val, depth, 1);
        return new_root;
    }
}

pub fn solve() {
    // let nodes = vec![Some(4),Some(2),None,Some(3),Some(1)];
    let nodes = vec![4, 2, 6, 3, 1, 5];
    let some_nodes = nodes.iter().map(|&x| Some(x)).collect::<Vec<_>>();
    let root = build_tree(&some_nodes);
    print_tree_level_order(&root);

    let new_root = Solution::add_one_row(root, 1, 1);

    print_tree_level_order(&new_root);

    println!();
}
