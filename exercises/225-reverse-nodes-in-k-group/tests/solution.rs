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
    let result = Solution::reverse_k_group(build_list(&[1, 2]), 2);
    assert_eq!(list_to_vec(&result), vec![2, 1]);
}

#[test]
fn test_2() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6]), 4);
    assert_eq!(list_to_vec(&result), vec![4, 3, 2, 1, 5, 6]);
}

#[test]
fn test_3() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7]), 3);
    assert_eq!(list_to_vec(&result), vec![3, 2, 1, 6, 5, 4, 7]);
}

#[test]
fn test_4() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 5);
    assert_eq!(list_to_vec(&result), vec![5, 4, 3, 2, 1, 10, 9, 8, 7, 6]);
}

#[test]
fn test_5() {
    let result = Solution::reverse_k_group(build_list(&[]), 1);
    assert_eq!(list_to_vec(&result), vec![]);
}

#[test]
fn test_6() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9]), 5);
    assert_eq!(list_to_vec(&result), vec![5, 4, 3, 2, 1, 6, 7, 8, 9]);
}

#[test]
fn test_7() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]), 4);
    assert_eq!(list_to_vec(&result), vec![4, 3, 2, 1, 8, 7, 6, 5, 9, 10, 11]);
}

#[test]
fn test_8() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5]), 2);
    assert_eq!(list_to_vec(&result), vec![2, 1, 4, 3, 5]);
}

#[test]
fn test_9() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6]), 2);
    assert_eq!(list_to_vec(&result), vec![2, 1, 4, 3, 6, 5]);
}

#[test]
fn test_10() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]), 2);
    assert_eq!(list_to_vec(&result), vec![2, 1, 4, 3, 6, 5, 8, 7, 10, 9, 11]);
}

#[test]
fn test_11() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6]), 3);
    assert_eq!(list_to_vec(&result), vec![3, 2, 1, 6, 5, 4]);
}

#[test]
fn test_12() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5]), 3);
    assert_eq!(list_to_vec(&result), vec![3, 2, 1, 4, 5]);
}

#[test]
fn test_13() {
    let result = Solution::reverse_k_group(build_list(&[1, 2]), 1);
    assert_eq!(list_to_vec(&result), vec![1, 2]);
}

#[test]
fn test_14() {
    let result = Solution::reverse_k_group(build_list(&[1]), 1);
    assert_eq!(list_to_vec(&result), vec![1]);
}

#[test]
fn test_15() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8]), 4);
    assert_eq!(list_to_vec(&result), vec![4, 3, 2, 1, 8, 7, 6, 5]);
}

#[test]
fn test_16() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3]), 1);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3]);
}

#[test]
fn test_17() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9]), 3);
    assert_eq!(list_to_vec(&result), vec![3, 2, 1, 6, 5, 4, 9, 8, 7]);
}

#[test]
fn test_18() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21]), 6);
    assert_eq!(list_to_vec(&result), vec![6, 5, 4, 3, 2, 1, 12, 11, 10, 9, 8, 7, 18, 17, 16, 15, 14, 13, 19, 20, 21]);
}

#[test]
fn test_19() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60]), 13);
    assert_eq!(list_to_vec(&result), vec![13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 39, 38, 37, 36, 35, 34, 33, 32, 31, 30, 29, 28, 27, 52, 51, 50, 49, 48, 47, 46, 45, 44, 43, 42, 41, 40, 53, 54, 55, 56, 57, 58, 59, 60]);
}

#[test]
fn test_20() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 4);
    assert_eq!(list_to_vec(&result), vec![4, 3, 2, 1, 8, 7, 6, 5, 9, 10]);
}

#[test]
fn test_21() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]), 7);
    assert_eq!(list_to_vec(&result), vec![7, 6, 5, 4, 3, 2, 1, 14, 13, 12, 11, 10, 9, 8]);
}

#[test]
fn test_22() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 6);
    assert_eq!(list_to_vec(&result), vec![6, 5, 4, 3, 2, 1, 12, 11, 10, 9, 8, 7, 18, 17, 16, 15, 14, 13, 19, 20]);
}

#[test]
fn test_23() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25]), 11);
    assert_eq!(list_to_vec(&result), vec![11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 23, 24, 25]);
}

#[test]
fn test_24() {
    let result = Solution::reverse_k_group(build_list(&[9, 1, 7, 3, 8, 4, 6, 2, 5, 0]), 3);
    assert_eq!(list_to_vec(&result), vec![7, 1, 9, 4, 8, 3, 5, 2, 6, 0]);
}

#[test]
fn test_25() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]), 12);
    assert_eq!(list_to_vec(&result), vec![12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 25, 26, 27, 28, 29, 30]);
}

#[test]
fn test_26() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]), 7);
    assert_eq!(list_to_vec(&result), vec![7, 6, 5, 4, 3, 2, 1, 14, 13, 12, 11, 10, 9, 8, 21, 20, 19, 18, 17, 16, 15, 28, 27, 26, 25, 24, 23, 22, 29, 30]);
}

#[test]
fn test_27() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]), 4);
    assert_eq!(list_to_vec(&result), vec![4, 3, 2, 1, 8, 7, 6, 5, 12, 11, 10, 9]);
}

#[test]
fn test_28() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 7);
    assert_eq!(list_to_vec(&result), vec![7, 6, 5, 4, 3, 2, 1, 14, 13, 12, 11, 10, 9, 8, 15, 16, 17, 18, 19, 20]);
}

#[test]
fn test_29() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]), 6);
    assert_eq!(list_to_vec(&result), vec![6, 5, 4, 3, 2, 1, 12, 11, 10, 9, 8, 7]);
}

#[test]
fn test_30() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]), 10);
    assert_eq!(list_to_vec(&result), vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21]);
}

#[test]
fn test_31() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3]), 3);
    assert_eq!(list_to_vec(&result), vec![3, 2, 1]);
}

#[test]
fn test_32() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22]), 11);
    assert_eq!(list_to_vec(&result), vec![11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12]);
}

#[test]
fn test_33() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50]), 50);
    assert_eq!(list_to_vec(&result), vec![50, 49, 48, 47, 46, 45, 44, 43, 42, 41, 40, 39, 38, 37, 36, 35, 34, 33, 32, 31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
}

#[test]
fn test_34() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 3);
    assert_eq!(list_to_vec(&result), vec![3, 2, 1, 6, 5, 4, 9, 8, 7, 12, 11, 10, 15, 14, 13, 18, 17, 16, 19, 20]);
}

#[test]
fn test_35() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27]), 20);
    assert_eq!(list_to_vec(&result), vec![20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 21, 22, 23, 24, 25, 26, 27]);
}

#[test]
fn test_36() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 13);
    assert_eq!(list_to_vec(&result), vec![13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 14, 15, 16, 17, 18, 19, 20]);
}

#[test]
fn test_37() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21]), 10);
    assert_eq!(list_to_vec(&result), vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 21]);
}

#[test]
fn test_38() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4]), 4);
    assert_eq!(list_to_vec(&result), vec![4, 3, 2, 1]);
}

#[test]
fn test_39() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23]), 16);
    assert_eq!(list_to_vec(&result), vec![16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 17, 18, 19, 20, 21, 22, 23]);
}

#[test]
fn test_40() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]), 5);
    assert_eq!(list_to_vec(&result), vec![5, 4, 3, 2, 1, 10, 9, 8, 7, 6, 11, 12]);
}

#[test]
fn test_41() {
    let result = Solution::reverse_k_group(build_list(&[42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42]), 6);
    assert_eq!(list_to_vec(&result), vec![42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42]);
}

#[test]
fn test_42() {
    let result = Solution::reverse_k_group(build_list(&[5, 1, 9, 7, 4, 6, 2, 3]), 2);
    assert_eq!(list_to_vec(&result), vec![1, 5, 7, 9, 6, 4, 3, 2]);
}

#[test]
fn test_43() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40]), 11);
    assert_eq!(list_to_vec(&result), vec![11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 33, 32, 31, 30, 29, 28, 27, 26, 25, 24, 23, 34, 35, 36, 37, 38, 39, 40]);
}

#[test]
fn test_44() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26]), 19);
    assert_eq!(list_to_vec(&result), vec![19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 20, 21, 22, 23, 24, 25, 26]);
}

#[test]
fn test_45() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 5);
    assert_eq!(list_to_vec(&result), vec![5, 4, 3, 2, 1, 10, 9, 8, 7, 6, 15, 14, 13, 12, 11]);
}

#[test]
fn test_46() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50]), 12);
    assert_eq!(list_to_vec(&result), vec![12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 36, 35, 34, 33, 32, 31, 30, 29, 28, 27, 26, 25, 48, 47, 46, 45, 44, 43, 42, 41, 40, 39, 38, 37, 49, 50]);
}

#[test]
fn test_47() {
    let result = Solution::reverse_k_group(build_list(&[9, 8, 7, 6, 5, 4, 3, 2, 1]), 9);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_48() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]), 7);
    assert_eq!(list_to_vec(&result), vec![7, 6, 5, 4, 3, 2, 1, 8, 9, 10, 11, 12, 13]);
}

#[test]
fn test_49() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]), 3);
    assert_eq!(list_to_vec(&result), vec![3, 2, 1, 6, 5, 4, 9, 8, 7, 10, 11]);
}

#[test]
fn test_50() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]), 5);
    assert_eq!(list_to_vec(&result), vec![5, 4, 3, 2, 1, 10, 9, 8, 7, 6, 11, 12, 13, 14]);
}

#[test]
fn test_51() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]), 4);
    assert_eq!(list_to_vec(&result), vec![4, 3, 2, 1, 8, 7, 6, 5, 12, 11, 10, 9, 13]);
}

#[test]
fn test_52() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21]), 14);
    assert_eq!(list_to_vec(&result), vec![14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 15, 16, 17, 18, 19, 20, 21]);
}

#[test]
fn test_53() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25]), 5);
    assert_eq!(list_to_vec(&result), vec![5, 4, 3, 2, 1, 10, 9, 8, 7, 6, 15, 14, 13, 12, 11, 20, 19, 18, 17, 16, 25, 24, 23, 22, 21]);
}

#[test]
fn test_54() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]), 11);
    assert_eq!(list_to_vec(&result), vec![11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
}

#[test]
fn test_55() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]), 6);
    assert_eq!(list_to_vec(&result), vec![6, 5, 4, 3, 2, 1, 12, 11, 10, 9, 8, 7, 13, 14, 15, 16]);
}

#[test]
fn test_56() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]), 11);
    assert_eq!(list_to_vec(&result), vec![11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 12, 13, 14, 15, 16, 17, 18]);
}

#[test]
fn test_57() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 8);
    assert_eq!(list_to_vec(&result), vec![8, 7, 6, 5, 4, 3, 2, 1, 16, 15, 14, 13, 12, 11, 10, 9, 17, 18, 19, 20]);
}

#[test]
fn test_58() {
    let result = Solution::reverse_k_group(build_list(&[1000, 999, 998, 997, 996, 995, 994, 993, 992, 991, 990, 989, 988, 987, 986, 985, 984, 983, 982, 981, 980, 979, 978, 977, 976, 975, 974, 973, 972, 971, 970, 969, 968, 967, 966, 965, 964, 963, 962, 961, 960]), 3);
    assert_eq!(list_to_vec(&result), vec![998, 999, 1000, 995, 996, 997, 992, 993, 994, 989, 990, 991, 986, 987, 988, 983, 984, 985, 980, 981, 982, 977, 978, 979, 974, 975, 976, 971, 972, 973, 968, 969, 970, 965, 966, 967, 962, 963, 964, 961, 960]);
}

#[test]
fn test_59() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75]), 15);
    assert_eq!(list_to_vec(&result), vec![15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 45, 44, 43, 42, 41, 40, 39, 38, 37, 36, 35, 34, 33, 32, 31, 60, 59, 58, 57, 56, 55, 54, 53, 52, 51, 50, 49, 48, 47, 46, 75, 74, 73, 72, 71, 70, 69, 68, 67, 66, 65, 64, 63, 62, 61]);
}

#[test]
fn test_60() {
    let result = Solution::reverse_k_group(build_list(&[100, 200, 300, 400, 500, 600, 700, 800]), 5);
    assert_eq!(list_to_vec(&result), vec![500, 400, 300, 200, 100, 600, 700, 800]);
}

#[test]
fn test_61() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]), 1);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]);
}

#[test]
fn test_62() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]), 5);
    assert_eq!(list_to_vec(&result), vec![5, 4, 3, 2, 1, 10, 9, 8, 7, 6, 11, 12, 13]);
}

#[test]
fn test_63() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28]), 21);
    assert_eq!(list_to_vec(&result), vec![21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 22, 23, 24, 25, 26, 27, 28]);
}

#[test]
fn test_64() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25]), 18);
    assert_eq!(list_to_vec(&result), vec![18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 19, 20, 21, 22, 23, 24, 25]);
}

#[test]
fn test_65() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 6);
    assert_eq!(list_to_vec(&result), vec![6, 5, 4, 3, 2, 1, 12, 11, 10, 9, 8, 7, 13, 14, 15]);
}

#[test]
fn test_66() {
    let result = Solution::reverse_k_group(build_list(&[7, 8, 9, 10, 11, 12, 13, 14, 15]), 4);
    assert_eq!(list_to_vec(&result), vec![10, 9, 8, 7, 14, 13, 12, 11, 15]);
}

#[test]
fn test_67() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17]), 10);
    assert_eq!(list_to_vec(&result), vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 11, 12, 13, 14, 15, 16, 17]);
}

#[test]
fn test_68() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]), 7);
    assert_eq!(list_to_vec(&result), vec![7, 6, 5, 4, 3, 2, 1, 14, 13, 12, 11, 10, 9, 8, 15, 16]);
}

#[test]
fn test_69() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]), 8);
    assert_eq!(list_to_vec(&result), vec![8, 7, 6, 5, 4, 3, 2, 1, 16, 15, 14, 13, 12, 11, 10, 9]);
}

#[test]
fn test_70() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7]), 1);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn test_71() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31]), 10);
    assert_eq!(list_to_vec(&result), vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 31]);
}

#[test]
fn test_72() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29]), 22);
    assert_eq!(list_to_vec(&result), vec![22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 23, 24, 25, 26, 27, 28, 29]);
}

#[test]
fn test_73() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25]), 9);
    assert_eq!(list_to_vec(&result), vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 18, 17, 16, 15, 14, 13, 12, 11, 10, 19, 20, 21, 22, 23, 24, 25]);
}

#[test]
fn test_74() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]), 6);
    assert_eq!(list_to_vec(&result), vec![6, 5, 4, 3, 2, 1, 12, 11, 10, 9, 8, 7, 13]);
}

#[test]
fn test_75() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25]), 8);
    assert_eq!(list_to_vec(&result), vec![8, 7, 6, 5, 4, 3, 2, 1, 16, 15, 14, 13, 12, 11, 10, 9, 24, 23, 22, 21, 20, 19, 18, 17, 25]);
}

#[test]
fn test_76() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36]), 14);
    assert_eq!(list_to_vec(&result), vec![14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 29, 30, 31, 32, 33, 34, 35, 36]);
}

#[test]
fn test_77() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 19);
    assert_eq!(list_to_vec(&result), vec![19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 20]);
}

#[test]
fn test_78() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8]), 2);
    assert_eq!(list_to_vec(&result), vec![2, 1, 4, 3, 6, 5, 8, 7]);
}

#[test]
fn test_79() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]), 9);
    assert_eq!(list_to_vec(&result), vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 18, 17, 16, 15, 14, 13, 12, 11, 10, 27, 26, 25, 24, 23, 22, 21, 20, 19, 28, 29, 30]);
}

#[test]
fn test_80() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21]), 7);
    assert_eq!(list_to_vec(&result), vec![7, 6, 5, 4, 3, 2, 1, 14, 13, 12, 11, 10, 9, 8, 21, 20, 19, 18, 17, 16, 15]);
}

#[test]
fn test_81() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]), 3);
    assert_eq!(list_to_vec(&result), vec![3, 2, 1, 6, 5, 4, 9, 8, 7, 12, 11, 10, 15, 14, 13, 16]);
}

#[test]
fn test_82() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31]), 13);
    assert_eq!(list_to_vec(&result), vec![13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 27, 28, 29, 30, 31]);
}

#[test]
fn test_83() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]), 9);
    assert_eq!(list_to_vec(&result), vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 10, 11, 12, 13, 14, 15, 16]);
}

#[test]
fn test_84() {
    let result = Solution::reverse_k_group(build_list(&[5, 4, 3, 2, 1, 0, -1, -2, -3, -4, -5]), 5);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, -4, -3, -2, -1, 0, -5]);
}

#[test]
fn test_85() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]), 2);
    assert_eq!(list_to_vec(&result), vec![2, 1, 4, 3, 6, 5, 8, 7, 10, 9, 12, 11, 14, 13, 16, 15, 18, 17, 19]);
}

#[test]
fn test_86() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17]), 6);
    assert_eq!(list_to_vec(&result), vec![6, 5, 4, 3, 2, 1, 12, 11, 10, 9, 8, 7, 13, 14, 15, 16, 17]);
}

#[test]
fn test_87() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 15);
    assert_eq!(list_to_vec(&result), vec![15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
}

#[test]
fn test_88() {
    let result = Solution::reverse_k_group(build_list(&[5, 4, 3, 2, 1, 0, -1, -2, -3, -4, -5, -6, -7, -8, -9, -10, -11]), 7);
    assert_eq!(list_to_vec(&result), vec![-1, 0, 1, 2, 3, 4, 5, -8, -7, -6, -5, -4, -3, -2, -9, -10, -11]);
}

#[test]
fn test_89() {
    let result = Solution::reverse_k_group(build_list(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 5);
    assert_eq!(list_to_vec(&result), vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_90() {
    let result = Solution::reverse_k_group(build_list(&[10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130]), 3);
    assert_eq!(list_to_vec(&result), vec![30, 20, 10, 60, 50, 40, 90, 80, 70, 120, 110, 100, 130]);
}

#[test]
fn test_91() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50]), 10);
    assert_eq!(list_to_vec(&result), vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 40, 39, 38, 37, 36, 35, 34, 33, 32, 31, 50, 49, 48, 47, 46, 45, 44, 43, 42, 41]);
}

#[test]
fn test_92() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]), 5);
    assert_eq!(list_to_vec(&result), vec![5, 4, 3, 2, 1, 10, 9, 8, 7, 6, 11]);
}

#[test]
fn test_93() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 1);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
}

#[test]
fn test_94() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50]), 1);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50]);
}

#[test]
fn test_95() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]), 9);
    assert_eq!(list_to_vec(&result), vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 18, 17, 16, 15, 14, 13, 12, 11, 10]);
}

#[test]
fn test_96() {
    let result = Solution::reverse_k_group(build_list(&[10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110]), 5);
    assert_eq!(list_to_vec(&result), vec![50, 40, 30, 20, 10, 100, 90, 80, 70, 60, 110]);
}

#[test]
fn test_97() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17]), 7);
    assert_eq!(list_to_vec(&result), vec![7, 6, 5, 4, 3, 2, 1, 14, 13, 12, 11, 10, 9, 8, 15, 16, 17]);
}

#[test]
fn test_98() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40]), 9);
    assert_eq!(list_to_vec(&result), vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 18, 17, 16, 15, 14, 13, 12, 11, 10, 27, 26, 25, 24, 23, 22, 21, 20, 19, 36, 35, 34, 33, 32, 31, 30, 29, 28, 37, 38, 39, 40]);
}

#[test]
fn test_99() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25]), 13);
    assert_eq!(list_to_vec(&result), vec![13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25]);
}

#[test]
fn test_100() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31]), 5);
    assert_eq!(list_to_vec(&result), vec![5, 4, 3, 2, 1, 10, 9, 8, 7, 6, 15, 14, 13, 12, 11, 20, 19, 18, 17, 16, 25, 24, 23, 22, 21, 30, 29, 28, 27, 26, 31]);
}

#[test]
fn test_101() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 2);
    assert_eq!(list_to_vec(&result), vec![2, 1, 4, 3, 6, 5, 8, 7, 10, 9, 12, 11, 14, 13, 16, 15, 18, 17, 20, 19]);
}

#[test]
fn test_102() {
    let result = Solution::reverse_k_group(build_list(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 2);
    assert_eq!(list_to_vec(&result), vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_103() {
    let result = Solution::reverse_k_group(build_list(&[1000, 999, 998, 997, 996, 995, 994, 993, 992, 991, 990, 989, 988, 987, 986, 985, 984, 983, 982, 981, 980, 979, 978, 977, 976, 975, 974, 973, 972, 971, 970, 969, 968, 967, 966, 965, 964, 963, 962, 961, 960]), 20);
    assert_eq!(list_to_vec(&result), vec![981, 982, 983, 984, 985, 986, 987, 988, 989, 990, 991, 992, 993, 994, 995, 996, 997, 998, 999, 1000, 961, 962, 963, 964, 965, 966, 967, 968, 969, 970, 971, 972, 973, 974, 975, 976, 977, 978, 979, 980, 960]);
}

#[test]
fn test_104() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]), 12);
    assert_eq!(list_to_vec(&result), vec![12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 13, 14, 15, 16, 17, 18, 19]);
}

#[test]
fn test_105() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22]), 15);
    assert_eq!(list_to_vec(&result), vec![15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 16, 17, 18, 19, 20, 21, 22]);
}

#[test]
fn test_106() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 8);
    assert_eq!(list_to_vec(&result), vec![8, 7, 6, 5, 4, 3, 2, 1, 9, 10, 11, 12, 13, 14, 15]);
}

#[test]
fn test_107() {
    let result = Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24]), 17);
    assert_eq!(list_to_vec(&result), vec![17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 18, 19, 20, 21, 22, 23, 24]);
}

#[test]
fn test_108() {
    let result = Solution::reverse_k_group(build_list(&[99, 98, 97, 96, 95, 94, 93, 92, 91, 90, 89, 88, 87, 86, 85, 84, 83, 82, 81, 80]), 15);
    assert_eq!(list_to_vec(&result), vec![85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 84, 83, 82, 81, 80]);
}
