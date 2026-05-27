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
    assert_eq!(Solution::reorder_list(build_list(&[1, 2, 3])), None);
}

#[test]
fn test_2() {
    assert_eq!(Solution::reorder_list(build_list(&[1, 2, 3, 4, 5])), None);
}

#[test]
fn test_3() {
    assert_eq!(Solution::reorder_list(build_list(&[1])), None);
}

#[test]
fn test_4() {
    assert_eq!(Solution::reorder_list(build_list(&[1, 2])), None);
}

#[test]
fn test_5() {
    assert_eq!(Solution::reorder_list(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10])), None);
}

#[test]
fn test_6() {
    assert_eq!(Solution::reorder_list(build_list(&[1, 2, 3, 4])), None);
}

#[test]
fn test_7() {
    assert_eq!(Solution::reorder_list(build_list(&[1, 2, 3, 4, 5, 6])), None);
}

#[test]
fn test_8() {
    assert_eq!(Solution::reorder_list(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21])), None);
}

#[test]
fn test_9() {
    assert_eq!(Solution::reorder_list(build_list(&[5, 4, 3, 2, 1])), None);
}

#[test]
fn test_10() {
    assert_eq!(Solution::reorder_list(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40])), None);
}

#[test]
fn test_11() {
    assert_eq!(Solution::reorder_list(build_list(&[1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21])), None);
}

#[test]
fn test_12() {
    assert_eq!(Solution::reorder_list(build_list(&[2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34])), None);
}

#[test]
fn test_13() {
    assert_eq!(Solution::reorder_list(build_list(&[2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44, 46, 48, 50])), None);
}

#[test]
fn test_14() {
    assert_eq!(Solution::reorder_list(build_list(&[10, 9, 8, 7, 6, 5, 4, 3, 2, 1])), None);
}

#[test]
fn test_15() {
    assert_eq!(Solution::reorder_list(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14])), None);
}

#[test]
fn test_16() {
    assert_eq!(Solution::reorder_list(build_list(&[40, 39, 38, 37, 36, 35, 34, 33, 32, 31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1])), None);
}

#[test]
fn test_17() {
    assert_eq!(Solution::reorder_list(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30])), None);
}

#[test]
fn test_18() {
    assert_eq!(Solution::reorder_list(build_list(&[30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1])), None);
}

#[test]
fn test_19() {
    assert_eq!(Solution::reorder_list(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20])), None);
}

#[test]
fn test_20() {
    assert_eq!(Solution::reorder_list(build_list(&[1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43, 45, 47, 49])), None);
}

#[test]
fn test_21() {
    assert_eq!(Solution::reorder_list(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11])), None);
}

#[test]
fn test_22() {
    assert_eq!(Solution::reorder_list(build_list(&[99, 98, 97, 96, 95, 94, 93, 92, 91, 90, 89, 88, 87, 86, 85, 84, 83, 82, 81, 80, 79, 78, 77, 76, 75, 74, 73, 72, 71, 70, 69, 68, 67, 66, 65, 64, 63, 62, 61, 60, 59, 58, 57, 56, 55, 54, 53, 52, 51, 50])), None);
}

#[test]
fn test_23() {
    assert_eq!(Solution::reorder_list(build_list(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1])), None);
}

#[test]
fn test_24() {
    assert_eq!(Solution::reorder_list(build_list(&[2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44, 46, 48, 50, 52, 54, 56, 58, 60])), None);
}

#[test]
fn test_25() {
    assert_eq!(Solution::reorder_list(build_list(&[1000, 900, 800, 700, 600, 500, 400, 300, 200, 100])), None);
}

#[test]
fn test_26() {
    assert_eq!(Solution::reorder_list(build_list(&[5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80, 85, 90, 95, 100])), None);
}

#[test]
fn test_27() {
    assert_eq!(Solution::reorder_list(build_list(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1])), None);
}

#[test]
fn test_28() {
    assert_eq!(Solution::reorder_list(build_list(&[10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160])), None);
}

#[test]
fn test_29() {
    assert_eq!(Solution::reorder_list(build_list(&[99, 98, 97, 96, 95, 94, 93, 92, 91, 90, 89, 88, 87, 86, 85, 84, 83, 82, 81, 80, 79, 78, 77, 76, 75, 74, 73, 72, 71, 70])), None);
}

#[test]
fn test_30() {
    assert_eq!(Solution::reorder_list(build_list(&[5])), None);
}

#[test]
fn test_31() {
    assert_eq!(Solution::reorder_list(build_list(&[500, 400, 300, 200, 100, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50])), None);
}

#[test]
fn test_32() {
    assert_eq!(Solution::reorder_list(build_list(&[1, 2, 3, 4, 5, 6, 7])), None);
}

#[test]
fn test_33() {
    assert_eq!(Solution::reorder_list(build_list(&[1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35])), None);
}

#[test]
fn test_34() {
    assert_eq!(Solution::reorder_list(build_list(&[50, 40, 30, 20, 10, 5, 15, 25, 35, 45, 55, 65, 75, 85, 95, 105, 115, 125, 135, 145, 155, 165, 175, 185, 195, 205])), None);
}

#[test]
fn test_35() {
    assert_eq!(Solution::reorder_list(build_list(&[1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000])), None);
}

#[test]
fn test_36() {
    assert_eq!(Solution::reorder_list(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13])), None);
}

#[test]
fn test_37() {
    assert_eq!(Solution::reorder_list(build_list(&[5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55])), None);
}

#[test]
fn test_38() {
    assert_eq!(Solution::reorder_list(build_list(&[5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5])), None);
}

#[test]
fn test_39() {
    assert_eq!(Solution::reorder_list(build_list(&[5, 9, 13, 17, 21, 25, 29, 33, 37, 41, 45, 49])), None);
}

#[test]
fn test_40() {
    assert_eq!(Solution::reorder_list(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15])), None);
}

#[test]
fn test_41() {
    assert_eq!(Solution::reorder_list(build_list(&[1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39])), None);
}

#[test]
fn test_42() {
    assert_eq!(Solution::reorder_list(build_list(&[5, 4, 3, 2, 1, 6, 7, 8, 9, 10])), None);
}

#[test]
fn test_43() {
    assert_eq!(Solution::reorder_list(build_list(&[1, 3, 5, 7, 9, 2, 4, 6, 8])), None);
}

#[test]
fn test_44() {
    assert_eq!(Solution::reorder_list(build_list(&[1000, 999, 998, 997, 996, 995, 994, 993, 992, 991, 990, 989])), None);
}

#[test]
fn test_45() {
    assert_eq!(Solution::reorder_list(build_list(&[10, 20, 30, 40, 50, 60, 70, 80])), None);
}

#[test]
fn test_46() {
    assert_eq!(Solution::reorder_list(build_list(&[9, 8, 7, 6, 5, 4, 3, 2, 1])), None);
}

#[test]
fn test_47() {
    assert_eq!(Solution::reorder_list(build_list(&[1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29])), None);
}

#[test]
fn test_48() {
    assert_eq!(Solution::reorder_list(build_list(&[500, 400, 300, 200, 100, 50, 25, 12, 6, 3, 1])), None);
}

#[test]
fn test_49() {
    assert_eq!(Solution::reorder_list(build_list(&[50, 49, 48, 47, 46, 45, 44, 43, 42, 41, 40, 39, 38, 37, 36, 35, 34, 33, 32, 31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1])), None);
}

#[test]
fn test_50() {
    assert_eq!(Solution::reorder_list(build_list(&[10, 20, 30, 40, 50, 60, 70, 80, 90])), None);
}

#[test]
fn test_51() {
    assert_eq!(Solution::reorder_list(build_list(&[1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26])), None);
}

#[test]
fn test_52() {
    assert_eq!(Solution::reorder_list(build_list(&[10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160, 170, 180, 190, 200, 210, 220, 230, 240, 250])), None);
}

#[test]
fn test_53() {
    assert_eq!(Solution::reorder_list(build_list(&[7, 14, 21, 28, 35, 42, 49, 56, 63, 70, 77, 84, 91, 98, 105, 112, 119, 126, 133, 140, 147, 154, 161, 168, 175, 182, 189, 196, 203, 210])), None);
}

#[test]
fn test_54() {
    assert_eq!(Solution::reorder_list(build_list(&[100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200, 1300, 1400, 1500, 1600, 1700, 1800, 1900, 2000])), None);
}

#[test]
fn test_55() {
    assert_eq!(Solution::reorder_list(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31])), None);
}

#[test]
fn test_56() {
    assert_eq!(Solution::reorder_list(build_list(&[1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41])), None);
}

#[test]
fn test_57() {
    assert_eq!(Solution::reorder_list(build_list(&[10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120])), None);
}

#[test]
fn test_58() {
    assert_eq!(Solution::reorder_list(build_list(&[5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65])), None);
}

#[test]
fn test_59() {
    assert_eq!(Solution::reorder_list(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50])), None);
}

#[test]
fn test_60() {
    assert_eq!(Solution::reorder_list(build_list(&[100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200, 1300, 1400, 1500, 1600, 1700, 1800, 1900, 2000, 2100, 2200, 2300, 2400, 2500, 2600, 2700, 2800, 2900, 3000])), None);
}

#[test]
fn test_61() {
    assert_eq!(Solution::reorder_list(build_list(&[5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25])), None);
}

#[test]
fn test_62() {
    assert_eq!(Solution::reorder_list(build_list(&[10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130])), None);
}

#[test]
fn test_63() {
    assert_eq!(Solution::reorder_list(build_list(&[10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, -1, -2, -3, -4, -5, -6, -7, -8, -9, -10])), None);
}

#[test]
fn test_64() {
    assert_eq!(Solution::reorder_list(build_list(&[1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20])), None);
}

#[test]
fn test_65() {
    assert_eq!(Solution::reorder_list(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25])), None);
}

#[test]
fn test_66() {
    assert_eq!(Solution::reorder_list(build_list(&[99, 98, 97, 96, 95, 94, 93, 92, 91, 90, 89, 88, 87, 86, 85, 84, 83, 82, 81, 80])), None);
}

#[test]
fn test_67() {
    assert_eq!(Solution::reorder_list(build_list(&[1, 2, 3, 4, 5, 6, 7, 8])), None);
}
