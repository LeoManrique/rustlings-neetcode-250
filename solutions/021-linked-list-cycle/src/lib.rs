pub struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

impl Solution {
    pub fn has_cycle(head: Option<Box<ListNode>>) -> bool {
        // With safe Box-owned lists, cycles cannot be represented.
        let _ = head;
        false
    }
}
