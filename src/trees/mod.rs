use std::collections::VecDeque;

#[derive(Debug)]
pub struct Node {
    pub val: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    pub fn new(val: i32) -> Self {
        Node {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn build_tree(nodes: &[i32], index: usize) -> Option<Box<Node>> {
  if index >= nodes.len() {
      return None;
  }

  let val = nodes[index];
  let mut node = Box::new(Node::new(val));

  node.left = build_tree(nodes, 2 * index + 1);
  node.right = build_tree(nodes, 2 * index + 2);
  Some(node)
}

// Function to traverse the binary tree and print the values in BFS order
pub fn print_values(root: &Option<Box<Node>>) {
  if root.is_none() {
      return;
  }

  let mut queue = VecDeque::new();
  queue.push_back(root);

  while !queue.is_empty() {
      let node = queue.pop_front().unwrap();

      if let Some(node) = node {
          print!("{} ", node.val);

          if node.left.is_some() {
              queue.push_back(&node.left);
          }

          if node.right.is_some() {
              queue.push_back(&node.right);
          }
      }
  }
}
