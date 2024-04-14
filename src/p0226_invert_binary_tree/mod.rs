use std::cell::RefCell;
use std::rc::Rc;

use crate::trees::print_tree_level_order;
use crate::trees::build_tree;
use crate::trees::TreeNode;

fn invert_tree(node: &mut Option<Rc<RefCell<TreeNode>>>) {
  if let Some(node) = node {
    let mut node_borrowed = node.borrow_mut();
    let left = std::mem::replace(&mut node_borrowed.left, None);
    let right = std::mem::replace(&mut node_borrowed.right, None);
    node_borrowed.left = right;
    node_borrowed.right = left;

    invert_tree(&mut node_borrowed.left);
    invert_tree(&mut node_borrowed.right);
}
}
pub fn solve() {
    let nodes = vec![1,2,3,4,5,6,7];
    let some_nodes = nodes.iter().map(|&x| Some(x)).collect::<Vec<_>>();
    let mut root = build_tree(&some_nodes, 0);
    print_tree_level_order(&root);
    println!();
    invert_tree(&mut root);
    print_tree_level_order(&root);
}
