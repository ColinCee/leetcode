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

pub fn build_tree(nodes: &[Option<i32>], index: usize) -> Option<Rc<RefCell<TreeNode>>> {
  if index >= nodes.len() || nodes[index].is_none() {
      None
  } else {
      let node = Rc::new(RefCell::new(TreeNode::new(nodes[index].unwrap())));
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
      queue.push_back(Some(node.clone()));

      while !queue.is_empty() {
          if let Some(current_node_opt) = queue.pop_front() {
              if let Some(current_node) = current_node_opt {
                  print!("{} ", current_node.borrow().val);

                  let current_node_ref = current_node.borrow();
                  queue.push_back(current_node_ref.left.clone());
                  queue.push_back(current_node_ref.right.clone());
              } else {
                  print!("null ");
              }
          }
      }
      println!();
  }
}
