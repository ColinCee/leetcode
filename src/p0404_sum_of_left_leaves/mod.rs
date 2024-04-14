use crate::trees::print_values;
use crate::trees::build_tree;
use crate::trees::Node;

fn invert_tree(node: &mut Option<Box<Node>>) {
  if let Some(node) = node.as_mut() {
    std::mem::swap(&mut node.left, &mut node.right);

    invert_tree(&mut node.left);
    invert_tree(&mut node.right);
  }
}

pub fn solve() {
    let nodes = vec![1,2,3,4,5,6,7];
    let mut root = build_tree(&nodes, 0);
    
    print_values(&root);
    println!();
    invert_tree(&mut root);
    print_values(&root);
}
