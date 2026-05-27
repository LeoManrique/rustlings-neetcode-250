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
    assert_eq!(Solution::has_cycle(build_list(&[])), false);
}

#[test]
fn test_2() {
    assert_eq!(Solution::has_cycle(build_list(&[])), false);
}

#[test]
fn test_3() {
    assert_eq!(Solution::has_cycle(build_list(&[])), false);
}

#[test]
fn test_4() {
    assert_eq!(Solution::has_cycle(build_list(&[])), false);
}

#[test]
fn test_5() {
    assert_eq!(Solution::has_cycle(build_list(&[])), false);
}

#[test]
fn test_6() {
    assert_eq!(Solution::has_cycle(build_list(&[])), false);
}

#[test]
fn test_7() {
    assert_eq!(Solution::has_cycle(build_list(&[])), false);
}

#[test]
fn test_8() {
    assert_eq!(Solution::has_cycle(build_list(&[])), false);
}

#[test]
fn test_9() {
    assert_eq!(Solution::has_cycle(build_list(&[])), false);
}

#[test]
fn test_10() {
    assert_eq!(Solution::has_cycle(build_list(&[])), false);
}
