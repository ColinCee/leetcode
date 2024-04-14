use std::cell::RefCell;
use std::rc::Rc;

use crate::trees::print_tree_level_order;
use crate::trees::build_tree;
use crate::trees::TreeNode;

fn sum_left_leaves(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
  if let Some(node) = node {
    let node = &node.borrow();
    return sum_left_leaves( &node.left) + node.val
  }

  return 0
}

pub fn solve() {
    let nodes = vec![1,2,3,4,5,6,7];
    let root = build_tree(&nodes, 0);

    print_tree_level_order(&root);
    println!();
    let sum = sum_left_leaves( &root);
    println!("{}", sum)
}
