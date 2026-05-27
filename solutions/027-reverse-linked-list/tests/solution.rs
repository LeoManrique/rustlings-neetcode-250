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
    let result = Solution::reverse_list(build_list(&[5000, -5000, 0, 1000, -1000]));
    assert_eq!(list_to_vec(&result), vec![-1000, 1000, 0, -5000, 5000]);
}

#[test]
fn test_2() {
    let result = Solution::reverse_list(build_list(&[1, 2, 3, 4, 5]));
    assert_eq!(list_to_vec(&result), vec![5, 4, 3, 2, 1]);
}

#[test]
fn test_3() {
    let result = Solution::reverse_list(build_list(&[]));
    assert_eq!(list_to_vec(&result), vec![]);
}

#[test]
fn test_4() {
    let result = Solution::reverse_list(build_list(&[1, 2]));
    assert_eq!(list_to_vec(&result), vec![2, 1]);
}

#[test]
fn test_5() {
    let result = Solution::reverse_list(build_list(&[1]));
    assert_eq!(list_to_vec(&result), vec![1]);
}

#[test]
fn test_6() {
    let result = Solution::reverse_list(build_list(&[-1, -2, -3, -4, -5]));
    assert_eq!(list_to_vec(&result), vec![-5, -4, -3, -2, -1]);
}

#[test]
fn test_7() {
    let result = Solution::reverse_list(build_list(&[3]));
    assert_eq!(list_to_vec(&result), vec![3]);
}

#[test]
fn test_8() {
    let result = Solution::reverse_list(build_list(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]));
    assert_eq!(list_to_vec(&result), vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_9() {
    let result = Solution::reverse_list(build_list(&[-1000, 0, 1000]));
    assert_eq!(list_to_vec(&result), vec![1000, 0, -1000]);
}

#[test]
fn test_10() {
    let result = Solution::reverse_list(build_list(&[5000, -5000, 5000, -5000]));
    assert_eq!(list_to_vec(&result), vec![-5000, 5000, -5000, 5000]);
}

#[test]
fn test_11() {
    let result = Solution::reverse_list(build_list(&[100, 95, 90, 85, 80, 75, 70, 65, 60, 55, 50, 45, 40, 35, 30, 25, 20, 15, 10, 5]));
    assert_eq!(list_to_vec(&result), vec![5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80, 85, 90, 95, 100]);
}

#[test]
fn test_12() {
    let result = Solution::reverse_list(build_list(&[1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21]));
    assert_eq!(list_to_vec(&result), vec![21, 19, 17, 15, 13, 11, 9, 7, 5, 3, 1]);
}

#[test]
fn test_13() {
    let result = Solution::reverse_list(build_list(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
    assert_eq!(list_to_vec(&result), vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_14() {
    let result = Solution::reverse_list(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]));
    assert_eq!(list_to_vec(&result), vec![30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
}

#[test]
fn test_15() {
    let result = Solution::reverse_list(build_list(&[-5000, -4000, -3000, -2000, -1000, 0, 1000, 2000, 3000, 4000, 5000]));
    assert_eq!(list_to_vec(&result), vec![5000, 4000, 3000, 2000, 1000, 0, -1000, -2000, -3000, -4000, -5000]);
}

#[test]
fn test_16() {
    let result = Solution::reverse_list(build_list(&[-5000, -2500, 0, 2500, 5000]));
    assert_eq!(list_to_vec(&result), vec![5000, 2500, 0, -2500, -5000]);
}

#[test]
fn test_17() {
    let result = Solution::reverse_list(build_list(&[1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25]));
    assert_eq!(list_to_vec(&result), vec![25, 23, 21, 19, 17, 15, 13, 11, 9, 7, 5, 3, 1]);
}

#[test]
fn test_18() {
    let result = Solution::reverse_list(build_list(&[-1, -2, -3, -4, -5, -6, -7, -8, -9, -10, -11, -12, -13, -14, -15, -16, -17, -18, -19, -20]));
    assert_eq!(list_to_vec(&result), vec![-20, -19, -18, -17, -16, -15, -14, -13, -12, -11, -10, -9, -8, -7, -6, -5, -4, -3, -2, -1]);
}

#[test]
fn test_19() {
    let result = Solution::reverse_list(build_list(&[-5000]));
    assert_eq!(list_to_vec(&result), vec![-5000]);
}

#[test]
fn test_20() {
    let result = Solution::reverse_list(build_list(&[5000, 2500, 0, -2500, -5000, 5000, 2500, 0, -2500, -5000]));
    assert_eq!(list_to_vec(&result), vec![-5000, -2500, 0, 2500, 5000, -5000, -2500, 0, 2500, 5000]);
}

#[test]
fn test_21() {
    let result = Solution::reverse_list(build_list(&[1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 8, 8, 8, 9, 9, 9, 10, 10, 10]));
    assert_eq!(list_to_vec(&result), vec![10, 10, 10, 9, 9, 9, 8, 8, 8, 7, 7, 7, 6, 6, 6, 5, 5, 5, 4, 4, 4, 3, 3, 3, 2, 2, 2, 1, 1, 1]);
}

#[test]
fn test_22() {
    let result = Solution::reverse_list(build_list(&[0]));
    assert_eq!(list_to_vec(&result), vec![0]);
}

#[test]
fn test_23() {
    let result = Solution::reverse_list(build_list(&[25, 23, 21, 19, 17, 15, 13, 11, 9, 7, 5, 3, 1]));
    assert_eq!(list_to_vec(&result), vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25]);
}

#[test]
fn test_24() {
    let result = Solution::reverse_list(build_list(&[1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39]));
    assert_eq!(list_to_vec(&result), vec![39, 37, 35, 33, 31, 29, 27, 25, 23, 21, 19, 17, 15, 13, 11, 9, 7, 5, 3, 1]);
}

#[test]
fn test_25() {
    let result = Solution::reverse_list(build_list(&[100, 200, 300, 400, 500, 600, 700, 800, 900, 1000]));
    assert_eq!(list_to_vec(&result), vec![1000, 900, 800, 700, 600, 500, 400, 300, 200, 100]);
}

#[test]
fn test_26() {
    let result = Solution::reverse_list(build_list(&[0, 1, 0, 1, 0, 1, 0, 1, 0, 1]));
    assert_eq!(list_to_vec(&result), vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0]);
}

#[test]
fn test_27() {
    let result = Solution::reverse_list(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
    assert_eq!(list_to_vec(&result), vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
}

#[test]
fn test_28() {
    let result = Solution::reverse_list(build_list(&[-5000, 5000, -5000, 5000, -5000, 5000]));
    assert_eq!(list_to_vec(&result), vec![5000, -5000, 5000, -5000, 5000, -5000]);
}

#[test]
fn test_29() {
    let result = Solution::reverse_list(build_list(&[1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10]));
    assert_eq!(list_to_vec(&result), vec![10, 10, 9, 9, 8, 8, 7, 7, 6, 6, 5, 5, 4, 4, 3, 3, 2, 2, 1, 1]);
}

#[test]
fn test_30() {
    let result = Solution::reverse_list(build_list(&[1, -2, 3, -4, 5, -6, 7, -8, 9, -10]));
    assert_eq!(list_to_vec(&result), vec![-10, 9, -8, 7, -6, 5, -4, 3, -2, 1]);
}

#[test]
fn test_31() {
    let result = Solution::reverse_list(build_list(&[15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]));
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
}

#[test]
fn test_32() {
    let result = Solution::reverse_list(build_list(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]));
    assert_eq!(list_to_vec(&result), vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_33() {
    let result = Solution::reverse_list(build_list(&[-1, 1, -2, 2, -3, 3, -4, 4, -5, 5, -6, 6, -7, 7, -8, 8, -9, 9, -10, 10]));
    assert_eq!(list_to_vec(&result), vec![10, -10, 9, -9, 8, -8, 7, -7, 6, -6, 5, -5, 4, -4, 3, -3, 2, -2, 1, -1]);
}

#[test]
fn test_34() {
    let result = Solution::reverse_list(build_list(&[10, 9, 8, 7, 6, 5, 4, 3, 2, 1]));
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_35() {
    let result = Solution::reverse_list(build_list(&[5000, 4999, 4998, 4997, 4996, 4995, 4994, 4993, 4992, 4991]));
    assert_eq!(list_to_vec(&result), vec![4991, 4992, 4993, 4994, 4995, 4996, 4997, 4998, 4999, 5000]);
}

#[test]
fn test_36() {
    let result = Solution::reverse_list(build_list(&[1, -1, 2, -2, 3, -3, 4, -4, 5, -5]));
    assert_eq!(list_to_vec(&result), vec![-5, 5, -4, 4, -3, 3, -2, 2, -1, 1]);
}

#[test]
fn test_37() {
    let result = Solution::reverse_list(build_list(&[1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10]));
    assert_eq!(list_to_vec(&result), vec![10, 10, 9, 9, 8, 8, 7, 7, 6, 6, 5, 5, 4, 4, 3, 3, 2, 2, 1, 1]);
}

#[test]
fn test_38() {
    let result = Solution::reverse_list(build_list(&[5000, -5000, 5000, -5000, 5000, -5000, 5000, -5000, 5000, -5000]));
    assert_eq!(list_to_vec(&result), vec![-5000, 5000, -5000, 5000, -5000, 5000, -5000, 5000, -5000, 5000]);
}

#[test]
fn test_39() {
    let result = Solution::reverse_list(build_list(&[1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21]));
    assert_eq!(list_to_vec(&result), vec![21, 19, 17, 15, 13, 11, 9, 7, 5, 3, 1]);
}

#[test]
fn test_40() {
    let result = Solution::reverse_list(build_list(&[10, 9, 8, 7, 6, 5, 4, 3, 2, 1]));
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_41() {
    let result = Solution::reverse_list(build_list(&[5000]));
    assert_eq!(list_to_vec(&result), vec![5000]);
}

#[test]
fn test_42() {
    let result = Solution::reverse_list(build_list(&[20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]));
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);
}

#[test]
fn test_43() {
    let result = Solution::reverse_list(build_list(&[1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43, 45, 47, 49]));
    assert_eq!(list_to_vec(&result), vec![49, 47, 45, 43, 41, 39, 37, 35, 33, 31, 29, 27, 25, 23, 21, 19, 17, 15, 13, 11, 9, 7, 5, 3, 1]);
}

#[test]
fn test_44() {
    let result = Solution::reverse_list(build_list(&[1000, 900, 800, 700, 600, 500, 400, 300, 200, 100]));
    assert_eq!(list_to_vec(&result), vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000]);
}

#[test]
fn test_45() {
    let result = Solution::reverse_list(build_list(&[21, 19, 17, 15, 13, 11, 9, 7, 5, 3, 1]));
    assert_eq!(list_to_vec(&result), vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21]);
}

#[test]
fn test_46() {
    let result = Solution::reverse_list(build_list(&[5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80, 85, 90, 95, 100]));
    assert_eq!(list_to_vec(&result), vec![100, 95, 90, 85, 80, 75, 70, 65, 60, 55, 50, 45, 40, 35, 30, 25, 20, 15, 10, 5]);
}

#[test]
fn test_47() {
    let result = Solution::reverse_list(build_list(&[1000, 2000, 3000, 4000, 5000, 4000, 3000, 2000, 1000]));
    assert_eq!(list_to_vec(&result), vec![1000, 2000, 3000, 4000, 5000, 4000, 3000, 2000, 1000]);
}

#[test]
fn test_48() {
    let result = Solution::reverse_list(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25]));
    assert_eq!(list_to_vec(&result), vec![25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
}

#[test]
fn test_49() {
    let result = Solution::reverse_list(build_list(&[-10, 9, -8, 7, -6, 5, -4, 3, -2, 1]));
    assert_eq!(list_to_vec(&result), vec![1, -2, 3, -4, 5, -6, 7, -8, 9, -10]);
}

#[test]
fn test_50() {
    let result = Solution::reverse_list(build_list(&[2, 2, 2, 1, 1, 1, 3, 3, 3, 4, 4, 4]));
    assert_eq!(list_to_vec(&result), vec![4, 4, 4, 3, 3, 3, 1, 1, 1, 2, 2, 2]);
}

#[test]
fn test_51() {
    let result = Solution::reverse_list(build_list(&[-1]));
    assert_eq!(list_to_vec(&result), vec![-1]);
}

#[test]
fn test_52() {
    let result = Solution::reverse_list(build_list(&[5, 5, 5, 5, 5, 4, 4, 4, 4, 4, 3, 3, 3, 3, 3, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1]));
    assert_eq!(list_to_vec(&result), vec![1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5]);
}

#[test]
fn test_53() {
    let result = Solution::reverse_list(build_list(&[2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40]));
    assert_eq!(list_to_vec(&result), vec![40, 38, 36, 34, 32, 30, 28, 26, 24, 22, 20, 18, 16, 14, 12, 10, 8, 6, 4, 2]);
}

#[test]
fn test_54() {
    let result = Solution::reverse_list(build_list(&[25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]));
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25]);
}

#[test]
fn test_55() {
    let result = Solution::reverse_list(build_list(&[1, 3, 5, 7, 9, 11, 13, 15]));
    assert_eq!(list_to_vec(&result), vec![15, 13, 11, 9, 7, 5, 3, 1]);
}

#[test]
fn test_56() {
    let result = Solution::reverse_list(build_list(&[-1, -2, -3, -4, -5, -6, -7, -8, -9, -10]));
    assert_eq!(list_to_vec(&result), vec![-10, -9, -8, -7, -6, -5, -4, -3, -2, -1]);
}

#[test]
fn test_57() {
    let result = Solution::reverse_list(build_list(&[5000, 4999, 4998, 4997, 4996, 4995, 4994, 4993, 4992, 4991, 4990, 4989, 4988, 4987, 4986, 4985, 4984, 4983, 4982, 4981, 4980, 4979, 4978, 4977, 4976, 4975, 4974, 4973, 4972, 4971, 4970]));
    assert_eq!(list_to_vec(&result), vec![4970, 4971, 4972, 4973, 4974, 4975, 4976, 4977, 4978, 4979, 4980, 4981, 4982, 4983, 4984, 4985, 4986, 4987, 4988, 4989, 4990, 4991, 4992, 4993, 4994, 4995, 4996, 4997, 4998, 4999, 5000]);
}

#[test]
fn test_58() {
    let result = Solution::reverse_list(build_list(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
    assert_eq!(list_to_vec(&result), vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_59() {
    let result = Solution::reverse_list(build_list(&[10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150]));
    assert_eq!(list_to_vec(&result), vec![150, 140, 130, 120, 110, 100, 90, 80, 70, 60, 50, 40, 30, 20, 10]);
}

#[test]
fn test_60() {
    let result = Solution::reverse_list(build_list(&[1000, 2000, 3000, 4000, 5000, -1000, -2000, -3000, -4000, -5000]));
    assert_eq!(list_to_vec(&result), vec![-5000, -4000, -3000, -2000, -1000, 5000, 4000, 3000, 2000, 1000]);
}

#[test]
fn test_61() {
    let result = Solution::reverse_list(build_list(&[1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9, 0, 10, 0]));
    assert_eq!(list_to_vec(&result), vec![0, 10, 0, 9, 0, 8, 0, 7, 0, 6, 0, 5, 0, 4, 0, 3, 0, 2, 0, 1]);
}

#[test]
fn test_62() {
    let result = Solution::reverse_list(build_list(&[100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200, 1300, 1400, 1500]));
    assert_eq!(list_to_vec(&result), vec![1500, 1400, 1300, 1200, 1100, 1000, 900, 800, 700, 600, 500, 400, 300, 200, 100]);
}

#[test]
fn test_63() {
    let result = Solution::reverse_list(build_list(&[0, 0, 0, 0, 0, 0, 0, 0]));
    assert_eq!(list_to_vec(&result), vec![0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_64() {
    let result = Solution::reverse_list(build_list(&[-5000, 5000, -4000, 4000, -3000, 3000, -2000, 2000, -1000, 1000]));
    assert_eq!(list_to_vec(&result), vec![1000, -1000, 2000, -2000, 3000, -3000, 4000, -4000, 5000, -5000]);
}

#[test]
fn test_65() {
    let result = Solution::reverse_list(build_list(&[0, 10, 0, 20, 0, 30, 0, 40, 0, 50, 0, 60, 0, 70, 0, 80, 0, 90, 0, 100, 0]));
    assert_eq!(list_to_vec(&result), vec![0, 100, 0, 90, 0, 80, 0, 70, 0, 60, 0, 50, 0, 40, 0, 30, 0, 20, 0, 10, 0]);
}

#[test]
fn test_66() {
    let result = Solution::reverse_list(build_list(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]));
    assert_eq!(list_to_vec(&result), vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_67() {
    let result = Solution::reverse_list(build_list(&[1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39]));
    assert_eq!(list_to_vec(&result), vec![39, 37, 35, 33, 31, 29, 27, 25, 23, 21, 19, 17, 15, 13, 11, 9, 7, 5, 3, 1]);
}

#[test]
fn test_68() {
    let result = Solution::reverse_list(build_list(&[1, 1, 2, 2, 3, 3, 4, 4, 5, 5]));
    assert_eq!(list_to_vec(&result), vec![5, 5, 4, 4, 3, 3, 2, 2, 1, 1]);
}

#[test]
fn test_69() {
    let result = Solution::reverse_list(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
    assert_eq!(list_to_vec(&result), vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
}

#[test]
fn test_70() {
    let result = Solution::reverse_list(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]));
    assert_eq!(list_to_vec(&result), vec![20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
}

#[test]
fn test_71() {
    let result = Solution::reverse_list(build_list(&[1, 3, 5, 7, 9, 2, 4, 6, 8, 10]));
    assert_eq!(list_to_vec(&result), vec![10, 8, 6, 4, 2, 9, 7, 5, 3, 1]);
}

#[test]
fn test_72() {
    let result = Solution::reverse_list(build_list(&[-5000, 5000, -4999, 4999, -4998, 4998, -4997, 4997, -4996, 4996]));
    assert_eq!(list_to_vec(&result), vec![4996, -4996, 4997, -4997, 4998, -4998, 4999, -4999, 5000, -5000]);
}

#[test]
fn test_73() {
    let result = Solution::reverse_list(build_list(&[1, 3, 2, 4, 6, 5, 7, 9, 8, 10]));
    assert_eq!(list_to_vec(&result), vec![10, 8, 9, 7, 5, 6, 4, 2, 3, 1]);
}

#[test]
fn test_74() {
    let result = Solution::reverse_list(build_list(&[1, -1, 2, -2, 3, -3, 4, -4, 5, -5, 6, -6, 7, -7, 8, -8, 9, -9, 10, -10]));
    assert_eq!(list_to_vec(&result), vec![-10, 10, -9, 9, -8, 8, -7, 7, -6, 6, -5, 5, -4, 4, -3, 3, -2, 2, -1, 1]);
}

#[test]
fn test_75() {
    let result = Solution::reverse_list(build_list(&[1, 1, 2, 2, 3, 3, 2, 2, 1, 1]));
    assert_eq!(list_to_vec(&result), vec![1, 1, 2, 2, 3, 3, 2, 2, 1, 1]);
}

#[test]
fn test_76() {
    let result = Solution::reverse_list(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]));
    assert_eq!(list_to_vec(&result), vec![15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
}

#[test]
fn test_77() {
    let result = Solution::reverse_list(build_list(&[5000, 4000, 3000, 2000, 1000, 0, -1000, -2000, -3000, -4000, -5000]));
    assert_eq!(list_to_vec(&result), vec![-5000, -4000, -3000, -2000, -1000, 0, 1000, 2000, 3000, 4000, 5000]);
}

#[test]
fn test_78() {
    let result = Solution::reverse_list(build_list(&[1, 2, 3, 2, 1]));
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 2, 1]);
}

#[test]
fn test_79() {
    let result = Solution::reverse_list(build_list(&[100, 200, 300, 400, 500, 600, 700, 800, 900, 1000]));
    assert_eq!(list_to_vec(&result), vec![1000, 900, 800, 700, 600, 500, 400, 300, 200, 100]);
}

#[test]
fn test_80() {
    let result = Solution::reverse_list(build_list(&[5, -5, 10, -10, 15, -15, 20, -20, 25, -25, 30, -30, 35, -35, 40, -40]));
    assert_eq!(list_to_vec(&result), vec![-40, 40, -35, 35, -30, 30, -25, 25, -20, 20, -15, 15, -10, 10, -5, 5]);
}

#[test]
fn test_81() {
    let result = Solution::reverse_list(build_list(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]));
    assert_eq!(list_to_vec(&result), vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_82() {
    let result = Solution::reverse_list(build_list(&[100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200, 1300, 1400, 1500, 1600, 1700, 1800, 1900, 2000]));
    assert_eq!(list_to_vec(&result), vec![2000, 1900, 1800, 1700, 1600, 1500, 1400, 1300, 1200, 1100, 1000, 900, 800, 700, 600, 500, 400, 300, 200, 100]);
}

#[test]
fn test_83() {
    let result = Solution::reverse_list(build_list(&[0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]));
    assert_eq!(list_to_vec(&result), vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0]);
}

#[test]
fn test_84() {
    let result = Solution::reverse_list(build_list(&[1000, -1000, 2000, -2000, 3000, -3000, 4000, -4000, 5000, -5000]));
    assert_eq!(list_to_vec(&result), vec![-5000, 5000, -4000, 4000, -3000, 3000, -2000, 2000, -1000, 1000]);
}

#[test]
fn test_85() {
    let result = Solution::reverse_list(build_list(&[5000, 2500, 0, -2500, -5000]));
    assert_eq!(list_to_vec(&result), vec![-5000, -2500, 0, 2500, 5000]);
}

#[test]
fn test_86() {
    let result = Solution::reverse_list(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]));
    assert_eq!(list_to_vec(&result), vec![20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
}

#[test]
fn test_87() {
    let result = Solution::reverse_list(build_list(&[1000, 900, 800, 700, 600, 500, 400, 300, 200, 100]));
    assert_eq!(list_to_vec(&result), vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000]);
}

#[test]
fn test_88() {
    let result = Solution::reverse_list(build_list(&[1, 1, 2, 2, 3, 3, 4, 4, 5, 5]));
    assert_eq!(list_to_vec(&result), vec![5, 5, 4, 4, 3, 3, 2, 2, 1, 1]);
}

#[test]
fn test_89() {
    let result = Solution::reverse_list(build_list(&[20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]));
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);
}

#[test]
fn test_90() {
    let result = Solution::reverse_list(build_list(&[500, 400, 300, 200, 100, 50, 25, 12, 6, 3, 1, 0, -1, -3, -6, -12, -25, -50, -100, -200, -300, -400, -500]));
    assert_eq!(list_to_vec(&result), vec![-500, -400, -300, -200, -100, -50, -25, -12, -6, -3, -1, 0, 1, 3, 6, 12, 25, 50, 100, 200, 300, 400, 500]);
}

#[test]
fn test_91() {
    let result = Solution::reverse_list(build_list(&[-1, -1, -1, -1, -1, -1, -1, -1, -1, -1]));
    assert_eq!(list_to_vec(&result), vec![-1, -1, -1, -1, -1, -1, -1, -1, -1, -1]);
}

#[test]
fn test_92() {
    let result = Solution::reverse_list(build_list(&[49, 47, 45, 43, 41, 39, 37, 35, 33, 31, 29, 27, 25, 23, 21, 19, 17, 15, 13, 11, 9, 7, 5, 3, 1]));
    assert_eq!(list_to_vec(&result), vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43, 45, 47, 49]);
}

#[test]
fn test_93() {
    let result = Solution::reverse_list(build_list(&[-5000, 5000, -5000, 5000, -5000, 5000, -5000, 5000, -5000, 5000]));
    assert_eq!(list_to_vec(&result), vec![5000, -5000, 5000, -5000, 5000, -5000, 5000, -5000, 5000, -5000]);
}

#[test]
fn test_94() {
    let result = Solution::reverse_list(build_list(&[-1, -2, -3, -4, -5, -6, -7, -8, -9, -10]));
    assert_eq!(list_to_vec(&result), vec![-10, -9, -8, -7, -6, -5, -4, -3, -2, -1]);
}

#[test]
fn test_95() {
    let result = Solution::reverse_list(build_list(&[1, -1, 2, -2, 3, -3, 4, -4, 5, -5]));
    assert_eq!(list_to_vec(&result), vec![-5, 5, -4, 4, -3, 3, -2, 2, -1, 1]);
}

#[test]
fn test_96() {
    let result = Solution::reverse_list(build_list(&[21, 19, 17, 15, 13, 11, 9, 7, 5, 3, 1]));
    assert_eq!(list_to_vec(&result), vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21]);
}

#[test]
fn test_97() {
    let result = Solution::reverse_list(build_list(&[1, 2, 3, 4, 5, 4, 3, 2, 1]));
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 4, 3, 2, 1]);
}

#[test]
fn test_98() {
    let result = Solution::reverse_list(build_list(&[5000, -5000, 4000, -4000, 3000, -3000, 2000, -2000, 1000, -1000, 0]));
    assert_eq!(list_to_vec(&result), vec![0, -1000, 1000, -2000, 2000, -3000, 3000, -4000, 4000, -5000, 5000]);
}

#[test]
fn test_99() {
    let result = Solution::reverse_list(build_list(&[5, -5, 5, -5, 5, -5, 5, -5, 5, -5, 5, -5]));
    assert_eq!(list_to_vec(&result), vec![-5, 5, -5, 5, -5, 5, -5, 5, -5, 5, -5, 5]);
}
