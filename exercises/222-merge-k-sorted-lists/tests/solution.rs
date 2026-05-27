include!("../src/lib.rs");

#[allow(dead_code)]
fn build_list(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &val in vals.iter().rev() {
        let mut node = ListNode::new(val);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

#[allow(dead_code)]
fn build_lists(lists: &[Vec<i32>]) -> Vec<Option<Box<ListNode>>> {
    lists.iter().map(|v| build_list(v)).collect()
}

#[allow(dead_code)]
fn list_to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = vec![];
    let mut current = head;
    while let Some(node) = current {
        result.push(node.val);
        current = &node.next;
    }
    result
}

#[test]
fn test_1() {
    let result = Solution::merge_k_lists(vec![]);
    assert_eq!(list_to_vec(&result), vec![]);
}

#[test]
fn test_2() {
    let result = Solution::merge_k_lists(build_lists(&[vec![], vec![], vec![], vec![]]));
    assert_eq!(list_to_vec(&result), vec![]);
}

#[test]
fn test_3() {
    let result = Solution::merge_k_lists(build_lists(&[vec![]]));
    assert_eq!(list_to_vec(&result), vec![]);
}
