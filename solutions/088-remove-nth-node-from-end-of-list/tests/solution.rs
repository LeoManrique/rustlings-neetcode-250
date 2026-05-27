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
    let result = Solution::remove_nth_from_end(build_list(&[100, 90, 80, 70, 60]), 3);
    assert_eq!(list_to_vec(&result), vec![100, 90, 70, 60]);
}

#[test]
fn test_2() {
    let result = Solution::remove_nth_from_end(build_list(&[1, 2]), 1);
    assert_eq!(list_to_vec(&result), vec![1]);
}

#[test]
fn test_3() {
    let result = Solution::remove_nth_from_end(build_list(&[5, 4, 3, 2, 1]), 5);
    assert_eq!(list_to_vec(&result), vec![4, 3, 2, 1]);
}

#[test]
fn test_4() {
    let result = Solution::remove_nth_from_end(build_list(&[1, 2, 3, 4, 5]), 2);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 5]);
}

#[test]
fn test_5() {
    let result = Solution::remove_nth_from_end(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 5);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 7, 8, 9, 10]);
}

#[test]
fn test_6() {
    let result = Solution::remove_nth_from_end(build_list(&[1]), 1);
    assert_eq!(list_to_vec(&result), vec![]);
}

#[test]
fn test_7() {
    let result = Solution::remove_nth_from_end(build_list(&[1, 2, 3]), 3);
    assert_eq!(list_to_vec(&result), vec![2, 3]);
}

#[test]
fn test_8() {
    let result = Solution::remove_nth_from_end(build_list(&[1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43, 45, 47, 49, 51, 53, 55, 57, 59]), 15);
    assert_eq!(list_to_vec(&result), vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 33, 35, 37, 39, 41, 43, 45, 47, 49, 51, 53, 55, 57, 59]);
}

#[test]
fn test_9() {
    let result = Solution::remove_nth_from_end(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]), 15);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]);
}

#[test]
fn test_10() {
    let result = Solution::remove_nth_from_end(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 1);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_11() {
    let result = Solution::remove_nth_from_end(build_list(&[10, 20, 30, 40, 50, 60, 70, 80, 90, 100]), 5);
    assert_eq!(list_to_vec(&result), vec![10, 20, 30, 40, 50, 70, 80, 90, 100]);
}

#[test]
fn test_12() {
    let result = Solution::remove_nth_from_end(build_list(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29]), 1);
    assert_eq!(list_to_vec(&result), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28]);
}

#[test]
fn test_13() {
    let result = Solution::remove_nth_from_end(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 3);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 9, 10]);
}

#[test]
fn test_14() {
    let result = Solution::remove_nth_from_end(build_list(&[30, 20, 10]), 2);
    assert_eq!(list_to_vec(&result), vec![30, 10]);
}

#[test]
fn test_15() {
    let result = Solution::remove_nth_from_end(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]), 29);
    assert_eq!(list_to_vec(&result), vec![1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]);
}

#[test]
fn test_16() {
    let result = Solution::remove_nth_from_end(build_list(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 30);
    assert_eq!(list_to_vec(&result), vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_17() {
    let result = Solution::remove_nth_from_end(build_list(&[7, 14, 21, 28, 35, 42, 49, 56, 63, 70, 77, 84, 91, 98]), 13);
    assert_eq!(list_to_vec(&result), vec![7, 21, 28, 35, 42, 49, 56, 63, 70, 77, 84, 91, 98]);
}

#[test]
fn test_18() {
    let result = Solution::remove_nth_from_end(build_list(&[10, 20, 30, 40, 50, 60]), 1);
    assert_eq!(list_to_vec(&result), vec![10, 20, 30, 40, 50]);
}

#[test]
fn test_19() {
    let result = Solution::remove_nth_from_end(build_list(&[2, 3, 4, 5]), 1);
    assert_eq!(list_to_vec(&result), vec![2, 3, 4]);
}

#[test]
fn test_20() {
    let result = Solution::remove_nth_from_end(build_list(&[1, 0, 1, 0, 1, 0, 1, 0, 1, 0]), 2);
    assert_eq!(list_to_vec(&result), vec![1, 0, 1, 0, 1, 0, 1, 0, 0]);
}

#[test]
fn test_21() {
    let result = Solution::remove_nth_from_end(build_list(&[5, 10]), 2);
    assert_eq!(list_to_vec(&result), vec![10]);
}

#[test]
fn test_22() {
    let result = Solution::remove_nth_from_end(build_list(&[3, 2, 1]), 1);
    assert_eq!(list_to_vec(&result), vec![3, 2]);
}

#[test]
fn test_23() {
    let result = Solution::remove_nth_from_end(build_list(&[99, 98, 97, 96, 95, 94, 93, 92, 91, 90, 89]), 1);
    assert_eq!(list_to_vec(&result), vec![99, 98, 97, 96, 95, 94, 93, 92, 91, 90]);
}

#[test]
fn test_24() {
    let result = Solution::remove_nth_from_end(build_list(&[10, 20, 30, 40, 50, 60, 70, 80, 90]), 5);
    assert_eq!(list_to_vec(&result), vec![10, 20, 30, 40, 60, 70, 80, 90]);
}

#[test]
fn test_25() {
    let result = Solution::remove_nth_from_end(build_list(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 10);
    assert_eq!(list_to_vec(&result), vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_26() {
    let result = Solution::remove_nth_from_end(build_list(&[10, 20, 30, 40, 50, 60, 70, 80, 90, 100]), 10);
    assert_eq!(list_to_vec(&result), vec![20, 30, 40, 50, 60, 70, 80, 90, 100]);
}

#[test]
fn test_27() {
    let result = Solution::remove_nth_from_end(build_list(&[5]), 1);
    assert_eq!(list_to_vec(&result), vec![]);
}

#[test]
fn test_28() {
    let result = Solution::remove_nth_from_end(build_list(&[30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 29);
    assert_eq!(list_to_vec(&result), vec![30, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
}

#[test]
fn test_29() {
    let result = Solution::remove_nth_from_end(build_list(&[3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33, 36, 39, 42, 45]), 7);
    assert_eq!(list_to_vec(&result), vec![3, 6, 9, 12, 15, 18, 21, 24, 30, 33, 36, 39, 42, 45]);
}

#[test]
fn test_30() {
    let result = Solution::remove_nth_from_end(build_list(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 29);
    assert_eq!(list_to_vec(&result), vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_31() {
    let result = Solution::remove_nth_from_end(build_list(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 10);
    assert_eq!(list_to_vec(&result), vec![1, 1, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_32() {
    let result = Solution::remove_nth_from_end(build_list(&[2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44, 46, 48, 50, 52, 54, 56, 58, 60]), 30);
    assert_eq!(list_to_vec(&result), vec![4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44, 46, 48, 50, 52, 54, 56, 58, 60]);
}

#[test]
fn test_33() {
    let result = Solution::remove_nth_from_end(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 10);
    assert_eq!(list_to_vec(&result), vec![2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_34() {
    let result = Solution::remove_nth_from_end(build_list(&[5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80, 85, 90, 95, 100]), 10);
    assert_eq!(list_to_vec(&result), vec![5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 60, 65, 70, 75, 80, 85, 90, 95, 100]);
}

#[test]
fn test_35() {
    let result = Solution::remove_nth_from_end(build_list(&[1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 30]), 16);
    assert_eq!(list_to_vec(&result), vec![3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 30]);
}

#[test]
fn test_36() {
    let result = Solution::remove_nth_from_end(build_list(&[30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 1);
    assert_eq!(list_to_vec(&result), vec![30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2]);
}

#[test]
fn test_37() {
    let result = Solution::remove_nth_from_end(build_list(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 5);
    assert_eq!(list_to_vec(&result), vec![1, 1, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_38() {
    let result = Solution::remove_nth_from_end(build_list(&[100, 99, 98, 97, 96, 95, 94, 93, 92, 91, 90]), 3);
    assert_eq!(list_to_vec(&result), vec![100, 99, 98, 97, 96, 95, 94, 93, 91, 90]);
}

#[test]
fn test_39() {
    let result = Solution::remove_nth_from_end(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 1);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
}

#[test]
fn test_40() {
    let result = Solution::remove_nth_from_end(build_list(&[3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 9, 7, 9, 3, 2, 3, 8, 4, 6, 2, 6, 4, 3, 3, 8, 3, 2, 7, 9]), 10);
    assert_eq!(list_to_vec(&result), vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 9, 7, 9, 3, 2, 3, 8, 4, 6, 6, 4, 3, 3, 8, 3, 2, 7, 9]);
}

#[test]
fn test_41() {
    let result = Solution::remove_nth_from_end(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]), 25);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]);
}

#[test]
fn test_42() {
    let result = Solution::remove_nth_from_end(build_list(&[5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]), 28);
    assert_eq!(list_to_vec(&result), vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]);
}

#[test]
fn test_43() {
    let result = Solution::remove_nth_from_end(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 10);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
}

#[test]
fn test_44() {
    let result = Solution::remove_nth_from_end(build_list(&[5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 1]), 29);
    assert_eq!(list_to_vec(&result), vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 1]);
}

#[test]
fn test_45() {
    let result = Solution::remove_nth_from_end(build_list(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 20);
    assert_eq!(list_to_vec(&result), vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_46() {
    let result = Solution::remove_nth_from_end(build_list(&[3, 5, 1, 2, 4, 7, 6, 8, 9]), 3);
    assert_eq!(list_to_vec(&result), vec![3, 5, 1, 2, 4, 7, 8, 9]);
}

#[test]
fn test_47() {
    let result = Solution::remove_nth_from_end(build_list(&[2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30]), 1);
    assert_eq!(list_to_vec(&result), vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28]);
}

#[test]
fn test_48() {
    let result = Solution::remove_nth_from_end(build_list(&[1, 2]), 2);
    assert_eq!(list_to_vec(&result), vec![2]);
}

#[test]
fn test_49() {
    let result = Solution::remove_nth_from_end(build_list(&[1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43, 45, 47, 49, 51, 53, 55, 57, 59]), 28);
    assert_eq!(list_to_vec(&result), vec![1, 3, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43, 45, 47, 49, 51, 53, 55, 57, 59]);
}

#[test]
fn test_50() {
    let result = Solution::remove_nth_from_end(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 4);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 8, 9, 10]);
}

#[test]
fn test_51() {
    let result = Solution::remove_nth_from_end(build_list(&[9, 8, 7, 6, 5, 4, 3, 2, 1]), 4);
    assert_eq!(list_to_vec(&result), vec![9, 8, 7, 6, 5, 3, 2, 1]);
}

#[test]
fn test_52() {
    let result = Solution::remove_nth_from_end(build_list(&[30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 15);
    assert_eq!(list_to_vec(&result), vec![30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
}

#[test]
fn test_53() {
    let result = Solution::remove_nth_from_end(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]), 1);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29]);
}

#[test]
fn test_54() {
    let result = Solution::remove_nth_from_end(build_list(&[2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]), 2);
    assert_eq!(list_to_vec(&result), vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]);
}

#[test]
fn test_55() {
    let result = Solution::remove_nth_from_end(build_list(&[2, 3, 4, 5]), 4);
    assert_eq!(list_to_vec(&result), vec![3, 4, 5]);
}

#[test]
fn test_56() {
    let result = Solution::remove_nth_from_end(build_list(&[10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150]), 14);
    assert_eq!(list_to_vec(&result), vec![10, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150]);
}

#[test]
fn test_57() {
    let result = Solution::remove_nth_from_end(build_list(&[10, 20, 30, 40, 50, 60, 70, 80, 90, 100]), 1);
    assert_eq!(list_to_vec(&result), vec![10, 20, 30, 40, 50, 60, 70, 80, 90]);
}

#[test]
fn test_58() {
    let result = Solution::remove_nth_from_end(build_list(&[29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 29);
    assert_eq!(list_to_vec(&result), vec![28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
}

#[test]
fn test_59() {
    let result = Solution::remove_nth_from_end(build_list(&[3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3]), 15);
    assert_eq!(list_to_vec(&result), vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3]);
}

#[test]
fn test_60() {
    let result = Solution::remove_nth_from_end(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]), 30);
    assert_eq!(list_to_vec(&result), vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]);
}

#[test]
fn test_61() {
    let result = Solution::remove_nth_from_end(build_list(&[5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]), 25);
    assert_eq!(list_to_vec(&result), vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]);
}

#[test]
fn test_62() {
    let result = Solution::remove_nth_from_end(build_list(&[99, 98, 97, 96, 95, 94, 93, 92, 91, 90, 89, 88, 87, 86, 85, 84, 83, 82, 81, 80, 79, 78, 77, 76, 75, 74, 73, 72, 71, 70]), 15);
    assert_eq!(list_to_vec(&result), vec![99, 98, 97, 96, 95, 94, 93, 92, 91, 90, 89, 88, 87, 86, 85, 83, 82, 81, 80, 79, 78, 77, 76, 75, 74, 73, 72, 71, 70]);
}

#[test]
fn test_63() {
    let result = Solution::remove_nth_from_end(build_list(&[6, 2, 8, 2, 8, 4, 1, 8, 5, 2, 8, 6, 4, 3, 2, 8, 6, 2, 4, 3, 8, 2, 6, 4, 3, 2, 8, 6, 2, 4]), 20);
    assert_eq!(list_to_vec(&result), vec![6, 2, 8, 2, 8, 4, 1, 8, 5, 2, 6, 4, 3, 2, 8, 6, 2, 4, 3, 8, 2, 6, 4, 3, 2, 8, 6, 2, 4]);
}

#[test]
fn test_64() {
    let result = Solution::remove_nth_from_end(build_list(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 20);
    assert_eq!(list_to_vec(&result), vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_65() {
    let result = Solution::remove_nth_from_end(build_list(&[100, 99, 98, 97, 96, 95, 94, 93, 92, 91, 90, 89, 88, 87, 86, 85, 84, 83, 82, 81, 80, 79, 78, 77, 76, 75, 74, 73, 72, 71]), 10);
    assert_eq!(list_to_vec(&result), vec![100, 99, 98, 97, 96, 95, 94, 93, 92, 91, 90, 89, 88, 87, 86, 85, 84, 83, 82, 81, 79, 78, 77, 76, 75, 74, 73, 72, 71]);
}

#[test]
fn test_66() {
    let result = Solution::remove_nth_from_end(build_list(&[10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 10);
    assert_eq!(list_to_vec(&result), vec![9, 8, 7, 6, 5, 4, 3, 2, 1]);
}

#[test]
fn test_67() {
    let result = Solution::remove_nth_from_end(build_list(&[99, 88, 77, 66, 55, 44, 33, 22, 11]), 9);
    assert_eq!(list_to_vec(&result), vec![88, 77, 66, 55, 44, 33, 22, 11]);
}

#[test]
fn test_68() {
    let result = Solution::remove_nth_from_end(build_list(&[10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 1);
    assert_eq!(list_to_vec(&result), vec![10, 9, 8, 7, 6, 5, 4, 3, 2]);
}

#[test]
fn test_69() {
    let result = Solution::remove_nth_from_end(build_list(&[2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 28, 26, 24, 22, 20, 18, 16, 14, 12, 10, 8, 6, 4, 2]), 14);
    assert_eq!(list_to_vec(&result), vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 26, 24, 22, 20, 18, 16, 14, 12, 10, 8, 6, 4, 2]);
}

#[test]
fn test_70() {
    let result = Solution::remove_nth_from_end(build_list(&[7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7]), 15);
    assert_eq!(list_to_vec(&result), vec![7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7]);
}

#[test]
fn test_71() {
    let result = Solution::remove_nth_from_end(build_list(&[99, 98, 97, 96, 95, 94, 93, 92, 91, 90, 89, 88, 87, 86, 85, 84, 83, 82, 81, 80, 79, 78, 77, 76, 75, 74, 73, 72, 71]), 1);
    assert_eq!(list_to_vec(&result), vec![99, 98, 97, 96, 95, 94, 93, 92, 91, 90, 89, 88, 87, 86, 85, 84, 83, 82, 81, 80, 79, 78, 77, 76, 75, 74, 73, 72]);
}

#[test]
fn test_72() {
    let result = Solution::remove_nth_from_end(build_list(&[100, 99, 98, 97, 96, 95, 94, 93, 92, 91, 90, 89, 88, 87, 86, 85, 84, 83, 82, 81, 80, 79, 78, 77, 76, 75, 74, 73, 72, 71]), 1);
    assert_eq!(list_to_vec(&result), vec![100, 99, 98, 97, 96, 95, 94, 93, 92, 91, 90, 89, 88, 87, 86, 85, 84, 83, 82, 81, 80, 79, 78, 77, 76, 75, 74, 73, 72]);
}

#[test]
fn test_73() {
    let result = Solution::remove_nth_from_end(build_list(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]), 30);
    assert_eq!(list_to_vec(&result), vec![0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]);
}

#[test]
fn test_74() {
    let result = Solution::remove_nth_from_end(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 7);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 10, 11, 12, 13, 14, 15]);
}

#[test]
fn test_75() {
    let result = Solution::remove_nth_from_end(build_list(&[3, 2, 1]), 2);
    assert_eq!(list_to_vec(&result), vec![3, 1]);
}

#[test]
fn test_76() {
    let result = Solution::remove_nth_from_end(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]), 2);
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 30]);
}

#[test]
fn test_77() {
    let result = Solution::remove_nth_from_end(build_list(&[1, 2, 2, 3, 4, 5, 5, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 8);
    assert_eq!(list_to_vec(&result), vec![1, 2, 2, 3, 4, 5, 5, 5, 6, 7, 9, 10, 11, 12, 13, 14, 15]);
}

#[test]
fn test_78() {
    let result = Solution::remove_nth_from_end(build_list(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 15);
    assert_eq!(list_to_vec(&result), vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_79() {
    let result = Solution::remove_nth_from_end(build_list(&[9, 8, 7, 6, 5, 4, 3, 2, 1]), 9);
    assert_eq!(list_to_vec(&result), vec![8, 7, 6, 5, 4, 3, 2, 1]);
}
