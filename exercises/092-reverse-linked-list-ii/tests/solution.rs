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
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5]), 3, 4);
    assert_eq!(list_to_vec(&result), vec![1, 2, 4, 3, 5]);
}

#[test]
fn test_2() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5]), 2, 4);
    assert_eq!(list_to_vec(&result), vec![1, 4, 3, 2, 5]);
}

#[test]
fn test_3() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 3, 8);
    assert_eq!(list_to_vec(&result), vec![1, 2, 8, 7, 6, 5, 4, 3, 9, 10]);
}

#[test]
fn test_4() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5]), 2, 2);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_5() {
    let result = Solution::reverse_between(build_list(&[10, 20, 30, 40, 50]), 1, 3);
    assert_eq!(list_to_vec(&result), vec![30, 20, 10, 40, 50]);
}

#[test]
fn test_6() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3]), 1, 3);
    assert_eq!(list_to_vec(&result), vec![3, 2, 1]);
}

#[test]
fn test_7() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9]), 3, 7);
    assert_eq!(list_to_vec(&result), vec![1, 2, 7, 6, 5, 4, 3, 8, 9]);
}

#[test]
fn test_8() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5]), 2, 3);
    assert_eq!(list_to_vec(&result), vec![1, 3, 2, 4, 5]);
}

#[test]
fn test_9() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5]), 5, 5);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_10() {
    let result = Solution::reverse_between(build_list(&[1, 3, 5, 7, 9]), 2, 3);
    assert_eq!(list_to_vec(&result), vec![1, 5, 3, 7, 9]);
}

#[test]
fn test_11() {
    let result = Solution::reverse_between(build_list(&[5]), 1, 1);
    assert_eq!(list_to_vec(&result), vec![5]);
}

#[test]
fn test_12() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5]), 1, 1);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_13() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7]), 3, 5);
    assert_eq!(list_to_vec(&result), vec![1, 2, 5, 4, 3, 6, 7]);
}

#[test]
fn test_14() {
    let result = Solution::reverse_between(build_list(&[1, 3, 5, 7, 9]), 2, 4);
    assert_eq!(list_to_vec(&result), vec![1, 7, 5, 3, 9]);
}

#[test]
fn test_15() {
    let result = Solution::reverse_between(build_list(&[-1, 0, 1, 2, 3]), 2, 5);
    assert_eq!(list_to_vec(&result), vec![-1, 3, 2, 1, 0]);
}

#[test]
fn test_16() {
    let result = Solution::reverse_between(build_list(&[10, 20, 30, 40, 50]), 1, 5);
    assert_eq!(list_to_vec(&result), vec![50, 40, 30, 20, 10]);
}

#[test]
fn test_17() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5]), 1, 5);
    assert_eq!(list_to_vec(&result), vec![5, 4, 3, 2, 1]);
}

#[test]
fn test_18() {
    let result = Solution::reverse_between(build_list(&[-1, -2, -3, -4, -5, -6, -7, -8, -9, -10]), 3, 8);
    assert_eq!(list_to_vec(&result), vec![-1, -2, -8, -7, -6, -5, -4, -3, -9, -10]);
}

#[test]
fn test_19() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]), 7, 7);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
}

#[test]
fn test_20() {
    let result = Solution::reverse_between(build_list(&[-1, -2, -3, -4, -5, -6, -7, -8, -9, -10]), 4, 7);
    assert_eq!(list_to_vec(&result), vec![-1, -2, -3, -7, -6, -5, -4, -8, -9, -10]);
}

#[test]
fn test_21() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 8, 8);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
}

#[test]
fn test_22() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 5, 16);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 17, 18, 19, 20]);
}

#[test]
fn test_23() {
    let result = Solution::reverse_between(build_list(&[500, -500, 500, -500, 500]), 2, 4);
    assert_eq!(list_to_vec(&result), vec![500, -500, 500, -500, 500]);
}

#[test]
fn test_24() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 6, 15);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 16, 17, 18, 19, 20]);
}

#[test]
fn test_25() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]), 8, 10);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 10, 9, 8, 11, 12]);
}

#[test]
fn test_26() {
    let result = Solution::reverse_between(build_list(&[10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150]), 5, 15);
    assert_eq!(list_to_vec(&result), vec![10, 20, 30, 40, 150, 140, 130, 120, 110, 100, 90, 80, 70, 60, 50]);
}

#[test]
fn test_27() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 5, 12);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 12, 11, 10, 9, 8, 7, 6, 5, 13, 14, 15]);
}

#[test]
fn test_28() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]), 1, 12);
    assert_eq!(list_to_vec(&result), vec![12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
}

#[test]
fn test_29() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 4, 8);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 8, 7, 6, 5, 4, 9, 10]);
}

#[test]
fn test_30() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 6, 11);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 11, 10, 9, 8, 7, 6, 12, 13, 14, 15]);
}

#[test]
fn test_31() {
    let result = Solution::reverse_between(build_list(&[500, 400, 300, 200, 100]), 2, 4);
    assert_eq!(list_to_vec(&result), vec![500, 200, 300, 400, 100]);
}

#[test]
fn test_32() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 7, 7);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
}

#[test]
fn test_33() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 13, 15);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 15, 14, 13]);
}

#[test]
fn test_34() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 1, 1);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
}

#[test]
fn test_35() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 5, 5);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_36() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 11, 15);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 15, 14, 13, 12, 11]);
}

#[test]
fn test_37() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]), 2, 5);
    assert_eq!(list_to_vec(&result), vec![1, 5, 4, 3, 2, 6, 7, 8, 9, 10, 11, 12]);
}

#[test]
fn test_38() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 3, 3);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_39() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 4, 10);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 10, 9, 8, 7, 6, 5, 4, 11, 12, 13, 14, 15]);
}

#[test]
fn test_40() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]), 5, 11);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 11, 10, 9, 8, 7, 6, 5, 12]);
}

#[test]
fn test_41() {
    let result = Solution::reverse_between(build_list(&[100, -200, 300, -400, 500]), 2, 4);
    assert_eq!(list_to_vec(&result), vec![100, -400, 300, -200, 500]);
}

#[test]
fn test_42() {
    let result = Solution::reverse_between(build_list(&[5, 4, 3, 2, 1, 0, -1, -2, -3, -4, -5]), 3, 9);
    assert_eq!(list_to_vec(&result), vec![5, 4, -3, -2, -1, 0, 1, 2, 3, -4, -5]);
}

#[test]
fn test_43() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 6, 6);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_44() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 2, 9);
    assert_eq!(list_to_vec(&result), vec![1, 9, 8, 7, 6, 5, 4, 3, 2, 10]);
}

#[test]
fn test_45() {
    let result = Solution::reverse_between(build_list(&[-1, -2, -3, -4, -5, -6, -7, -8, -9, -10, -11, -12, -13, -14, -15]), 4, 10);
    assert_eq!(list_to_vec(&result), vec![-1, -2, -3, -10, -9, -8, -7, -6, -5, -4, -11, -12, -13, -14, -15]);
}

#[test]
fn test_46() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 6, 10);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 10, 9, 8, 7, 6]);
}

#[test]
fn test_47() {
    let result = Solution::reverse_between(build_list(&[-10, -20, -30, -40, -50]), 1, 3);
    assert_eq!(list_to_vec(&result), vec![-30, -20, -10, -40, -50]);
}

#[test]
fn test_48() {
    let result = Solution::reverse_between(build_list(&[5, 4, 3, 2, 1]), 2, 5);
    assert_eq!(list_to_vec(&result), vec![5, 1, 2, 3, 4]);
}

#[test]
fn test_49() {
    let result = Solution::reverse_between(build_list(&[10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 2, 9);
    assert_eq!(list_to_vec(&result), vec![10, 2, 3, 4, 5, 6, 7, 8, 9, 1]);
}

#[test]
fn test_50() {
    let result = Solution::reverse_between(build_list(&[10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 4, 7);
    assert_eq!(list_to_vec(&result), vec![10, 9, 8, 4, 5, 6, 7, 3, 2, 1]);
}

#[test]
fn test_51() {
    let result = Solution::reverse_between(build_list(&[1, 3, 5, 7, 9, 11, 13, 15, 17, 19]), 1, 2);
    assert_eq!(list_to_vec(&result), vec![3, 1, 5, 7, 9, 11, 13, 15, 17, 19]);
}

#[test]
fn test_52() {
    let result = Solution::reverse_between(build_list(&[100, 200, 300, 400, 500, 600, 700, 800]), 2, 7);
    assert_eq!(list_to_vec(&result), vec![100, 700, 600, 500, 400, 300, 200, 800]);
}

#[test]
fn test_53() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 5, 9);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 9, 8, 7, 6, 5, 10]);
}

#[test]
fn test_54() {
    let result = Solution::reverse_between(build_list(&[-100, -200, -300, -400, -500]), 1, 4);
    assert_eq!(list_to_vec(&result), vec![-400, -300, -200, -100, -500]);
}

#[test]
fn test_55() {
    let result = Solution::reverse_between(build_list(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 5, 15);
    assert_eq!(list_to_vec(&result), vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_56() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 1, 7);
    assert_eq!(list_to_vec(&result), vec![7, 6, 5, 4, 3, 2, 1, 8, 9, 10, 11, 12, 13, 14, 15]);
}

#[test]
fn test_57() {
    let result = Solution::reverse_between(build_list(&[-1, -2, -3, -4, -5, -6, -7, -8, -9, -10]), 4, 8);
    assert_eq!(list_to_vec(&result), vec![-1, -2, -3, -8, -7, -6, -5, -4, -9, -10]);
}

#[test]
fn test_58() {
    let result = Solution::reverse_between(build_list(&[10, 20, 30, 40, 50, 60, 70, 80, 90]), 3, 7);
    assert_eq!(list_to_vec(&result), vec![10, 20, 70, 60, 50, 40, 30, 80, 90]);
}

#[test]
fn test_59() {
    let result = Solution::reverse_between(build_list(&[-10, -9, -8, -7, -6, -5, -4, -3, -2, -1]), 3, 7);
    assert_eq!(list_to_vec(&result), vec![-10, -9, -4, -5, -6, -7, -8, -3, -2, -1]);
}

#[test]
fn test_60() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 9, 10);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 10, 9]);
}

#[test]
fn test_61() {
    let result = Solution::reverse_between(build_list(&[2, 4, 6, 8, 10, 12, 14, 16, 18, 20]), 5, 5);
    assert_eq!(list_to_vec(&result), vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20]);
}

#[test]
fn test_62() {
    let result = Solution::reverse_between(build_list(&[7, 6, 5, 4, 3, 2, 1]), 1, 1);
    assert_eq!(list_to_vec(&result), vec![7, 6, 5, 4, 3, 2, 1]);
}

#[test]
fn test_63() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 1, 1);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_64() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 1, 15);
    assert_eq!(list_to_vec(&result), vec![15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
}

#[test]
fn test_65() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 10, 20);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10]);
}

#[test]
fn test_66() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 1, 5);
    assert_eq!(list_to_vec(&result), vec![5, 4, 3, 2, 1, 6, 7, 8, 9, 10]);
}

#[test]
fn test_67() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 4, 12);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 12, 11, 10, 9, 8, 7, 6, 5, 4, 13, 14, 15]);
}

#[test]
fn test_68() {
    let result = Solution::reverse_between(build_list(&[10, 20, 30, 40, 50, 60, 70, 80, 90, 100]), 3, 8);
    assert_eq!(list_to_vec(&result), vec![10, 20, 80, 70, 60, 50, 40, 30, 90, 100]);
}

#[test]
fn test_69() {
    let result = Solution::reverse_between(build_list(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]), 1, 5);
    assert_eq!(list_to_vec(&result), vec![4, 3, 2, 1, 0, 5, 6, 7, 8, 9]);
}

#[test]
fn test_70() {
    let result = Solution::reverse_between(build_list(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 3, 7);
    assert_eq!(list_to_vec(&result), vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_71() {
    let result = Solution::reverse_between(build_list(&[-10, -20, -30, -40, -50, -60, -70]), 2, 5);
    assert_eq!(list_to_vec(&result), vec![-10, -50, -40, -30, -20, -60, -70]);
}

#[test]
fn test_72() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 1, 2);
    assert_eq!(list_to_vec(&result), vec![2, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
}

#[test]
fn test_73() {
    let result = Solution::reverse_between(build_list(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 5, 9);
    assert_eq!(list_to_vec(&result), vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_74() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 7, 11);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 11, 10, 9, 8, 7, 12, 13, 14, 15]);
}

#[test]
fn test_75() {
    let result = Solution::reverse_between(build_list(&[-1, -2, -3, -4, -5, -6, -7, -8, -9, -10]), 3, 7);
    assert_eq!(list_to_vec(&result), vec![-1, -2, -7, -6, -5, -4, -3, -8, -9, -10]);
}

#[test]
fn test_76() {
    let result = Solution::reverse_between(build_list(&[100, 200, 300, 400, 500, 600, 700, 800, 900, 1000]), 1, 5);
    assert_eq!(list_to_vec(&result), vec![500, 400, 300, 200, 100, 600, 700, 800, 900, 1000]);
}

#[test]
fn test_77() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 2, 14);
    assert_eq!(list_to_vec(&result), vec![1, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 15]);
}

#[test]
fn test_78() {
    let result = Solution::reverse_between(build_list(&[100, 200, 300, 400, 500, 600]), 1, 5);
    assert_eq!(list_to_vec(&result), vec![500, 400, 300, 200, 100, 600]);
}

#[test]
fn test_79() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 5, 8);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 8, 7, 6, 5, 9, 10]);
}

#[test]
fn test_80() {
    let result = Solution::reverse_between(build_list(&[15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 2, 14);
    assert_eq!(list_to_vec(&result), vec![15, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 1]);
}

#[test]
fn test_81() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 10, 14);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 14, 13, 12, 11, 10, 15]);
}

#[test]
fn test_82() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 6, 12);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 12, 11, 10, 9, 8, 7, 6, 13, 14, 15]);
}

#[test]
fn test_83() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 1, 10);
    assert_eq!(list_to_vec(&result), vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
}

#[test]
fn test_84() {
    let result = Solution::reverse_between(build_list(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 10, 30);
    assert_eq!(list_to_vec(&result), vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_85() {
    let result = Solution::reverse_between(build_list(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 3, 8);
    assert_eq!(list_to_vec(&result), vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_86() {
    let result = Solution::reverse_between(build_list(&[-10, -20, -30, -40, -50]), 3, 5);
    assert_eq!(list_to_vec(&result), vec![-10, -20, -50, -40, -30]);
}

#[test]
fn test_87() {
    let result = Solution::reverse_between(build_list(&[50, 40, 30, 20, 10]), 2, 4);
    assert_eq!(list_to_vec(&result), vec![50, 20, 30, 40, 10]);
}

#[test]
fn test_88() {
    let result = Solution::reverse_between(build_list(&[5, 10, 15, 20, 25, 30, 35, 40]), 3, 5);
    assert_eq!(list_to_vec(&result), vec![5, 10, 25, 20, 15, 30, 35, 40]);
}

#[test]
fn test_89() {
    let result = Solution::reverse_between(build_list(&[9, 8, 7, 6, 5, 4, 3, 2, 1]), 2, 8);
    assert_eq!(list_to_vec(&result), vec![9, 2, 3, 4, 5, 6, 7, 8, 1]);
}

#[test]
fn test_90() {
    let result = Solution::reverse_between(build_list(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 2, 9);
    assert_eq!(list_to_vec(&result), vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_91() {
    let result = Solution::reverse_between(build_list(&[500, -500, 250, -250, 0, 1, 2, 3, 4, 5]), 2, 6);
    assert_eq!(list_to_vec(&result), vec![500, 1, 0, -250, 250, -500, 2, 3, 4, 5]);
}

#[test]
fn test_92() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 5, 9);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 9, 8, 7, 6, 5, 10, 11, 12, 13, 14, 15]);
}

#[test]
fn test_93() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 9, 15);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 15, 14, 13, 12, 11, 10, 9]);
}

#[test]
fn test_94() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 4, 7);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 7, 6, 5, 4, 8, 9, 10]);
}

#[test]
fn test_95() {
    let result = Solution::reverse_between(build_list(&[10, 20, 30, 40, 50, 60, 70, 80, 90, 100]), 1, 10);
    assert_eq!(list_to_vec(&result), vec![100, 90, 80, 70, 60, 50, 40, 30, 20, 10]);
}

#[test]
fn test_96() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]), 4, 10);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 10, 9, 8, 7, 6, 5, 4, 11, 12]);
}

#[test]
fn test_97() {
    let result = Solution::reverse_between(build_list(&[10, 20, 30, 40, 50, 60, 70]), 3, 6);
    assert_eq!(list_to_vec(&result), vec![10, 20, 60, 50, 40, 30, 70]);
}

#[test]
fn test_98() {
    let result = Solution::reverse_between(build_list(&[-500, 500, -500, 500, -500]), 1, 5);
    assert_eq!(list_to_vec(&result), vec![-500, 500, -500, 500, -500]);
}

#[test]
fn test_99() {
    let result = Solution::reverse_between(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 4, 9);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 9, 8, 7, 6, 5, 4, 10]);
}
