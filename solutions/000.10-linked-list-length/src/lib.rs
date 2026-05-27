#[derive(Debug, PartialEq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

pub struct Solution;

impl Solution {
    pub fn list_length(head: Option<Box<ListNode>>) -> i32 {
        let mut count = 0;
        let mut cur = head.as_deref();
        while let Some(node) = cur {
            count += 1;
            cur = node.next.as_deref();
        }
        count
    }
}
