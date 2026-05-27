#[derive(Debug, PartialEq)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

pub struct Solution;

impl Solution {
    pub fn list_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut sum = 0;
        let mut cur = head.as_deref();
        while let Some(node) = cur {
            sum += node.val;
            cur = node.next.as_deref();
        }
        sum
    }
}
