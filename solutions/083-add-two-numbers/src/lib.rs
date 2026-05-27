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

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        let mut tail = &mut head;
        let mut a = l1;
        let mut b = l2;
        let mut carry = 0;
        while a.is_some() || b.is_some() || carry > 0 {
            let av = a.as_ref().map_or(0, |n| n.val);
            let bv = b.as_ref().map_or(0, |n| n.val);
            let sum = av + bv + carry;
            carry = sum / 10;
            *tail = Some(Box::new(ListNode::new(sum % 10)));
            tail = &mut tail.as_mut().unwrap().next;
            a = a.and_then(|n| n.next);
            b = b.and_then(|n| n.next);
        }
        head
    }
}
