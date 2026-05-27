pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

impl Solution {
    pub fn insert_greatest_common_divisors(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut cur = head.as_mut();
        while let Some(node) = cur {
            if let Some(next) = node.next.take() {
                let g = gcd(node.val, next.val);
                let mut new_node = Box::new(ListNode::new(g));
                new_node.next = Some(next);
                node.next = Some(new_node);
                // Advance two steps so we land on the original `next` node.
                cur = node.next.as_mut().and_then(|n| n.next.as_mut());
            } else {
                break;
            }
        }
        head
    }
}
