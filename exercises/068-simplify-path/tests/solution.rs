include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::simplify_path("/a/../../b/../c///.//".to_string()), "/c".to_string());
}

#[test]
fn test_2() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/../../..".to_string()), "/a/b/c/d".to_string());
}

#[test]
fn test_3() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/../../..//f/".to_string()), "/a/b/f".to_string());
}

#[test]
fn test_4() {
    assert_eq!(Solution::simplify_path("/a/b/c/../".to_string()), "/a/b".to_string());
}

#[test]
fn test_5() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/../../../..///f/".to_string()), "/a/f".to_string());
}

#[test]
fn test_6() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/../../../".to_string()), "/a/b".to_string());
}

#[test]
fn test_7() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/".to_string()), "/a/b/c/d".to_string());
}

#[test]
fn test_8() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/../../../../f/".to_string()), "/a/f".to_string());
}

#[test]
fn test_9() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/../../..".to_string()), "/a".to_string());
}

#[test]
fn test_10() {
    assert_eq!(Solution::simplify_path("/a//b////c/d//././/..".to_string()), "/a/b/c".to_string());
}

#[test]
fn test_11() {
    assert_eq!(Solution::simplify_path("/home/".to_string()), "/home".to_string());
}

#[test]
fn test_12() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/./../../f/".to_string()), "/a/b/c/f".to_string());
}

#[test]
fn test_13() {
    assert_eq!(Solution::simplify_path("/a/..".to_string()), "/".to_string());
}

#[test]
fn test_14() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/..".to_string()), "/a/b/c/d/e/f".to_string());
}

#[test]
fn test_15() {
    assert_eq!(Solution::simplify_path("/a/b/c/../../d/e/".to_string()), "/a/d/e".to_string());
}

#[test]
fn test_16() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/..".to_string()), "/a/b/c/d/e".to_string());
}

#[test]
fn test_17() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/../..".to_string()), "/a/b/c/d/e".to_string());
}

#[test]
fn test_18() {
    assert_eq!(Solution::simplify_path("/a/b/c/../../".to_string()), "/a".to_string());
}

#[test]
fn test_19() {
    assert_eq!(Solution::simplify_path("/a/./b/../c/../../d/../../../e/../../../../f/".to_string()), "/f".to_string());
}

#[test]
fn test_20() {
    assert_eq!(Solution::simplify_path("/home/user/Documents/../Pictures".to_string()), "/home/user/Pictures".to_string());
}

#[test]
fn test_21() {
    assert_eq!(Solution::simplify_path("/a/./b/../../c/".to_string()), "/c".to_string());
}

#[test]
fn test_22() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z".to_string());
}

#[test]
fn test_23() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g".to_string()), "/a/b/c/d/e/f/g".to_string());
}

#[test]
fn test_24() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/../../..///f/".to_string()), "/a/b/f".to_string());
}

#[test]
fn test_25() {
    assert_eq!(Solution::simplify_path("/a/./b/./c/./d/".to_string()), "/a/b/c/d".to_string());
}

#[test]
fn test_26() {
    assert_eq!(Solution::simplify_path("/home//foo/".to_string()), "/home/foo".to_string());
}

#[test]
fn test_27() {
    assert_eq!(Solution::simplify_path("/a/./b/./c/./".to_string()), "/a/b/c".to_string());
}

#[test]
fn test_28() {
    assert_eq!(Solution::simplify_path("/a/b/c/./././".to_string()), "/a/b/c".to_string());
}

#[test]
fn test_29() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/../../../..//f/".to_string()), "/a/f".to_string());
}

#[test]
fn test_30() {
    assert_eq!(Solution::simplify_path("/../".to_string()), "/".to_string());
}

#[test]
fn test_31() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../.././..".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w".to_string());
}

#[test]
fn test_32() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../..//".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s".to_string());
}

#[test]
fn test_33() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../../../../../../../../../../../../../../../../../../../../../".to_string()), "/".to_string());
}

#[test]
fn test_34() {
    assert_eq!(Solution::simplify_path("/..////../home/user/Documents".to_string()), "/home/user/Documents".to_string());
}

#[test]
fn test_35() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/./e/../../f/g/".to_string()), "/a/b/c/f/g".to_string());
}

#[test]
fn test_36() {
    assert_eq!(Solution::simplify_path("/valid/directory/name/./with/./multiple/./dots/./in/./it".to_string()), "/valid/directory/name/with/multiple/dots/in/it".to_string());
}

#[test]
fn test_37() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../../../..".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m".to_string());
}

#[test]
fn test_38() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../..////".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s".to_string());
}

#[test]
fn test_39() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../../../../../../../..".to_string()), "/a/b/c/d/e/f/g/h/i".to_string());
}

#[test]
fn test_40() {
    assert_eq!(Solution::simplify_path("/home/user/../../../var/log/../../usr/local/bin/".to_string()), "/usr/local/bin".to_string());
}

#[test]
fn test_41() {
    assert_eq!(Solution::simplify_path("/home/user/./././Documents/".to_string()), "/home/user/Documents".to_string());
}

#[test]
fn test_42() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../../".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o".to_string());
}

#[test]
fn test_43() {
    assert_eq!(Solution::simplify_path("/home/user/./Documents/./../Pictures/./".to_string()), "/home/user/Pictures".to_string());
}

#[test]
fn test_44() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../..".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o".to_string());
}

#[test]
fn test_45() {
    assert_eq!(Solution::simplify_path("/start/..//end".to_string()), "/end".to_string());
}

#[test]
fn test_46() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/..///./../../".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w".to_string());
}

#[test]
fn test_47() {
    assert_eq!(Solution::simplify_path("/a/b/c/././../d/e/f/..".to_string()), "/a/b/d/e".to_string());
}

#[test]
fn test_48() {
    assert_eq!(Solution::simplify_path("/a/./b/./c/./d/./e/./f/./g/./h/./i/./j/./k/./l/./m/./n/./o/./p/./q/./r/./s/./t/./u/./v/./w/./x/./y/./z/..".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y".to_string());
}

#[test]
fn test_49() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../.././..".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u".to_string());
}

#[test]
fn test_50() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../..///".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s".to_string());
}

#[test]
fn test_51() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../..//////".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s".to_string());
}

#[test]
fn test_52() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../../../../".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m".to_string());
}

#[test]
fn test_53() {
    assert_eq!(Solution::simplify_path("/one/two/three/../../../four/five/six/../../seven".to_string()), "/four/seven".to_string());
}

#[test]
fn test_54() {
    assert_eq!(Solution::simplify_path("/home/user/./Documents/./../Pictures/./../Videos/./../Music/./../Downloads/./../Documents/./../Pictures/./../Videos".to_string()), "/home/user/Videos".to_string());
}

#[test]
fn test_55() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p".to_string());
}

#[test]
fn test_56() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../.././..".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s".to_string());
}

#[test]
fn test_57() {
    assert_eq!(Solution::simplify_path("/a/./b/./c/./d/./e/./f/./g/./h/./i/./j/./k/./l/./m/./n/./o/./p/./q/./r/./s/./t/./u/./v/./w/./x/./y/./z".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z".to_string());
}

#[test]
fn test_58() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../..".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u".to_string());
}

#[test]
fn test_59() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r".to_string());
}

#[test]
fn test_60() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../../../../../../../../../../../../../../../../../../../..".to_string()), "/".to_string());
}

#[test]
fn test_61() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/../../f/g/./h/./i/./j/./k/./l/./m/./n/./o/./p/./q/./r/./s/./t/./u/./v/./w/./x/./y/./z/././".to_string()), "/a/b/c/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z".to_string());
}

#[test]
fn test_62() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../a/b/c/d".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/a/b/c/d".to_string());
}

#[test]
fn test_63() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/../../f/g/./h/./i/./j/./k/./l/./m/./n/./o/./p/./q/./r/./s/./t/./u/./v/./w/./x/./y/./z/./".to_string()), "/a/b/c/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z".to_string());
}

#[test]
fn test_64() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/../../../../e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z".to_string()), "/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z".to_string());
}

#[test]
fn test_65() {
    assert_eq!(Solution::simplify_path("/home/user/./Documents/./../Pictures/./../Videos/./../Music/./../Downloads/./../".to_string()), "/home/user".to_string());
}

#[test]
fn test_66() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/..///././././..///".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x".to_string());
}

#[test]
fn test_67() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/../../../..//../m/n/o/p/".to_string()), "/a/b/m/n/o/p".to_string());
}

#[test]
fn test_68() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z".to_string());
}

#[test]
fn test_69() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../..".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q".to_string());
}

#[test]
fn test_70() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../..".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w".to_string());
}

#[test]
fn test_71() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../..///u/".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u".to_string());
}

#[test]
fn test_72() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../../../../../../../../../..".to_string()), "/a/b/c/d/e/f/g".to_string());
}

#[test]
fn test_73() {
    assert_eq!(Solution::simplify_path("/a/./b/./c/./d/./e".to_string()), "/a/b/c/d/e".to_string());
}

#[test]
fn test_74() {
    assert_eq!(Solution::simplify_path("/many/../../dots/../../../in/../../../../path/../../../../../root".to_string()), "/root".to_string());
}

#[test]
fn test_75() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../../../../../../../../../../../../../../../../../../../../../..".to_string()), "/".to_string());
}

#[test]
fn test_76() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/./f/g/././h/./i/./j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z".to_string());
}

#[test]
fn test_77() {
    assert_eq!(Solution::simplify_path("/home////user/..///Documents///".to_string()), "/home/Documents".to_string());
}

#[test]
fn test_78() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/../../../../../../".to_string()), "/a".to_string());
}

#[test]
fn test_79() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/../../.././../../".to_string()), "/a/b".to_string());
}

#[test]
fn test_80() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/../../f/g/./h/./i/./j/./k/./l/./m/./n/./o/./p/./q/./r/./s/./t/./u/./v/./w/./x/./y/./z/../../".to_string()), "/a/b/c/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x".to_string());
}

#[test]
fn test_81() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z".to_string());
}

#[test]
fn test_82() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../..".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t".to_string());
}

#[test]
fn test_83() {
    assert_eq!(Solution::simplify_path("/home/user/./Documents/./../Pictures/./../Videos/./../Music/./../Downloads/./../Documents/./../".to_string()), "/home/user".to_string());
}

#[test]
fn test_84() {
    assert_eq!(Solution::simplify_path("/x/y/z/../../../../w/x/y".to_string()), "/w/x/y".to_string());
}

#[test]
fn test_85() {
    assert_eq!(Solution::simplify_path("/home/user/./Documents/./../Pictures/./../Videos/./../Music/./../Downloads/./../Documents/./../Pictures".to_string()), "/home/user/Pictures".to_string());
}

#[test]
fn test_86() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../../../../../../../../../../../../../".to_string()), "/a/b/c/d".to_string());
}

#[test]
fn test_87() {
    assert_eq!(Solution::simplify_path("/x/y/z/../../w/../../v/..///u/".to_string()), "/u".to_string());
}

#[test]
fn test_88() {
    assert_eq!(Solution::simplify_path("/home/user/././././././././././././././././././././././././././././././././".to_string()), "/home/user".to_string());
}

#[test]
fn test_89() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u".to_string());
}

#[test]
fn test_90() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p".to_string());
}

#[test]
fn test_91() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../..".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s".to_string());
}

#[test]
fn test_92() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/../../f/g/../../h/i/../../j/k/../../l/m/../../n/o/p/../../q/r/s/t/../../u/v/w/x/../../y/z".to_string()), "/a/b/c/n/q/r/u/v/y/z".to_string());
}

#[test]
fn test_93() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../../../../../".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l".to_string());
}

#[test]
fn test_94() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/././././..".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y".to_string());
}

#[test]
fn test_95() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p".to_string());
}

#[test]
fn test_96() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../../../../../../../../../../../../../..".to_string()), "/a/b/c".to_string());
}

#[test]
fn test_97() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z".to_string());
}

#[test]
fn test_98() {
    assert_eq!(Solution::simplify_path("/nested/../../deep/../../..".to_string()), "/".to_string());
}

#[test]
fn test_99() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t".to_string());
}

#[test]
fn test_100() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/././././..//..".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x".to_string());
}

#[test]
fn test_101() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/./.././..".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x".to_string());
}

#[test]
fn test_102() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v".to_string());
}

#[test]
fn test_103() {
    assert_eq!(Solution::simplify_path("/usr/local/../../local/bin/../../../bin".to_string()), "/bin".to_string());
}

#[test]
fn test_104() {
    assert_eq!(Solution::simplify_path("/a/b/c/../../d/e/../../f/g/h/../../../i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z".to_string()), "/a/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z".to_string());
}

#[test]
fn test_105() {
    assert_eq!(Solution::simplify_path("/var/log/../log/./error.log".to_string()), "/var/log/error.log".to_string());
}

#[test]
fn test_106() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/..".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y".to_string());
}

#[test]
fn test_107() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/./.././h/i/j/k/l/..".to_string()), "/a/b/c/d/e/f/h/i/j/k".to_string());
}

#[test]
fn test_108() {
    assert_eq!(Solution::simplify_path("/multiple/////slashes/are/here/./../still/reduced".to_string()), "/multiple/slashes/are/still/reduced".to_string());
}

#[test]
fn test_109() {
    assert_eq!(Solution::simplify_path("/foo/./bar/./baz/../qux/../.././quux".to_string()), "/foo/quux".to_string());
}

#[test]
fn test_110() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/./././././././././././././././././././././././././././././././././.././././././././././././././././././././././././././././././././././././././././.".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y".to_string());
}

#[test]
fn test_111() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../a/b/c/d/./".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/a/b/c/d".to_string());
}

#[test]
fn test_112() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../../../../../../../../../../../../../../../".to_string()), "/a/b".to_string());
}

#[test]
fn test_113() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../../../../../../../".to_string()), "/a/b/c/d/e/f/g/h/i/j".to_string());
}

#[test]
fn test_114() {
    assert_eq!(Solution::simplify_path("/home/user/./Documents/./../Pictures/./../Videos/./../Music/./../Downloads/./../Documents/./../Pictures/./../Videos/./../Music/./../Downloads".to_string()), "/home/user/Downloads".to_string());
}

#[test]
fn test_115() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/./i/../../j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z".to_string()), "/a/b/c/d/e/f/g/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z".to_string());
}

#[test]
fn test_116() {
    assert_eq!(Solution::simplify_path("/a/b/c/././././././d/e/f/../g/h/i/j/k/l/m/n/o/p/../../../q/r/s/t/u/v/w/x/y/z".to_string()), "/a/b/c/d/e/g/h/i/j/k/l/m/q/r/s/t/u/v/w/x/y/z".to_string());
}

#[test]
fn test_117() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/../../f/g/././h/./i/./j/./k/./l/./m/./n/./o/./p/./q/./r/./s/./t/./u/./v/./w/./x/./y/./z".to_string()), "/a/b/c/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z".to_string());
}

#[test]
fn test_118() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w".to_string());
}

#[test]
fn test_119() {
    assert_eq!(Solution::simplify_path("/home/user/../../../../../../".to_string()), "/".to_string());
}

#[test]
fn test_120() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p".to_string());
}

#[test]
fn test_121() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/./.././.././../../".to_string()), "/a/b/c".to_string());
}

#[test]
fn test_122() {
    assert_eq!(Solution::simplify_path("/a/b/c/../../d/e/../../f/g/h/./../".to_string()), "/a/f/g".to_string());
}

#[test]
fn test_123() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../../../../../../../../../../../../../../../../../".to_string()), "/".to_string());
}

#[test]
fn test_124() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/..////..////..".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w".to_string());
}

#[test]
fn test_125() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../a/b/c/d/../".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/a/b/c".to_string());
}

#[test]
fn test_126() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../../../../..".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l".to_string());
}

#[test]
fn test_127() {
    assert_eq!(Solution::simplify_path("/x/y/z/../../../../../../../../../".to_string()), "/".to_string());
}

#[test]
fn test_128() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../../../../../../../../../../../../../../../../../..".to_string()), "/".to_string());
}

#[test]
fn test_129() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../../../../../../../../../../../../../../../../../../../".to_string()), "/".to_string());
}

#[test]
fn test_130() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/../../f/g/./h/./i/./j/./k/./l/./m/./n/./o/./p/./q/./r/./s/./t/./u/./v/./w/./x/./y/./z/../../../".to_string()), "/a/b/c/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w".to_string());
}

#[test]
fn test_131() {
    assert_eq!(Solution::simplify_path("/trailing/./slashes/./should/./be/./removed".to_string()), "/trailing/slashes/should/be/removed".to_string());
}

#[test]
fn test_132() {
    assert_eq!(Solution::simplify_path("/home/user/../../root///".to_string()), "/root".to_string());
}

#[test]
fn test_133() {
    assert_eq!(Solution::simplify_path("/a/b/c/./../../d/e/./f/../g".to_string()), "/a/d/e/g".to_string());
}

#[test]
fn test_134() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../../../../../../../../../".to_string()), "/a/b/c/d/e/f/g/h".to_string());
}

#[test]
fn test_135() {
    assert_eq!(Solution::simplify_path("/home/user/./Documents/../Pictures/./Vacation/../../Pictures".to_string()), "/home/user/Pictures".to_string());
}

#[test]
fn test_136() {
    assert_eq!(Solution::simplify_path("/x/y/z/./././././././".to_string()), "/x/y/z".to_string());
}

#[test]
fn test_137() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/../../f/g/./h/./i/./j/./k/./l/./m/./n/./o/./p/./q/./r/./s/./t/./u/./v/./w/./x/./y/./z/../".to_string()), "/a/b/c/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y".to_string());
}

#[test]
fn test_138() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/././././e/f/g/../././h".to_string()), "/a/b/c/d/e/f/h".to_string());
}

#[test]
fn test_139() {
    assert_eq!(Solution::simplify_path("/home/user/./Documents/./../Pictures/./../Videos/./../Music/./../Downloads/./../Documents/./../Pictures/./../Videos/./../Music".to_string()), "/home/user/Music".to_string());
}

#[test]
fn test_140() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../../../../../../../../../../../..".to_string()), "/a/b/c/d/e".to_string());
}

#[test]
fn test_141() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../..////////".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s".to_string());
}

#[test]
fn test_142() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../../../".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n".to_string());
}

#[test]
fn test_143() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z".to_string());
}

#[test]
fn test_144() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../../../../../../../../../../../".to_string()), "/a/b/c/d/e/f".to_string());
}

#[test]
fn test_145() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../../../..///..".to_string()), "/a/b/c/d/e/f/g/h/i/j/k/l".to_string());
}

#[test]
fn test_146() {
    assert_eq!(Solution::simplify_path("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/../../../../../../../../../../../../../../../../../../../../../../../../..".to_string()), "/a".to_string());
}
