use std::cell::RefCell;
use std::rc::Rc;

use crate::trees::print_tree_level_order;
use crate::trees::build_tree;
use crate::trees::TreeNode;

fn sum_left_leaves(node: &Option<Rc<RefCell<TreeNode>>>, should_count: bool) -> i32 {
  if let Some(node) = node {
    let node = &node.borrow();
    if node.left.is_none() && node.right.is_none() && should_count{
      return node.val
    }

    return sum_left_leaves( &node.left, true) + sum_left_leaves(&node.right, false)
  }

  return 0
}

pub fn solve() {
    let nodes = vec![Some(3),Some(9),Some(20),None,None,Some(15),Some(7)];
    let root = build_tree(&nodes, 0);

    print_tree_level_order(&root);
    println!();
    let sum = sum_left_leaves( &root, false);
    println!("{}", sum)
}
