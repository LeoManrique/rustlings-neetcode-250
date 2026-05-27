include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::partition_labels("abcdabcde".to_string()), vec![8, 1]);
}

#[test]
fn test_2() {
    assert_eq!(Solution::partition_labels("aaaaaabbbbbccccc".to_string()), vec![6, 5, 5]);
}

#[test]
fn test_3() {
    assert_eq!(Solution::partition_labels("xyzxyzxyz".to_string()), vec![9]);
}

#[test]
fn test_4() {
    assert_eq!(Solution::partition_labels("aaaabbbbbccccc".to_string()), vec![4, 5, 5]);
}

#[test]
fn test_5() {
    assert_eq!(Solution::partition_labels("abcdabcdeabcdabcde".to_string()), vec![18]);
}

#[test]
fn test_6() {
    assert_eq!(Solution::partition_labels("abcdefghijklmnopqrstuvwxyz".to_string()), vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_7() {
    assert_eq!(Solution::partition_labels("aabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyzz".to_string()), vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]);
}

#[test]
fn test_8() {
    assert_eq!(Solution::partition_labels("zab".to_string()), vec![1, 1, 1]);
}

#[test]
fn test_9() {
    assert_eq!(Solution::partition_labels("a".to_string()), vec![1]);
}

#[test]
fn test_10() {
    assert_eq!(Solution::partition_labels("zabbcdefghijklmnopqrstuvwxyz".to_string()), vec![28]);
}

#[test]
fn test_11() {
    assert_eq!(Solution::partition_labels("aabbccddeee".to_string()), vec![2, 2, 2, 2, 3]);
}

#[test]
fn test_12() {
    assert_eq!(Solution::partition_labels("abcdcba".to_string()), vec![7]);
}

#[test]
fn test_13() {
    assert_eq!(Solution::partition_labels("ababababab".to_string()), vec![10]);
}

#[test]
fn test_14() {
    assert_eq!(Solution::partition_labels("zabzabc".to_string()), vec![6, 1]);
}

#[test]
fn test_15() {
    assert_eq!(Solution::partition_labels("abababab".to_string()), vec![8]);
}

#[test]
fn test_16() {
    assert_eq!(Solution::partition_labels("abcde".to_string()), vec![1, 1, 1, 1, 1]);
}

#[test]
fn test_17() {
    assert_eq!(Solution::partition_labels("ababcbacadefegdehijhklij".to_string()), vec![9, 7, 8]);
}

#[test]
fn test_18() {
    assert_eq!(Solution::partition_labels("eccbbbbdec".to_string()), vec![10]);
}

#[test]
fn test_19() {
    assert_eq!(Solution::partition_labels("abcabcdabcdeabcdefabcdefg".to_string()), vec![24, 1]);
}

#[test]
fn test_20() {
    assert_eq!(Solution::partition_labels("xyzzyxzyxzyxzyxzyxzyxzyxzyxzyxzyxzyxzyxzyxzyxzyxzyxzyxzyxzyxzyxzyx".to_string()), vec![66]);
}

#[test]
fn test_21() {
    assert_eq!(Solution::partition_labels("aaaaaaaaaabbbbbbbbccccccccddddddddeeeeeeeefffffffffghhhhhhhhiiiiiiiijjjjjjjjkkkkkkkkllllllllmmmmmmmmnnnnnnnnooooooooppppppppqqqqqqqqrrrrrrrrssssssssttttttttuuuuuuuuvvvvvvvvwwwwwwwwxxxxxxxxxyyyyyyyyzzzzzzzz".to_string()), vec![10, 8, 8, 8, 8, 9, 1, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 9, 8, 8]);
}

#[test]
fn test_22() {
    assert_eq!(Solution::partition_labels("abcdabcdeabcde".to_string()), vec![14]);
}

#[test]
fn test_23() {
    assert_eq!(Solution::partition_labels("aaaabbbbccccddddeeeeffffgggg".to_string()), vec![4, 4, 4, 4, 4, 4, 4]);
}

#[test]
fn test_24() {
    assert_eq!(Solution::partition_labels("abcadefeghijklmnopqrstuvwxyzzxywvutsrqponmlkjihgfedcba".to_string()), vec![54]);
}

#[test]
fn test_25() {
    assert_eq!(Solution::partition_labels("xyzzyxzyxzzyxzyxzyxzzyxzyxzyxzzyxzyxzzyxzyxzyxzzyxzyxzyxzzyxzyxzyxzyxzzyxzyxzyxzyx".to_string()), vec![82]);
}

#[test]
fn test_26() {
    assert_eq!(Solution::partition_labels("zzyzxzyzxzyz".to_string()), vec![12]);
}

#[test]
fn test_27() {
    assert_eq!(Solution::partition_labels("abcabcabcabcabcabcabcabcabcabc".to_string()), vec![30]);
}

#[test]
fn test_28() {
    assert_eq!(Solution::partition_labels("abacabadabacabadabacabadabacabad".to_string()), vec![32]);
}

#[test]
fn test_29() {
    assert_eq!(Solution::partition_labels("abcdexyzabcdexyzabcdexyzabcdexyz".to_string()), vec![32]);
}

#[test]
fn test_30() {
    assert_eq!(Solution::partition_labels("abcdefghfedcba".to_string()), vec![14]);
}

#[test]
fn test_31() {
    assert_eq!(Solution::partition_labels("abcdefghihgfedcbaijklmnopqrstuvutsrqponmlkjihgfedcba".to_string()), vec![52]);
}

#[test]
fn test_32() {
    assert_eq!(Solution::partition_labels("abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij".to_string()), vec![80]);
}

#[test]
fn test_33() {
    assert_eq!(Solution::partition_labels("aabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyzzzyxwvutsrqponmlkjihgfedcba".to_string()), vec![78]);
}

#[test]
fn test_34() {
    assert_eq!(Solution::partition_labels("mnopqrstuvwxyzzyxwvutsrqponmlkjihgfedcbazyxwvutsrqponmlkjihgfedcba".to_string()), vec![66]);
}

#[test]
fn test_35() {
    assert_eq!(Solution::partition_labels("ababcbacadeafgafghijghijklmnopqrstuvwxyzzyxwvutsrqponmlkjihgfedcba".to_string()), vec![66]);
}

#[test]
fn test_36() {
    assert_eq!(Solution::partition_labels("zzzyyxxwwvvuuttssrrqqppoonnmmllkkjjiihhggffeeddccbaaazzzyyxxwwvvuuttssrrqqppoonnmmllkkjjiihhggffeeddccbaaazzzyyxxwwvvuuttssrrqqppoonnmmllkkjjiihhggffeeddccbaaazzzyyxxwwvvuuttssrrqqppoonnmmllkkjjiihhggffeeddccbaaa".to_string()), vec![212]);
}

#[test]
fn test_37() {
    assert_eq!(Solution::partition_labels("abcdefgabcdefgabcdefg".to_string()), vec![21]);
}

#[test]
fn test_38() {
    assert_eq!(Solution::partition_labels("zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz".to_string()), vec![70]);
}

#[test]
fn test_39() {
    assert_eq!(Solution::partition_labels("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string()), vec![62]);
}

#[test]
fn test_40() {
    assert_eq!(Solution::partition_labels("abcdefghihgfedcba".to_string()), vec![17]);
}

#[test]
fn test_41() {
    assert_eq!(Solution::partition_labels("abcdefgabcdefgabcdefgabcdefgabcdefgabcdefgabcdefgabcdefgabcdefg".to_string()), vec![63]);
}

#[test]
fn test_42() {
    assert_eq!(Solution::partition_labels("abcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc".to_string()), vec![66]);
}

#[test]
fn test_43() {
    assert_eq!(Solution::partition_labels("aabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz".to_string()), vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 36]);
}

#[test]
fn test_44() {
    assert_eq!(Solution::partition_labels("abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz".to_string()), vec![52]);
}

#[test]
fn test_45() {
    assert_eq!(Solution::partition_labels("xyzxyzxyzxyzxyzxyzxyzxyzxyzxyz".to_string()), vec![30]);
}

#[test]
fn test_46() {
    assert_eq!(Solution::partition_labels("mnopqrsmnopqrstuvwpqrstuv".to_string()), vec![25]);
}

#[test]
fn test_47() {
    assert_eq!(Solution::partition_labels("abcabcabcabcabcabc".to_string()), vec![18]);
}

#[test]
fn test_48() {
    assert_eq!(Solution::partition_labels("lkjihgfedcbaedcba".to_string()), vec![1, 1, 1, 1, 1, 1, 1, 10]);
}

#[test]
fn test_49() {
    assert_eq!(Solution::partition_labels("abcdefghijkabcdefghijk".to_string()), vec![22]);
}

#[test]
fn test_50() {
    assert_eq!(Solution::partition_labels("zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz".to_string()), vec![52]);
}

#[test]
fn test_51() {
    assert_eq!(Solution::partition_labels("abcdefedcbafedcba".to_string()), vec![17]);
}

#[test]
fn test_52() {
    assert_eq!(Solution::partition_labels("aaaaabbbbccccdddddeeeeefffffggggghhhhiiiiijjjjkkkkllllmmmmnnnnoooo".to_string()), vec![5, 4, 4, 5, 5, 5, 5, 4, 5, 4, 4, 4, 4, 4, 4]);
}

#[test]
fn test_53() {
    assert_eq!(Solution::partition_labels("mnopqrstuvwxyzzyxwvutsrqponmlkjihgfedcba".to_string()), vec![28, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_54() {
    assert_eq!(Solution::partition_labels("abcdefghijklmnopqrstuvwxyzzyxwvutsrqponmlkjihgfedcbaabcdefghijklmnopqrstuvwxyz".to_string()), vec![78]);
}

#[test]
fn test_55() {
    assert_eq!(Solution::partition_labels("zabacabadefegdehijhklijkmnopqrstuvwxyzyxwvutsrqponmlkjihgfedcba".to_string()), vec![63]);
}

#[test]
fn test_56() {
    assert_eq!(Solution::partition_labels("mnopqrstuabcrstuvwxyzzyxwvutsrqponmlkjihgfedcba".to_string()), vec![47]);
}

#[test]
fn test_57() {
    assert_eq!(Solution::partition_labels("aabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyzzzzzzzzzzzzzzzzzz".to_string()), vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 18]);
}

#[test]
fn test_58() {
    assert_eq!(Solution::partition_labels("xyzxyzxyzxyz".to_string()), vec![12]);
}

#[test]
fn test_59() {
    assert_eq!(Solution::partition_labels("abcdefghijklmnopqrstuvwxyzyxwvutsrqponmlkjihgfedcba".to_string()), vec![51]);
}

#[test]
fn test_60() {
    assert_eq!(Solution::partition_labels("abacbacbacbacbacbacbacbacbacbacbacbacbacbacbacbacbacbacbacbacbac".to_string()), vec![64]);
}

#[test]
fn test_61() {
    assert_eq!(Solution::partition_labels("aabababababcabcabcabcdabcdabcdabcdeabcdeabcdefabcdef".to_string()), vec![52]);
}

#[test]
fn test_62() {
    assert_eq!(Solution::partition_labels("zababzabz".to_string()), vec![9]);
}

#[test]
fn test_63() {
    assert_eq!(Solution::partition_labels("aabbccddeeffaabbccddeeff".to_string()), vec![24]);
}

#[test]
fn test_64() {
    assert_eq!(Solution::partition_labels("aaaabbbbccccddddeeeeffffgggghhhh".to_string()), vec![4, 4, 4, 4, 4, 4, 4, 4]);
}
