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
    let result = Solution::insert_greatest_common_divisors(build_list(&[7, 14, 28, 56]));
    assert_eq!(list_to_vec(&result), vec![7, 7, 14, 14, 28, 28, 56]);
}

#[test]
fn test_2() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[33, 51, 68]));
    assert_eq!(list_to_vec(&result), vec![33, 3, 51, 17, 68]);
}

#[test]
fn test_3() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[1, 2, 3, 4, 5]));
    assert_eq!(list_to_vec(&result), vec![1, 1, 2, 1, 3, 1, 4, 1, 5]);
}

#[test]
fn test_4() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[10, 20, 30, 40, 50]));
    assert_eq!(list_to_vec(&result), vec![10, 10, 20, 10, 30, 10, 40, 10, 50]);
}

#[test]
fn test_5() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[1000, 1000]));
    assert_eq!(list_to_vec(&result), vec![1000, 1000, 1000]);
}

#[test]
fn test_6() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[3, 9, 27, 81]));
    assert_eq!(list_to_vec(&result), vec![3, 3, 9, 9, 27, 27, 81]);
}

#[test]
fn test_7() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[48, 18, 30]));
    assert_eq!(list_to_vec(&result), vec![48, 6, 18, 6, 30]);
}

#[test]
fn test_8() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[100, 200, 300]));
    assert_eq!(list_to_vec(&result), vec![100, 100, 200, 100, 300]);
}

#[test]
fn test_9() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[100, 50, 25, 10]));
    assert_eq!(list_to_vec(&result), vec![100, 50, 50, 25, 25, 5, 10]);
}

#[test]
fn test_10() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[3, 3, 3, 3, 3]));
    assert_eq!(list_to_vec(&result), vec![3, 3, 3, 3, 3, 3, 3, 3, 3]);
}

#[test]
fn test_11() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[7]));
    assert_eq!(list_to_vec(&result), vec![7]);
}

#[test]
fn test_12() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[12, 15, 20]));
    assert_eq!(list_to_vec(&result), vec![12, 3, 15, 5, 20]);
}

#[test]
fn test_13() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[42, 56, 14]));
    assert_eq!(list_to_vec(&result), vec![42, 14, 56, 14, 14]);
}

#[test]
fn test_14() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[7, 7, 7, 7]));
    assert_eq!(list_to_vec(&result), vec![7, 7, 7, 7, 7, 7, 7]);
}

#[test]
fn test_15() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[18, 6, 10, 3]));
    assert_eq!(list_to_vec(&result), vec![18, 6, 6, 2, 10, 1, 3]);
}

#[test]
fn test_16() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[8, 12, 16, 20]));
    assert_eq!(list_to_vec(&result), vec![8, 4, 12, 4, 16, 4, 20]);
}

#[test]
fn test_17() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[99, 81, 27]));
    assert_eq!(list_to_vec(&result), vec![99, 9, 81, 27, 27]);
}

#[test]
fn test_18() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[5, 15, 25, 35]));
    assert_eq!(list_to_vec(&result), vec![5, 5, 15, 5, 25, 5, 35]);
}

#[test]
fn test_19() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[5, 10, 15, 20, 25]));
    assert_eq!(list_to_vec(&result), vec![5, 5, 10, 5, 15, 5, 20, 5, 25]);
}

#[test]
fn test_20() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[12, 15, 21]));
    assert_eq!(list_to_vec(&result), vec![12, 3, 15, 3, 21]);
}

#[test]
fn test_21() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[1000, 500, 250, 125]));
    assert_eq!(list_to_vec(&result), vec![1000, 500, 500, 250, 250, 125, 125]);
}

#[test]
fn test_22() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[3, 5, 8, 12]));
    assert_eq!(list_to_vec(&result), vec![3, 1, 5, 1, 8, 4, 12]);
}

#[test]
fn test_23() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[1000, 1000, 1000]));
    assert_eq!(list_to_vec(&result), vec![1000, 1000, 1000, 1000, 1000]);
}

#[test]
fn test_24() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[98, 49, 24, 12, 6, 3]));
    assert_eq!(list_to_vec(&result), vec![98, 49, 49, 1, 24, 12, 12, 6, 6, 3, 3]);
}

#[test]
fn test_25() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[13, 17, 19, 23, 29]));
    assert_eq!(list_to_vec(&result), vec![13, 1, 17, 1, 19, 1, 23, 1, 29]);
}

#[test]
fn test_26() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[11, 22, 33, 44, 55, 66, 77, 88, 99]));
    assert_eq!(list_to_vec(&result), vec![11, 11, 22, 11, 33, 11, 44, 11, 55, 11, 66, 11, 77, 11, 88, 11, 99]);
}

#[test]
fn test_27() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[2023, 2021, 2019, 2017, 2015]));
    assert_eq!(list_to_vec(&result), vec![2023, 1, 2021, 1, 2019, 1, 2017, 1, 2015]);
}

#[test]
fn test_28() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[21, 14, 49, 35, 70, 56]));
    assert_eq!(list_to_vec(&result), vec![21, 7, 14, 7, 49, 7, 35, 35, 70, 14, 56]);
}

#[test]
fn test_29() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[77, 49, 35, 91, 63]));
    assert_eq!(list_to_vec(&result), vec![77, 7, 49, 7, 35, 7, 91, 7, 63]);
}

#[test]
fn test_30() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[60, 45, 30, 15]));
    assert_eq!(list_to_vec(&result), vec![60, 15, 45, 15, 30, 15, 15]);
}

#[test]
fn test_31() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[60, 30, 15, 5, 1]));
    assert_eq!(list_to_vec(&result), vec![60, 30, 30, 15, 15, 5, 5, 1, 1]);
}

#[test]
fn test_32() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[1000, 500, 250, 125, 62, 31, 15, 7, 3, 1]));
    assert_eq!(list_to_vec(&result), vec![1000, 500, 500, 250, 250, 125, 125, 1, 62, 31, 31, 1, 15, 1, 7, 1, 3, 1, 1]);
}

#[test]
fn test_33() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[112, 128, 144, 160, 176]));
    assert_eq!(list_to_vec(&result), vec![112, 16, 128, 16, 144, 16, 160, 16, 176]);
}

#[test]
fn test_34() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[77, 14, 28, 49]));
    assert_eq!(list_to_vec(&result), vec![77, 7, 14, 14, 28, 7, 49]);
}

#[test]
fn test_35() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[21, 35, 105, 175]));
    assert_eq!(list_to_vec(&result), vec![21, 7, 35, 35, 105, 35, 175]);
}

#[test]
fn test_36() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[8, 12, 18, 24]));
    assert_eq!(list_to_vec(&result), vec![8, 4, 12, 6, 18, 6, 24]);
}

#[test]
fn test_37() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[8, 12, 16, 20, 24, 28]));
    assert_eq!(list_to_vec(&result), vec![8, 4, 12, 4, 16, 4, 20, 4, 24, 4, 28]);
}

#[test]
fn test_38() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[11, 22, 33, 44, 55, 66, 77]));
    assert_eq!(list_to_vec(&result), vec![11, 11, 22, 11, 33, 11, 44, 11, 55, 11, 66, 11, 77]);
}

#[test]
fn test_39() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[21, 14, 7, 49, 35, 28]));
    assert_eq!(list_to_vec(&result), vec![21, 7, 14, 7, 7, 7, 49, 7, 35, 7, 28]);
}

#[test]
fn test_40() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1]));
    assert_eq!(list_to_vec(&result), vec![1024, 512, 512, 256, 256, 128, 128, 64, 64, 32, 32, 16, 16, 8, 8, 4, 4, 2, 2, 1, 1]);
}

#[test]
fn test_41() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[54, 27, 9, 3, 1, 3, 9, 27, 54]));
    assert_eq!(list_to_vec(&result), vec![54, 27, 27, 9, 9, 3, 3, 1, 1, 1, 3, 3, 9, 9, 27, 27, 54]);
}

#[test]
fn test_42() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[11, 22, 33, 44, 55, 66, 77, 88, 99, 110]));
    assert_eq!(list_to_vec(&result), vec![11, 11, 22, 11, 33, 11, 44, 11, 55, 11, 66, 11, 77, 11, 88, 11, 99, 11, 110]);
}

#[test]
fn test_43() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[150, 120, 90, 60, 30, 15, 5]));
    assert_eq!(list_to_vec(&result), vec![150, 30, 120, 30, 90, 30, 60, 30, 30, 15, 15, 5, 5]);
}

#[test]
fn test_44() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[111, 222, 333, 444, 555, 666, 777, 888, 999]));
    assert_eq!(list_to_vec(&result), vec![111, 111, 222, 111, 333, 111, 444, 111, 555, 111, 666, 111, 777, 111, 888, 111, 999]);
}

#[test]
fn test_45() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1]));
    assert_eq!(list_to_vec(&result), vec![1024, 512, 512, 256, 256, 128, 128, 64, 64, 32, 32, 16, 16, 8, 8, 4, 4, 2, 2, 1, 1]);
}

#[test]
fn test_46() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
    assert_eq!(list_to_vec(&result), vec![1, 1, 2, 1, 3, 1, 4, 1, 5, 1, 6, 1, 7, 1, 8, 1, 9, 1, 10]);
}

#[test]
fn test_47() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[30, 15, 5, 25]));
    assert_eq!(list_to_vec(&result), vec![30, 15, 15, 5, 5, 5, 25]);
}

#[test]
fn test_48() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[72, 48, 60, 36, 90]));
    assert_eq!(list_to_vec(&result), vec![72, 24, 48, 12, 60, 12, 36, 18, 90]);
}

#[test]
fn test_49() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[101, 103, 107, 109]));
    assert_eq!(list_to_vec(&result), vec![101, 1, 103, 1, 107, 1, 109]);
}

#[test]
fn test_50() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[55, 110, 220, 440, 880, 1760, 3520]));
    assert_eq!(list_to_vec(&result), vec![55, 55, 110, 110, 220, 220, 440, 440, 880, 880, 1760, 1760, 3520]);
}

#[test]
fn test_51() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[81, 27, 9, 3, 1]));
    assert_eq!(list_to_vec(&result), vec![81, 27, 27, 9, 9, 3, 3, 1, 1]);
}

#[test]
fn test_52() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[120, 80, 40, 20, 10]));
    assert_eq!(list_to_vec(&result), vec![120, 40, 80, 40, 40, 20, 20, 10, 10]);
}

#[test]
fn test_53() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[48, 64, 80, 96, 112, 128]));
    assert_eq!(list_to_vec(&result), vec![48, 16, 64, 16, 80, 16, 96, 16, 112, 16, 128]);
}

#[test]
fn test_54() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[100, 25, 45, 15, 30]));
    assert_eq!(list_to_vec(&result), vec![100, 25, 25, 5, 45, 15, 15, 15, 30]);
}

#[test]
fn test_55() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[1000, 500, 250, 125, 62, 31, 1]));
    assert_eq!(list_to_vec(&result), vec![1000, 500, 500, 250, 250, 125, 125, 1, 62, 31, 31, 1, 1]);
}

#[test]
fn test_56() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[8, 12, 18, 24, 30]));
    assert_eq!(list_to_vec(&result), vec![8, 4, 12, 6, 18, 6, 24, 6, 30]);
}

#[test]
fn test_57() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[44, 66, 88, 110, 132]));
    assert_eq!(list_to_vec(&result), vec![44, 22, 66, 22, 88, 22, 110, 22, 132]);
}

#[test]
fn test_58() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[225, 150, 100, 50]));
    assert_eq!(list_to_vec(&result), vec![225, 75, 150, 50, 100, 50, 50]);
}

#[test]
fn test_59() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[19, 38, 76, 152]));
    assert_eq!(list_to_vec(&result), vec![19, 19, 38, 38, 76, 76, 152]);
}

#[test]
fn test_60() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[1000, 500, 250, 125, 62, 31, 15, 7, 3, 1]));
    assert_eq!(list_to_vec(&result), vec![1000, 500, 500, 250, 250, 125, 125, 1, 62, 31, 31, 1, 15, 1, 7, 1, 3, 1, 1]);
}

#[test]
fn test_61() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[1, 2, 4, 8, 16, 32]));
    assert_eq!(list_to_vec(&result), vec![1, 1, 2, 2, 4, 4, 8, 8, 16, 16, 32]);
}

#[test]
fn test_62() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[24, 36, 48, 60, 72, 84]));
    assert_eq!(list_to_vec(&result), vec![24, 12, 36, 12, 48, 12, 60, 12, 72, 12, 84]);
}

#[test]
fn test_63() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[77, 14, 49, 7, 21, 35]));
    assert_eq!(list_to_vec(&result), vec![77, 7, 14, 7, 49, 7, 7, 7, 21, 7, 35]);
}

#[test]
fn test_64() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[99, 33, 66, 11, 22]));
    assert_eq!(list_to_vec(&result), vec![99, 33, 33, 33, 66, 11, 11, 11, 22]);
}

#[test]
fn test_65() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[15, 30, 45, 60, 75, 90]));
    assert_eq!(list_to_vec(&result), vec![15, 15, 30, 15, 45, 15, 60, 15, 75, 15, 90]);
}

#[test]
fn test_66() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[23, 46, 69, 92, 115, 138]));
    assert_eq!(list_to_vec(&result), vec![23, 23, 46, 23, 69, 23, 92, 23, 115, 23, 138]);
}

#[test]
fn test_67() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[72, 48, 24, 12, 6]));
    assert_eq!(list_to_vec(&result), vec![72, 24, 48, 24, 24, 12, 12, 6, 6]);
}

#[test]
fn test_68() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[56, 98, 154, 224]));
    assert_eq!(list_to_vec(&result), vec![56, 14, 98, 14, 154, 14, 224]);
}

#[test]
fn test_69() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[100, 25, 50, 20, 40]));
    assert_eq!(list_to_vec(&result), vec![100, 25, 25, 25, 50, 10, 20, 20, 40]);
}

#[test]
fn test_70() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[2000, 1000, 500, 250, 125, 62, 31]));
    assert_eq!(list_to_vec(&result), vec![2000, 1000, 1000, 500, 500, 250, 250, 125, 125, 1, 62, 31, 31]);
}

#[test]
fn test_71() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[6, 12, 18, 24, 30, 36, 42, 48, 54]));
    assert_eq!(list_to_vec(&result), vec![6, 6, 12, 6, 18, 6, 24, 6, 30, 6, 36, 6, 42, 6, 48, 6, 54]);
}

#[test]
fn test_72() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[144, 72, 36, 18, 9, 3, 1]));
    assert_eq!(list_to_vec(&result), vec![144, 72, 72, 36, 36, 18, 18, 9, 9, 3, 3, 1, 1]);
}

#[test]
fn test_73() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[84, 42, 21, 105]));
    assert_eq!(list_to_vec(&result), vec![84, 42, 42, 21, 21, 21, 105]);
}

#[test]
fn test_74() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[315, 360, 405, 450]));
    assert_eq!(list_to_vec(&result), vec![315, 45, 360, 45, 405, 45, 450]);
}

#[test]
fn test_75() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[999, 1000, 998, 997, 996, 995]));
    assert_eq!(list_to_vec(&result), vec![999, 1, 1000, 2, 998, 1, 997, 1, 996, 1, 995]);
}

#[test]
fn test_76() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[84, 105, 140, 175]));
    assert_eq!(list_to_vec(&result), vec![84, 21, 105, 35, 140, 35, 175]);
}

#[test]
fn test_77() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[100, 50, 25, 5]));
    assert_eq!(list_to_vec(&result), vec![100, 50, 50, 25, 25, 5, 5]);
}

#[test]
fn test_78() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[13, 26, 39, 52, 65]));
    assert_eq!(list_to_vec(&result), vec![13, 13, 26, 13, 39, 13, 52, 13, 65]);
}

#[test]
fn test_79() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[100, 25, 10, 50]));
    assert_eq!(list_to_vec(&result), vec![100, 25, 25, 5, 10, 10, 50]);
}

#[test]
fn test_80() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[101, 103, 107, 109, 113]));
    assert_eq!(list_to_vec(&result), vec![101, 1, 103, 1, 107, 1, 109, 1, 113]);
}

#[test]
fn test_81() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[54, 24, 36, 18, 90, 60]));
    assert_eq!(list_to_vec(&result), vec![54, 6, 24, 12, 36, 18, 18, 18, 90, 30, 60]);
}

#[test]
fn test_82() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[144, 120, 96, 72]));
    assert_eq!(list_to_vec(&result), vec![144, 24, 120, 24, 96, 24, 72]);
}

#[test]
fn test_83() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[45, 90, 135, 180, 225, 270]));
    assert_eq!(list_to_vec(&result), vec![45, 45, 90, 45, 135, 45, 180, 45, 225, 45, 270]);
}

#[test]
fn test_84() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[210, 105, 35, 7, 1]));
    assert_eq!(list_to_vec(&result), vec![210, 105, 105, 35, 35, 7, 7, 1, 1]);
}

#[test]
fn test_85() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[100, 25, 10, 5]));
    assert_eq!(list_to_vec(&result), vec![100, 25, 25, 5, 10, 5, 5]);
}

#[test]
fn test_86() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
    assert_eq!(list_to_vec(&result), vec![1, 1, 2, 1, 3, 1, 4, 1, 5, 1, 6, 1, 7, 1, 8, 1, 9, 1, 10]);
}

#[test]
fn test_87() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[100, 25, 50, 75, 125]));
    assert_eq!(list_to_vec(&result), vec![100, 25, 25, 25, 50, 25, 75, 25, 125]);
}

#[test]
fn test_88() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[500, 250, 125, 25]));
    assert_eq!(list_to_vec(&result), vec![500, 250, 250, 125, 125, 25, 25]);
}

#[test]
fn test_89() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[100, 25, 50, 125, 200]));
    assert_eq!(list_to_vec(&result), vec![100, 25, 25, 25, 50, 25, 125, 25, 200]);
}

#[test]
fn test_90() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[100, 25, 50, 125, 200]));
    assert_eq!(list_to_vec(&result), vec![100, 25, 25, 25, 50, 25, 125, 25, 200]);
}

#[test]
fn test_91() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[144, 108, 72, 36, 18]));
    assert_eq!(list_to_vec(&result), vec![144, 36, 108, 36, 72, 36, 36, 18, 18]);
}

#[test]
fn test_92() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[17, 19, 23, 29, 31, 37, 41, 43, 47]));
    assert_eq!(list_to_vec(&result), vec![17, 1, 19, 1, 23, 1, 29, 1, 31, 1, 37, 1, 41, 1, 43, 1, 47]);
}

#[test]
fn test_93() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[120, 180, 300, 420]));
    assert_eq!(list_to_vec(&result), vec![120, 60, 180, 60, 300, 60, 420]);
}

#[test]
fn test_94() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[210, 231, 252, 273, 294]));
    assert_eq!(list_to_vec(&result), vec![210, 21, 231, 21, 252, 21, 273, 21, 294]);
}

#[test]
fn test_95() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[128, 64, 32, 16, 8, 4, 2, 1]));
    assert_eq!(list_to_vec(&result), vec![128, 64, 64, 32, 32, 16, 16, 8, 8, 4, 4, 2, 2, 1, 1]);
}

#[test]
fn test_96() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[13, 26, 39, 52, 65, 78]));
    assert_eq!(list_to_vec(&result), vec![13, 13, 26, 13, 39, 13, 52, 13, 65, 13, 78]);
}

#[test]
fn test_97() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[44, 55, 66, 77, 88, 99, 110]));
    assert_eq!(list_to_vec(&result), vec![44, 11, 55, 11, 66, 11, 77, 11, 88, 11, 99, 11, 110]);
}

#[test]
fn test_98() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[13, 13, 13, 13, 13]));
    assert_eq!(list_to_vec(&result), vec![13, 13, 13, 13, 13, 13, 13, 13, 13]);
}

#[test]
fn test_99() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[99, 33, 11, 55, 22]));
    assert_eq!(list_to_vec(&result), vec![99, 33, 33, 11, 11, 11, 55, 11, 22]);
}

#[test]
fn test_100() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[17, 34, 51, 68]));
    assert_eq!(list_to_vec(&result), vec![17, 17, 34, 17, 51, 17, 68]);
}

#[test]
fn test_101() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[17, 19, 23, 29, 31, 37, 41, 43]));
    assert_eq!(list_to_vec(&result), vec![17, 1, 19, 1, 23, 1, 29, 1, 31, 1, 37, 1, 41, 1, 43]);
}

#[test]
fn test_102() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[9, 27, 81, 243, 729]));
    assert_eq!(list_to_vec(&result), vec![9, 9, 27, 27, 81, 81, 243, 243, 729]);
}

#[test]
fn test_103() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[72, 48, 64, 32, 16]));
    assert_eq!(list_to_vec(&result), vec![72, 24, 48, 16, 64, 32, 32, 16, 16]);
}

#[test]
fn test_104() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[17, 34, 51, 68, 85]));
    assert_eq!(list_to_vec(&result), vec![17, 17, 34, 17, 51, 17, 68, 17, 85]);
}

#[test]
fn test_105() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[84, 56, 42, 28, 14]));
    assert_eq!(list_to_vec(&result), vec![84, 28, 56, 14, 42, 14, 28, 14, 14]);
}

#[test]
fn test_106() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[101, 202, 303, 404, 505]));
    assert_eq!(list_to_vec(&result), vec![101, 101, 202, 101, 303, 101, 404, 101, 505]);
}

#[test]
fn test_107() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[13, 26, 39, 52]));
    assert_eq!(list_to_vec(&result), vec![13, 13, 26, 13, 39, 13, 52]);
}

#[test]
fn test_108() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[99, 98, 97, 96, 95, 94, 93, 92, 91, 90]));
    assert_eq!(list_to_vec(&result), vec![99, 1, 98, 1, 97, 1, 96, 1, 95, 1, 94, 1, 93, 1, 92, 1, 91, 1, 90]);
}

#[test]
fn test_109() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[101, 103, 107, 109, 113]));
    assert_eq!(list_to_vec(&result), vec![101, 1, 103, 1, 107, 1, 109, 1, 113]);
}

#[test]
fn test_110() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[1000, 500, 250, 125, 62, 31]));
    assert_eq!(list_to_vec(&result), vec![1000, 500, 500, 250, 250, 125, 125, 1, 62, 31, 31]);
}

#[test]
fn test_111() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[144, 72, 36, 18, 9]));
    assert_eq!(list_to_vec(&result), vec![144, 72, 72, 36, 36, 18, 18, 9, 9]);
}

#[test]
fn test_112() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[99, 33, 11, 3]));
    assert_eq!(list_to_vec(&result), vec![99, 33, 33, 11, 11, 1, 3]);
}

#[test]
fn test_113() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[101, 103, 107, 109, 113, 127, 131, 137, 139]));
    assert_eq!(list_to_vec(&result), vec![101, 1, 103, 1, 107, 1, 109, 1, 113, 1, 127, 1, 131, 1, 137, 1, 139]);
}

#[test]
fn test_114() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[81, 27, 54, 108, 162]));
    assert_eq!(list_to_vec(&result), vec![81, 27, 27, 27, 54, 54, 108, 54, 162]);
}

#[test]
fn test_115() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[123, 246, 369, 492, 615]));
    assert_eq!(list_to_vec(&result), vec![123, 123, 246, 123, 369, 123, 492, 123, 615]);
}

#[test]
fn test_116() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[99, 77, 55, 33, 11]));
    assert_eq!(list_to_vec(&result), vec![99, 11, 77, 11, 55, 11, 33, 11, 11]);
}

#[test]
fn test_117() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[300, 150, 75, 375, 1875]));
    assert_eq!(list_to_vec(&result), vec![300, 150, 150, 75, 75, 75, 375, 375, 1875]);
}

#[test]
fn test_118() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[441, 147, 49, 7]));
    assert_eq!(list_to_vec(&result), vec![441, 147, 147, 49, 49, 7, 7]);
}

#[test]
fn test_119() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[81, 27, 9, 3, 1]));
    assert_eq!(list_to_vec(&result), vec![81, 27, 27, 9, 9, 3, 3, 1, 1]);
}

#[test]
fn test_120() {
    let result = Solution::insert_greatest_common_divisors(build_list(&[21, 28, 35, 42, 49, 56]));
    assert_eq!(list_to_vec(&result), vec![21, 7, 28, 7, 35, 7, 42, 7, 49, 7, 56]);
}
