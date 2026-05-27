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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut lists = lists;
        while lists.len() > 1 {
            let mut merged: Vec<Option<Box<ListNode>>> = Vec::with_capacity(lists.len().div_ceil(2));
            let mut iter = lists.into_iter();
            while let Some(a) = iter.next() {
                let b = iter.next().flatten();
                merged.push(merge_two(a, b));
            }
            lists = merged;
        }
        lists.into_iter().next().flatten()
    }
}

fn merge_two(
    mut a: Option<Box<ListNode>>,
    mut b: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut tail = &mut dummy;
    loop {
        let take_a = match (&a, &b) {
            (Some(x), Some(y)) => x.val <= y.val,
            (Some(_), None) => true,
            (None, Some(_)) => false,
            (None, None) => break,
        };
        let mut node = if take_a { a.take().unwrap() } else { b.take().unwrap() };
        if take_a {
            a = node.next.take();
        } else {
            b = node.next.take();
        }
        tail.next = Some(node);
        tail = tail.next.as_mut().unwrap();
    }
    dummy.next
}
