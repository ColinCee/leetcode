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

pub fn build_tree(nodes: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    let mut iter = nodes.iter();
    let root_val = iter.next()?;
    let root = Rc::new(RefCell::new(TreeNode {
        val: root_val.unwrap(),
        left: None,
        right: None,
    }));

    let mut queue = VecDeque::new();
    queue.push_back(Rc::clone(&root));

    while let Some(node) = queue.pop_front() {
        let left_val = iter.next();
        if let Some(Some(val)) = left_val {
            let left_node = Rc::new(RefCell::new(TreeNode {
                val: *val,
                left: None,
                right: None,
            }));
            node.borrow_mut().left = Some(Rc::clone(&left_node));
            queue.push_back(left_node);
        }

        let right_val = iter.next();
        if let Some(Some(val)) = right_val {
            let right_node = Rc::new(RefCell::new(TreeNode {
                val: *val,
                left: None,
                right: None,
            }));
            node.borrow_mut().right = Some(Rc::clone(&right_node));
            queue.push_back(right_node);
        }
    }

    Some(root)
}

pub fn print_tree_level_order(root: &Option<Rc<RefCell<TreeNode>>>) {
    if let Some(root_node) = root {
        let mut queue = VecDeque::new();
        queue.push_back(Some(root_node.clone()));

        while let Some(current_node_opt) = queue.pop_front() {
            match current_node_opt {
                Some(current_node) => {
                    let current_node_ref = current_node.borrow();
                    print!("{} ", current_node_ref.val);
                    queue.push_back(current_node_ref.left.clone());
                    queue.push_back(current_node_ref.right.clone());
                }
                None => print!("null "),
            }
        }
        println!();
    }
}
