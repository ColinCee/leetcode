// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = head;
        let mut prev = None;

        while let Some(mut node) = current {
            current = node.next.take();
            node.next = prev;
            prev = Some(node);
        }

        prev
    }
}

pub fn solve() {
    // Implement the solution here
    let mut one = Some(Box::new(ListNode::new(1)));
    let mut two = Some(Box::new(ListNode::new(2)));
    let three = Some(Box::new(ListNode::new(3)));

    if let Some(node) = two.as_mut() {
        node.next = three;
    }

    if let Some(node) = one.as_mut() {
        node.next = two;
    }

    let head = Solution::reverse_list(one);

    let mut current = &head;
    while let Some(node) = current {
        print!("{}", node.val);
        current = &node.next;
        if current.is_some() {
            print!(" -> ");
        }
    }
    println!();
}
