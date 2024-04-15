use std::{cell::RefCell, rc::Rc};

use crate::trees::{build_tree, print_tree_level_order, TreeNode};


/*
 * Had to change this to a Vec<String> because storing Vec<i32> causes a bug
 * where if you prepend with two or more zeros 00 that's just parsed as 0
 * but it could've been a non zero after that node e.g 300 instead of 30
 */
pub fn recursive_sum(node: &Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
  if let Some(node) = node {
    if node.borrow().left.is_none() && node.borrow().right.is_none() {
      return vec![node.borrow().val.to_string()]
    }
    let left = recursive_sum(&node.borrow().left);
    let right = recursive_sum(&node.borrow().right);

    let combined = [left, right].concat();

    return combined.iter().map(|num| {
      format!("{}{}",node.borrow().val, num).parse().unwrap()
    }).collect();
  }

  vec![]
}
pub fn sum_numbers(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let result = recursive_sum(&node);
    return result.iter().map(|s| s.parse::<i32>().unwrap()).sum();
}

pub fn solve() {
  let nodes: Vec<Option<i32>> = vec![Some(5), Some(3), Some(2), Some(7), Some(0), Some(6), None, None, None, Some(0)];
  // let some_nodes = nodes.iter().map(|&x| Some(x)).collect::<Vec<_>>();
  let root = build_tree(&nodes, 0);
  print_tree_level_order(&root);

  let sum = sum_numbers(root);

  println!("Sum is {}", sum);
  
  println!();
}
