include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::character_replacement("ABABABAB".to_string(), 3), 7);
}

#[test]
fn test_2() {
    assert_eq!(Solution::character_replacement("AAAAAAAAAAAAABBBAABBCCDDEE".to_string(), 5), 20);
}

#[test]
fn test_3() {
    assert_eq!(Solution::character_replacement("AABBCCDD".to_string(), 2), 4);
}

#[test]
fn test_4() {
    assert_eq!(Solution::character_replacement("".to_string(), 0), 0);
}

#[test]
fn test_5() {
    assert_eq!(Solution::character_replacement("A".to_string(), 1), 1);
}

#[test]
fn test_6() {
    assert_eq!(Solution::character_replacement("AABAABBBCCCC".to_string(), 3), 7);
}

#[test]
fn test_7() {
    assert_eq!(Solution::character_replacement("ABBBB".to_string(), 0), 4);
}

#[test]
fn test_8() {
    assert_eq!(Solution::character_replacement("ABCCDEEEEE".to_string(), 3), 8);
}

#[test]
fn test_9() {
    assert_eq!(Solution::character_replacement("ABAB".to_string(), 2), 4);
}

#[test]
fn test_10() {
    assert_eq!(Solution::character_replacement("AABBB".to_string(), 2), 5);
}

#[test]
fn test_11() {
    assert_eq!(Solution::character_replacement("ZYXWVUTSRQPONMLKJIHGFEDCBA".to_string(), 25), 26);
}

#[test]
fn test_12() {
    assert_eq!(Solution::character_replacement("ABACBCAB".to_string(), 2), 4);
}

#[test]
fn test_13() {
    assert_eq!(Solution::character_replacement("ABBB".to_string(), 0), 3);
}

#[test]
fn test_14() {
    assert_eq!(Solution::character_replacement("ABBB".to_string(), 1), 4);
}

#[test]
fn test_15() {
    assert_eq!(Solution::character_replacement("ABCDE".to_string(), 1), 2);
}

#[test]
fn test_16() {
    assert_eq!(Solution::character_replacement("AABABBA".to_string(), 1), 4);
}

#[test]
fn test_17() {
    assert_eq!(Solution::character_replacement("AAABBBCCC".to_string(), 3), 6);
}

#[test]
fn test_18() {
    assert_eq!(Solution::character_replacement("AAAA".to_string(), 2), 4);
}

#[test]
fn test_19() {
    assert_eq!(Solution::character_replacement("ABCDE".to_string(), 3), 4);
}

#[test]
fn test_20() {
    assert_eq!(Solution::character_replacement("ABCDE".to_string(), 2), 3);
}

#[test]
fn test_21() {
    assert_eq!(Solution::character_replacement("AABBBBCCDDDDD".to_string(), 2), 7);
}

#[test]
fn test_22() {
    assert_eq!(Solution::character_replacement("ACBACBACBACBACBACBACBACBACBACBAC".to_string(), 5), 8);
}

#[test]
fn test_23() {
    assert_eq!(Solution::character_replacement("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".to_string(), 0), 40);
}

#[test]
fn test_24() {
    assert_eq!(Solution::character_replacement("AAAAAAAAAABBBBBBBBBBCCCCCCCCCCDDDDDDDDDDD".to_string(), 15), 26);
}

#[test]
fn test_25() {
    assert_eq!(Solution::character_replacement("AAAAAAAAAA".to_string(), 5), 10);
}

#[test]
fn test_26() {
    assert_eq!(Solution::character_replacement("ABCABCABCABCABCABCABCABCABCABC".to_string(), 10), 16);
}

#[test]
fn test_27() {
    assert_eq!(Solution::character_replacement("AAAAAAAAAAAABBBBCCCC".to_string(), 10), 20);
}

#[test]
fn test_28() {
    assert_eq!(Solution::character_replacement("ABABABAB".to_string(), 1), 3);
}

#[test]
fn test_29() {
    assert_eq!(Solution::character_replacement("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(), 25), 27);
}

#[test]
fn test_30() {
    assert_eq!(Solution::character_replacement("AAABBBCCCDDDAAABBBCCCDDD".to_string(), 7), 10);
}

#[test]
fn test_31() {
    assert_eq!(Solution::character_replacement("QWERTYUIOPASDFGHJKLZXCVBNM".to_string(), 24), 25);
}

#[test]
fn test_32() {
    assert_eq!(Solution::character_replacement("AAAAAAAAAAAABBBCCCCCCAAAAAAAAAAA".to_string(), 6), 18);
}

#[test]
fn test_33() {
    assert_eq!(Solution::character_replacement("AAABBBAABBCCDDDDDDDEEFFGGHHIIJJJKKKLLLMMMNNN".to_string(), 25), 32);
}

#[test]
fn test_34() {
    assert_eq!(Solution::character_replacement("AAAAAAAAAABBBBBBBBBBBCCCCCCDDDDDDDDDDD".to_string(), 25), 36);
}

#[test]
fn test_35() {
    assert_eq!(Solution::character_replacement("MMMMNNNNOOOO".to_string(), 5), 9);
}

#[test]
fn test_36() {
    assert_eq!(Solution::character_replacement("AAAAAAAAABBB".to_string(), 2), 11);
}

#[test]
fn test_37() {
    assert_eq!(Solution::character_replacement("AABABABABAB".to_string(), 5), 11);
}

#[test]
fn test_38() {
    assert_eq!(Solution::character_replacement("ABABABABABABABABABABABABABABABABAB".to_string(), 15), 31);
}

#[test]
fn test_39() {
    assert_eq!(Solution::character_replacement("ABACABAACBACABCABACBACABCABACBACABCABACBACABCABACBACAB".to_string(), 20), 36);
}

#[test]
fn test_40() {
    assert_eq!(Solution::character_replacement("AAAAAAAAAABBBBBBBBCCCCCCCCCC".to_string(), 10), 20);
}

#[test]
fn test_41() {
    assert_eq!(Solution::character_replacement("ABCDEABCDEABCDEABCDEABCDE".to_string(), 10), 13);
}

#[test]
fn test_42() {
    assert_eq!(Solution::character_replacement("ABBCCCDDDDEEEEEFFFFFF".to_string(), 6), 12);
}

#[test]
fn test_43() {
    assert_eq!(Solution::character_replacement("ABABABABABABABABAB".to_string(), 10), 18);
}

#[test]
fn test_44() {
    assert_eq!(Solution::character_replacement("ABACABACABACABACABACABACABACABACABACABAC".to_string(), 15), 31);
}

#[test]
fn test_45() {
    assert_eq!(Solution::character_replacement("ABABABABABABABABABABABABABABABABABABABAB".to_string(), 15), 31);
}

#[test]
fn test_46() {
    assert_eq!(Solution::character_replacement("ABABABABABABABABABABABABABABABAB".to_string(), 0), 1);
}

#[test]
fn test_47() {
    assert_eq!(Solution::character_replacement("AABABABABABABABABABABABABABABABAB".to_string(), 5), 12);
}

#[test]
fn test_48() {
    assert_eq!(Solution::character_replacement("AABCABCABCABCABCABC".to_string(), 4), 8);
}

#[test]
fn test_49() {
    assert_eq!(Solution::character_replacement("BAAAAAAAAAABAAAAAAAAAAB".to_string(), 5), 23);
}

#[test]
fn test_50() {
    assert_eq!(Solution::character_replacement("AABBCCDDEEFFGG".to_string(), 3), 5);
}

#[test]
fn test_51() {
    assert_eq!(Solution::character_replacement("ACACACACACACAC".to_string(), 2), 5);
}

#[test]
fn test_52() {
    assert_eq!(Solution::character_replacement("AAABBBCCCDDDAAABBBCCCDDDAAABBBCCCDDD".to_string(), 20), 29);
}

#[test]
fn test_53() {
    assert_eq!(Solution::character_replacement("ABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABAB".to_string(), 10), 21);
}

#[test]
fn test_54() {
    assert_eq!(Solution::character_replacement("ABACABACABACABACABACABACABACABAC".to_string(), 5), 11);
}

#[test]
fn test_55() {
    assert_eq!(Solution::character_replacement("AAABBBCCCDDD".to_string(), 4), 7);
}

#[test]
fn test_56() {
    assert_eq!(Solution::character_replacement("XYZXYZXYZXYZXYZXYZXYZXYZ".to_string(), 15), 23);
}

#[test]
fn test_57() {
    assert_eq!(Solution::character_replacement("ABCDEABCDEABCDEABCDEABCDEABCDE".to_string(), 6), 8);
}

#[test]
fn test_58() {
    assert_eq!(Solution::character_replacement("ABBBABAABBBBBBBBAAABBB".to_string(), 5), 17);
}

#[test]
fn test_59() {
    assert_eq!(Solution::character_replacement("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".to_string(), 100), 60);
}

#[test]
fn test_60() {
    assert_eq!(Solution::character_replacement("AAAAAAAAAAABBBBBBBBBBB".to_string(), 10), 21);
}

#[test]
fn test_61() {
    assert_eq!(Solution::character_replacement("ABABABABAB".to_string(), 5), 10);
}

#[test]
fn test_62() {
    assert_eq!(Solution::character_replacement("AAABBCCDDEEFFF".to_string(), 4), 7);
}

#[test]
fn test_63() {
    assert_eq!(Solution::character_replacement("AAAAAAAAAAAAAAAAAAAAAAAB".to_string(), 1), 24);
}

#[test]
fn test_64() {
    assert_eq!(Solution::character_replacement("XYZXYZXYZXYZ".to_string(), 3), 5);
}

#[test]
fn test_65() {
    assert_eq!(Solution::character_replacement("ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(), 26), 26);
}

#[test]
fn test_66() {
    assert_eq!(Solution::character_replacement("ABABABABABABABAB".to_string(), 8), 16);
}

#[test]
fn test_67() {
    assert_eq!(Solution::character_replacement("AAAAAAAAAAAAAAAAAAAA".to_string(), 0), 20);
}

#[test]
fn test_68() {
    assert_eq!(Solution::character_replacement("AABBBCCCDDDDDEEEEFFFFFFGGGGGGHHHHHIIIJJJKKKLLLLMMMMNNNNOOOOPPPP".to_string(), 30), 36);
}

#[test]
fn test_69() {
    assert_eq!(Solution::character_replacement("BBBBBBBBBBBBBBBBBBBBBAAAAAAAAAAAAA".to_string(), 5), 26);
}

#[test]
fn test_70() {
    assert_eq!(Solution::character_replacement("AABABBAACCCDDDEEE".to_string(), 3), 8);
}

#[test]
fn test_71() {
    assert_eq!(Solution::character_replacement("BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB".to_string(), 0), 106);
}

#[test]
fn test_72() {
    assert_eq!(Solution::character_replacement("AAAAAAAABBBBBBBB".to_string(), 0), 8);
}

#[test]
fn test_73() {
    assert_eq!(Solution::character_replacement("ABABABABABABAB".to_string(), 7), 14);
}

#[test]
fn test_74() {
    assert_eq!(Solution::character_replacement("AAABBBCCCDDDEEEFFFGGGHHHIIIJJJKKKLLLMMMNNNOOOPPPQQQRRRSSSTTTUUUVVVWWWXXXXYYYYZZZZ".to_string(), 50), 54);
}

#[test]
fn test_75() {
    assert_eq!(Solution::character_replacement("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAB".to_string(), 50), 41);
}

#[test]
fn test_76() {
    assert_eq!(Solution::character_replacement("MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM".to_string(), 100), 44);
}

#[test]
fn test_77() {
    assert_eq!(Solution::character_replacement("ABCABCABCABCABCABCABCABC".to_string(), 15), 23);
}

#[test]
fn test_78() {
    assert_eq!(Solution::character_replacement("AABBBCCDDEE".to_string(), 2), 5);
}

#[test]
fn test_79() {
    assert_eq!(Solution::character_replacement("BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB".to_string(), 1000), 31);
}

#[test]
fn test_80() {
    assert_eq!(Solution::character_replacement("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".to_string(), 100), 108);
}

#[test]
fn test_81() {
    assert_eq!(Solution::character_replacement("AABBCCDDEEFFGGHHIIJJKKLLMMNNOOPPQQRRSSTTUUVVWWXXYYZZ".to_string(), 20), 22);
}

#[test]
fn test_82() {
    assert_eq!(Solution::character_replacement("AAABBBCCCDDD".to_string(), 3), 6);
}

#[test]
fn test_83() {
    assert_eq!(Solution::character_replacement("AAAAA".to_string(), 0), 5);
}

#[test]
fn test_84() {
    assert_eq!(Solution::character_replacement("AABBBCCCDDDD".to_string(), 3), 7);
}

#[test]
fn test_85() {
    assert_eq!(Solution::character_replacement("AAABBBCCCDDDEEEFFF".to_string(), 6), 9);
}

#[test]
fn test_86() {
    assert_eq!(Solution::character_replacement("AAAAAAAAABBBBBBBBCCCCCCCCCC".to_string(), 10), 20);
}

#[test]
fn test_87() {
    assert_eq!(Solution::character_replacement("BBBAAAABBBAAAABBBAAAABBB".to_string(), 5), 13);
}

#[test]
fn test_88() {
    assert_eq!(Solution::character_replacement("ABABABABABABABABABAB".to_string(), 5), 11);
}

#[test]
fn test_89() {
    assert_eq!(Solution::character_replacement("ACACACACACACACACACACACAC".to_string(), 5), 11);
}

#[test]
fn test_90() {
    assert_eq!(Solution::character_replacement("ABACABACABAC".to_string(), 4), 9);
}

#[test]
fn test_91() {
    assert_eq!(Solution::character_replacement("ABABABABABABABABABAB".to_string(), 10), 20);
}

#[test]
fn test_92() {
    assert_eq!(Solution::character_replacement("AAAAAAABBBCCCDDDDDD".to_string(), 5), 12);
}

#[test]
fn test_93() {
    assert_eq!(Solution::character_replacement("BBBBAAAACCCD".to_string(), 4), 8);
}

#[test]
fn test_94() {
    assert_eq!(Solution::character_replacement("QWERTYUIOPASDFGHJKLZXCVBNM".to_string(), 26), 26);
}

#[test]
fn test_95() {
    assert_eq!(Solution::character_replacement("XYZXYZXYZXYZXYZXYZ".to_string(), 6), 10);
}

#[test]
fn test_96() {
    assert_eq!(Solution::character_replacement("AAAAAAAAAAABBBBCCCCCDDDD".to_string(), 10), 21);
}

#[test]
fn test_97() {
    assert_eq!(Solution::character_replacement("BBBBAAAACCCCDDDD".to_string(), 5), 9);
}

#[test]
fn test_98() {
    assert_eq!(Solution::character_replacement("XYZXYZXYZXYZXYZXYZXYZXYZ".to_string(), 20), 24);
}

#[test]
fn test_99() {
    assert_eq!(Solution::character_replacement("AAAAAAAABBBBBBBB".to_string(), 3), 11);
}

#[test]
fn test_100() {
    assert_eq!(Solution::character_replacement("AAAAAAAAAAAAAAAABBB".to_string(), 2), 18);
}

#[test]
fn test_101() {
    assert_eq!(Solution::character_replacement("XYZXYZXYZ".to_string(), 3), 5);
}

#[test]
fn test_102() {
    assert_eq!(Solution::character_replacement("AABBCDEEFGHIJKLLMNOPQRSTUVWXYZ".to_string(), 25), 27);
}

#[test]
fn test_103() {
    assert_eq!(Solution::character_replacement("AABABBAABBCCDDEEFFGG".to_string(), 4), 9);
}

#[test]
fn test_104() {
    assert_eq!(Solution::character_replacement("AABBCCDDEEFFGGHHIIJJKKLLMMNNOOPPQQRRSSTTUUVVWWXXYYZZ".to_string(), 25), 27);
}

#[test]
fn test_105() {
    assert_eq!(Solution::character_replacement("AAABBBCCCDDD".to_string(), 6), 9);
}
