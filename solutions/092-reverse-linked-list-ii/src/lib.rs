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
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        if left == right {
            return head;
        }
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        // Walk to the node just before `left`.
        let mut prev: &mut Box<ListNode> = &mut dummy;
        for _ in 1..left {
            prev = prev.next.as_mut().unwrap();
        }
        // Detach the sublist starting at `prev.next` (the `left`-th node).
        let mut curr = prev.next.take();
        // Reverse the first `right - left + 1` nodes; the rest stays attached
        // to the very first node we took, which becomes the sublist tail.
        let mut reversed: Option<Box<ListNode>> = None;
        for _ in 0..=(right - left) {
            let mut node = curr.unwrap();
            curr = node.next.take();
            node.next = reversed;
            reversed = Some(node);
        }
        // `reversed` is the new sublist head; its tail is the original `left`-th node.
        // Walk to that tail and attach `curr` (everything after the original `right`).
        let mut tail: &mut Box<ListNode> = reversed.as_mut().unwrap();
        while tail.next.is_some() {
            tail = tail.next.as_mut().unwrap();
        }
        tail.next = curr;
        prev.next = reversed;
        dummy.next
    }
}
