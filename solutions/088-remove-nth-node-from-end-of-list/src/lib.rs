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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut len = 0_i32;
        {
            let mut cur = &head;
            while let Some(node) = cur {
                len += 1;
                cur = &node.next;
            }
        }
        let target = len - n;
        if target < 0 {
            return head;
        }
        if target == 0 {
            return head.and_then(|b| b.next);
        }
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut cur = dummy.as_mut();
        for _ in 0..target {
            cur = cur.next.as_mut().unwrap();
        }
        let removed = cur.next.take();
        if let Some(removed_node) = removed {
            cur.next = removed_node.next;
        }
        dummy.next
    }
}
