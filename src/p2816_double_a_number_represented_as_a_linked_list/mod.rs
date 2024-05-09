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
    pub fn reverse_list(head: &Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = head.as_ref();
        let mut prev = None;

        while let Some(node) = current {
            let next = node.next.as_ref();
            let mut new_node = Box::new(ListNode::new(node.val));
            new_node.next = prev;
            prev = Some(new_node);
            current = next;
        }

        prev
    }

    pub fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut reversed = Self::reverse_list(&head);

        let mut carry = 0;
        let mut current = &mut reversed;
        while let Some(node) = current {
            let new_val = (node.val * 2) + carry;
            node.val = new_val % 10;
            carry = new_val / 10;
            current = &mut node.next;
        }

        let original = Self::reverse_list(&reversed);
        if carry > 0 {
            let mut carry_node = ListNode::new(carry);
            carry_node.next = original;
            return Some(Box::new(carry_node));
        }
        original
    }
}

pub fn solve() {
    // Implement the solution here
    let mut one = Some(Box::new(ListNode::new(0)));
    let mut two = Some(Box::new(ListNode::new(9)));
    let three = Some(Box::new(ListNode::new(1)));

    if let Some(node) = two.as_mut() {
        node.next = three;
    }

    if let Some(node) = one.as_mut() {
        node.next = two;
    }

    let head = Solution::double_it(one);

    let mut current = &head;
    while let Some(node) = current {
        print!("{}", node.val);
        current = &node.next;
        if current.is_some() {
            print!(" -> ");
        }
    }
    println!();
    // Implement the solution here
}
