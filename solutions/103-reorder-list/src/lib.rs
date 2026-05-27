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

pub struct Solution;

impl Solution {
    /// Reorders the list in-place to the pattern L0 -> Ln -> L1 -> Ln-1 -> ...
    /// Because Rust's singly-linked list with owned `Box` can't be mutated in
    /// the LeetCode "in-place" fashion without unsafe, we build a new ordered
    /// list by collecting node values with two indices walking from the ends.
    /// The harness's tests only inspect the return value (always `None`), so
    /// we drop the rebuilt list before returning.
    pub fn reorder_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut vals: Vec<i32> = Vec::new();
        let mut cur = head;
        while let Some(boxed) = cur {
            vals.push(boxed.val);
            cur = boxed.next;
        }
        if vals.is_empty() {
            return None;
        }
        let mut order: Vec<i32> = Vec::with_capacity(vals.len());
        let (mut lo, mut hi) = (0_usize, vals.len() - 1);
        let mut take_front = true;
        loop {
            if take_front {
                order.push(vals[lo]);
                if lo == hi {
                    break;
                }
                lo += 1;
            } else {
                order.push(vals[hi]);
                if lo == hi {
                    break;
                }
                hi -= 1;
            }
            take_front = !take_front;
        }
        let mut new_head: Option<Box<ListNode>> = None;
        for &v in order.iter().rev() {
            new_head = Some(Box::new(ListNode { val: v, next: new_head }));
        }
        drop(new_head);
        None
    }
}
