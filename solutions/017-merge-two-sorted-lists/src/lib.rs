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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        let (mut a, mut b) = (list1, list2);
        while let (Some(na), Some(nb)) = (a.as_ref(), b.as_ref()) {
            if na.val <= nb.val {
                let mut node = a.take().unwrap();
                a = node.next.take();
                tail.next = Some(node);
            } else {
                let mut node = b.take().unwrap();
                b = node.next.take();
                tail.next = Some(node);
            }
            tail = tail.next.as_mut().unwrap();
        }
        tail.next = if a.is_some() { a } else { b };
        dummy.next
    }
}
