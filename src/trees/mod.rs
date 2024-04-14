use std::{cell::RefCell, collections::VecDeque, rc::Rc};

#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
      TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn build_tree(nodes: &[i32], index: usize) -> Option<Rc<RefCell<TreeNode>>> {
  if index >= nodes.len() || nodes[index] == -1 {
      None
  } else {
      let node = Rc::new(RefCell::new(TreeNode::new(nodes[index])));
      let left_index = 2 * index + 1;
      let right_index = 2 * index + 2;

      node.borrow_mut().left = build_tree(nodes, left_index);
      node.borrow_mut().right = build_tree(nodes, right_index);

      Some(node)
  }
}

pub fn print_tree_level_order(root: &Option<Rc<RefCell<TreeNode>>>) {
  if let Some(node) = root {
      let mut queue = VecDeque::new();
      queue.push_back(node.clone());

      while !queue.is_empty() {
        if let Some(current_node) = queue.pop_front() {
          print!("{} ", current_node.borrow().val); 

          let current_node_ref = current_node.borrow();
          if let Some(left_node) = &current_node_ref.left {
              queue.push_back(left_node.clone());
          }
          if let Some(right_node) = &current_node_ref.right {
              queue.push_back(right_node.clone());
          }
        }

      }

      println!();
  }
}
