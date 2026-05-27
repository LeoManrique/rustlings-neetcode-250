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
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let k = k as usize;
        if k <= 1 {
            return head;
        }
        // Collect all values into a vector, then reverse in groups of k,
        // and rebuild the list. This keeps the borrow checker happy while
        // staying O(n) time and O(n) extra space.
        let mut vals: Vec<i32> = Vec::new();
        let mut cur = head.as_deref();
        while let Some(node) = cur {
            vals.push(node.val);
            cur = node.next.as_deref();
        }
        let n = vals.len();
        let full_groups = n / k;
        for g in 0..full_groups {
            let start = g * k;
            let end = start + k;
            let (mut i, mut j) = (start, end - 1);
            while i < j {
                vals.swap(i, j);
                i += 1;
                j -= 1;
            }
        }
        // Rebuild list.
        let mut head: Option<Box<ListNode>> = None;
        for &v in vals.iter().rev() {
            let mut node = Box::new(ListNode::new(v));
            node.next = head;
            head = Some(node);
        }
        head
    }
}
