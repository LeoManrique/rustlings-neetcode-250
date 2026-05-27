include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::predict_party_victory("RDDRDR".to_string()), "Dire".to_string());
}

#[test]
fn test_2() {
    assert_eq!(Solution::predict_party_victory("RDD".to_string()), "Dire".to_string());
}

#[test]
fn test_3() {
    assert_eq!(Solution::predict_party_victory("DDDARR".to_string()), "Dire".to_string());
}

#[test]
fn test_4() {
    assert_eq!(Solution::predict_party_victory("DRDR".to_string()), "Dire".to_string());
}

#[test]
fn test_5() {
    assert_eq!(Solution::predict_party_victory("RDDRRD".to_string()), "Radiant".to_string());
}

#[test]
fn test_6() {
    assert_eq!(Solution::predict_party_victory("RDRDRDRDRDRDRD".to_string()), "Radiant".to_string());
}

#[test]
fn test_7() {
    assert_eq!(Solution::predict_party_victory("RD".to_string()), "Radiant".to_string());
}

#[test]
fn test_8() {
    assert_eq!(Solution::predict_party_victory("DRDRDR".to_string()), "Dire".to_string());
}

#[test]
fn test_9() {
    assert_eq!(Solution::predict_party_victory("RRRDDD".to_string()), "Radiant".to_string());
}

#[test]
fn test_10() {
    assert_eq!(Solution::predict_party_victory("DDDD".to_string()), "Dire".to_string());
}

#[test]
fn test_11() {
    assert_eq!(Solution::predict_party_victory("RRDDRR".to_string()), "Radiant".to_string());
}

#[test]
fn test_12() {
    assert_eq!(Solution::predict_party_victory("DDDDRRRRRDDDDRRRR".to_string()), "Dire".to_string());
}

#[test]
fn test_13() {
    assert_eq!(Solution::predict_party_victory("RRDDDD".to_string()), "Dire".to_string());
}

#[test]
fn test_14() {
    assert_eq!(Solution::predict_party_victory("DDDDRRRR".to_string()), "Dire".to_string());
}

#[test]
fn test_15() {
    assert_eq!(Solution::predict_party_victory("RDRDRD".to_string()), "Radiant".to_string());
}

#[test]
fn test_16() {
    assert_eq!(Solution::predict_party_victory("DDRRR".to_string()), "Dire".to_string());
}

#[test]
fn test_17() {
    assert_eq!(Solution::predict_party_victory("DDDRRR".to_string()), "Dire".to_string());
}

#[test]
fn test_18() {
    assert_eq!(Solution::predict_party_victory("DDDDDDRRRRRR".to_string()), "Dire".to_string());
}

#[test]
fn test_19() {
    assert_eq!(Solution::predict_party_victory("RRDD".to_string()), "Radiant".to_string());
}

#[test]
fn test_20() {
    assert_eq!(Solution::predict_party_victory("RRRDDDDRRDDRR".to_string()), "Radiant".to_string());
}

#[test]
fn test_21() {
    assert_eq!(Solution::predict_party_victory("RRDDD".to_string()), "Radiant".to_string());
}

#[test]
fn test_22() {
    assert_eq!(Solution::predict_party_victory("DRDRDRDRDRDRDR".to_string()), "Dire".to_string());
}

#[test]
fn test_23() {
    assert_eq!(Solution::predict_party_victory("DRDRDRDR".to_string()), "Dire".to_string());
}

#[test]
fn test_24() {
    assert_eq!(Solution::predict_party_victory("DDRR".to_string()), "Dire".to_string());
}

#[test]
fn test_25() {
    assert_eq!(Solution::predict_party_victory("DDRDRR".to_string()), "Dire".to_string());
}

#[test]
fn test_26() {
    assert_eq!(Solution::predict_party_victory("RRRRDDDD".to_string()), "Radiant".to_string());
}

#[test]
fn test_27() {
    assert_eq!(Solution::predict_party_victory("RRRR".to_string()), "Radiant".to_string());
}

#[test]
fn test_28() {
    assert_eq!(Solution::predict_party_victory("RRRRRRRDDDDDDD".to_string()), "Radiant".to_string());
}

#[test]
fn test_29() {
    assert_eq!(Solution::predict_party_victory("RDRD".to_string()), "Radiant".to_string());
}

#[test]
fn test_30() {
    assert_eq!(Solution::predict_party_victory("DRDRDRDRDRDRDRDRDR".to_string()), "Dire".to_string());
}

#[test]
fn test_31() {
    assert_eq!(Solution::predict_party_victory("DRDRDRDRDRDRDRDRDRDRDRDRDRDRDRDRD".to_string()), "Dire".to_string());
}

#[test]
fn test_32() {
    assert_eq!(Solution::predict_party_victory("RRDDDRRRRDDDRRDD".to_string()), "Radiant".to_string());
}

#[test]
fn test_33() {
    assert_eq!(Solution::predict_party_victory("AAAAAAAAAAAAADDDDDDDDDDDAAAAAAAAAAAAADDDDDDDDDDDAAAAAAAAAAAAADDDDDDDDDDDAAAAAAAAAAAAADDDDDDDDDDD".to_string()), "Dire".to_string());
}

#[test]
fn test_34() {
    assert_eq!(Solution::predict_party_victory("RDRRDDRRDDRRDD".to_string()), "Radiant".to_string());
}

#[test]
fn test_35() {
    assert_eq!(Solution::predict_party_victory("DDDDDRRRRRRDDDDRRRRRRRRDDDRRR".to_string()), "Radiant".to_string());
}

#[test]
fn test_36() {
    assert_eq!(Solution::predict_party_victory("RDDDRDRDRRDDRR".to_string()), "Dire".to_string());
}

#[test]
fn test_37() {
    assert_eq!(Solution::predict_party_victory("DDDDDDDDRRRRRDDD".to_string()), "Dire".to_string());
}

#[test]
fn test_38() {
    assert_eq!(Solution::predict_party_victory("RRDDRRDDRRDDRRDD".to_string()), "Radiant".to_string());
}

#[test]
fn test_39() {
    assert_eq!(Solution::predict_party_victory("RRRRRDDDDD-DDRRR".to_string()), "Radiant".to_string());
}

#[test]
fn test_40() {
    assert_eq!(Solution::predict_party_victory("RRRRRRRRRRDDDDDDDDDD".to_string()), "Radiant".to_string());
}

#[test]
fn test_41() {
    assert_eq!(Solution::predict_party_victory("RDRDRDRDRDRDRDRD".to_string()), "Radiant".to_string());
}

#[test]
fn test_42() {
    assert_eq!(Solution::predict_party_victory("RRRDDDRRDDDRRDDDRRDDRRRRRR".to_string()), "Radiant".to_string());
}

#[test]
fn test_43() {
    assert_eq!(Solution::predict_party_victory("DDDRRDDDRRRDDDDD".to_string()), "Dire".to_string());
}

#[test]
fn test_44() {
    assert_eq!(Solution::predict_party_victory("DDDDDDDDRRRRRR".to_string()), "Dire".to_string());
}

#[test]
fn test_45() {
    assert_eq!(Solution::predict_party_victory("RDRRDDDRRDDDRRDDDRDD".to_string()), "Dire".to_string());
}

#[test]
fn test_46() {
    assert_eq!(Solution::predict_party_victory("RRDDDDRRRRDDDDRR".to_string()), "Radiant".to_string());
}

#[test]
fn test_47() {
    assert_eq!(Solution::predict_party_victory("DRRDDDDDDRRRRRDD".to_string()), "Dire".to_string());
}

#[test]
fn test_48() {
    assert_eq!(Solution::predict_party_victory("RRRRRRRRRRRDDDDDDDDDDDDDDDDD".to_string()), "Radiant".to_string());
}

#[test]
fn test_49() {
    assert_eq!(Solution::predict_party_victory("RRRDRRDDDRRDDDRRDDDRRDDDRRDDDR".to_string()), "Radiant".to_string());
}

#[test]
fn test_50() {
    assert_eq!(Solution::predict_party_victory("RDDDRRDDDRRDDDRRDDDRRDDDRRDD".to_string()), "Dire".to_string());
}

#[test]
fn test_51() {
    assert_eq!(Solution::predict_party_victory("RRRDDDDRRRDDDDRRRDDDDRRRDDDD".to_string()), "Dire".to_string());
}

#[test]
fn test_52() {
    assert_eq!(Solution::predict_party_victory("DDRDRDRDRDRDRDRDRDRD".to_string()), "Dire".to_string());
}

#[test]
fn test_53() {
    assert_eq!(Solution::predict_party_victory("RRDDDDDDRRRRRRDDDDDDRRRRRRDDDD".to_string()), "Dire".to_string());
}

#[test]
fn test_54() {
    assert_eq!(Solution::predict_party_victory("RRRRRRDDDDDDRRRRRRDDDDDDRRRRRRDD".to_string()), "Radiant".to_string());
}

#[test]
fn test_55() {
    assert_eq!(Solution::predict_party_victory("RRDDDDDRDDRRDDDR".to_string()), "Dire".to_string());
}

#[test]
fn test_56() {
    assert_eq!(Solution::predict_party_victory("DDDDDDRRRRRRRRRR".to_string()), "Radiant".to_string());
}

#[test]
fn test_57() {
    assert_eq!(Solution::predict_party_victory("RRRDDDRRDDDRRDDDRRDDRR".to_string()), "Radiant".to_string());
}

#[test]
fn test_58() {
    assert_eq!(Solution::predict_party_victory("RRDDDDDDRRRDDDRRRRDD".to_string()), "Dire".to_string());
}

#[test]
fn test_59() {
    assert_eq!(Solution::predict_party_victory("RDRDRDRDRDRDRDRDRDRDRDRDRDRDRDRDRDRDRDRDRD".to_string()), "Radiant".to_string());
}

#[test]
fn test_60() {
    assert_eq!(Solution::predict_party_victory("DDDDRRRRRDRRDDDD".to_string()), "Dire".to_string());
}

#[test]
fn test_61() {
    assert_eq!(Solution::predict_party_victory("RDDDRRRRDDDDRRRR".to_string()), "Radiant".to_string());
}

#[test]
fn test_62() {
    assert_eq!(Solution::predict_party_victory("RRDRRDDDRRDDDRDD".to_string()), "Radiant".to_string());
}

#[test]
fn test_63() {
    assert_eq!(Solution::predict_party_victory("DDRDDRDDRDDRDDRDDRDDRDDRDDRDDRDDRDDRDDRDDRDDRDDRDDRDDRDDRDDRDD".to_string()), "Dire".to_string());
}

#[test]
fn test_64() {
    assert_eq!(Solution::predict_party_victory("DRDRDRDRDRDRDRDRDRDR".to_string()), "Dire".to_string());
}

#[test]
fn test_65() {
    assert_eq!(Solution::predict_party_victory("DDDDDDDDDDDDDDDDAAAAAAAAAAAAAAAAAAAAADDDDDDDDDDDDDDDAAAAAAAAAAAAAAAAAAAAADDDDDDDDDDDDDDDAAAAAAAAAAAAAAAAAAAAADDDDDDDDDDDDDDDAAAAAAAAAAAAAAAAAAAAA".to_string()), "Dire".to_string());
}

#[test]
fn test_66() {
    assert_eq!(Solution::predict_party_victory("RRRRRRRDDDDDDDD".to_string()), "Radiant".to_string());
}

#[test]
fn test_67() {
    assert_eq!(Solution::predict_party_victory("DRDRDRDRDRDRDRDRDRDRDRDRDRDRDRDR".to_string()), "Dire".to_string());
}

#[test]
fn test_68() {
    assert_eq!(Solution::predict_party_victory("RRRDDDDRRRRRDDDD".to_string()), "Radiant".to_string());
}

#[test]
fn test_69() {
    assert_eq!(Solution::predict_party_victory("RRRDDDRRDDDRRDDDRRDD".to_string()), "Radiant".to_string());
}

#[test]
fn test_70() {
    assert_eq!(Solution::predict_party_victory("DDDDDDDDDAAAAAAAAAAAADDDDDDDDDAAAAAAAAAAAADDDDDDDDDAAAAAAAAAAAA".to_string()), "Dire".to_string());
}

#[test]
fn test_71() {
    assert_eq!(Solution::predict_party_victory("RRRRRRRRDDDDDDDDD".to_string()), "Radiant".to_string());
}

#[test]
fn test_72() {
    assert_eq!(Solution::predict_party_victory("DRRDDDRRDDDRRDDDRRDDDDDDDD".to_string()), "Dire".to_string());
}

#[test]
fn test_73() {
    assert_eq!(Solution::predict_party_victory("RRRRRDDDDRRRRRDD".to_string()), "Radiant".to_string());
}

#[test]
fn test_74() {
    assert_eq!(Solution::predict_party_victory("RRRRRRRRRRDDDDDDDDDDDDDDDDDDDD".to_string()), "Dire".to_string());
}

#[test]
fn test_75() {
    assert_eq!(Solution::predict_party_victory("RRDDDDRRRRRDDDDRRRR".to_string()), "Radiant".to_string());
}

#[test]
fn test_76() {
    assert_eq!(Solution::predict_party_victory("AAAAAAAAAAADDDDDDDDDAAAAAAAAAAADDDDDDDDDAAAAAAAAAAADDDDDDDDDAAAAAAAAAAA".to_string()), "Dire".to_string());
}

#[test]
fn test_77() {
    assert_eq!(Solution::predict_party_victory("DDDDDDRRRRRRRRRRRRRR".to_string()), "Radiant".to_string());
}

#[test]
fn test_78() {
    assert_eq!(Solution::predict_party_victory("DDDDRRRRDDRRRDDRRRDD".to_string()), "Dire".to_string());
}

#[test]
fn test_79() {
    assert_eq!(Solution::predict_party_victory("RRRRDDDDDDRRRRRDDDDDDRRRRRDD".to_string()), "Radiant".to_string());
}

#[test]
fn test_80() {
    assert_eq!(Solution::predict_party_victory("RRDDRRRDRDDDRRDD".to_string()), "Radiant".to_string());
}

#[test]
fn test_81() {
    assert_eq!(Solution::predict_party_victory("RRDDDDRRDDDDRRDDDDRRDDDDRRDDDD".to_string()), "Dire".to_string());
}

#[test]
fn test_82() {
    assert_eq!(Solution::predict_party_victory("RRDDDRRDDDDDRRDD".to_string()), "Dire".to_string());
}

#[test]
fn test_83() {
    assert_eq!(Solution::predict_party_victory("RDDRRRDDDDRRRDDDDRRRDD".to_string()), "Dire".to_string());
}

#[test]
fn test_84() {
    assert_eq!(Solution::predict_party_victory("DRRDDDRRDDDRRDDDRRDDDDDD".to_string()), "Dire".to_string());
}

#[test]
fn test_85() {
    assert_eq!(Solution::predict_party_victory("DDDDRRRDDDDRRRDDDDRRRDDDDRRRDD".to_string()), "Dire".to_string());
}

#[test]
fn test_86() {
    assert_eq!(Solution::predict_party_victory("RRDRDRDRDRDRDRDRDRDR".to_string()), "Radiant".to_string());
}

#[test]
fn test_87() {
    assert_eq!(Solution::predict_party_victory("RRDDRRDDRRDDRRDDRRDDRRDDRRDDRRDD".to_string()), "Radiant".to_string());
}

#[test]
fn test_88() {
    assert_eq!(Solution::predict_party_victory("RRRDDDRRDDDRRDDDRRDDRRRDDDRRDDDRRDD".to_string()), "Dire".to_string());
}

#[test]
fn test_89() {
    assert_eq!(Solution::predict_party_victory("DDRRDDRRDDRRDDRRDDRRDDRRDDRRDDRRDDRRDDRRDD".to_string()), "Dire".to_string());
}

#[test]
fn test_90() {
    assert_eq!(Solution::predict_party_victory("DDDDDRRRDDDRRRDD".to_string()), "Dire".to_string());
}

#[test]
fn test_91() {
    assert_eq!(Solution::predict_party_victory("DRRRRRDDDDDDD".to_string()), "Dire".to_string());
}

#[test]
fn test_92() {
    assert_eq!(Solution::predict_party_victory("RDRDRDRDRDRDRDRDRDRDRDRDRDRDRD".to_string()), "Radiant".to_string());
}

#[test]
fn test_93() {
    assert_eq!(Solution::predict_party_victory("DDDDDDRRRRRRDDDDDDRRRRRRDDDDDD".to_string()), "Dire".to_string());
}

#[test]
fn test_94() {
    assert_eq!(Solution::predict_party_victory("RRRDDDDRRRDDRRRRRR".to_string()), "Radiant".to_string());
}

#[test]
fn test_95() {
    assert_eq!(Solution::predict_party_victory("DDRRDDRRDDRRDDRRDDRRDDRRDDRRDD".to_string()), "Dire".to_string());
}

#[test]
fn test_96() {
    assert_eq!(Solution::predict_party_victory("DRDDRRDDRRDDRRDD".to_string()), "Dire".to_string());
}

#[test]
fn test_97() {
    assert_eq!(Solution::predict_party_victory("RDDDRRDDDRRDDDRRDD".to_string()), "Dire".to_string());
}

#[test]
fn test_98() {
    assert_eq!(Solution::predict_party_victory("RRRRRRRDRDDDDDDD".to_string()), "Radiant".to_string());
}

#[test]
fn test_99() {
    assert_eq!(Solution::predict_party_victory("DDDRRRDDRRRDD".to_string()), "Dire".to_string());
}

#[test]
fn test_100() {
    assert_eq!(Solution::predict_party_victory("DDRDRDRDRDRDRDRDRDRDRDRDRDRDRDRDRD".to_string()), "Dire".to_string());
}

#[test]
fn test_101() {
    assert_eq!(Solution::predict_party_victory("RRRRDDDDDDDDRRRRRRDDDD".to_string()), "Dire".to_string());
}

#[test]
fn test_102() {
    assert_eq!(Solution::predict_party_victory("DDRDRDRDRDRDRDRDRDRDRDRDRDRD".to_string()), "Dire".to_string());
}

#[test]
fn test_103() {
    assert_eq!(Solution::predict_party_victory("DRRDDDRRDDDRRDDDRRDDRRRDDDRRDDDRRDD".to_string()), "Dire".to_string());
}

#[test]
fn test_104() {
    assert_eq!(Solution::predict_party_victory("DDDDDDDDDDRRRRRRRRRR".to_string()), "Dire".to_string());
}

#[test]
fn test_105() {
    assert_eq!(Solution::predict_party_victory("DRDDRRDRDRDRDR".to_string()), "Dire".to_string());
}

#[test]
fn test_106() {
    assert_eq!(Solution::predict_party_victory("DDRRDDDDBBRRDDDDRRRRDDRRDDDDBB".to_string()), "Dire".to_string());
}

#[test]
fn test_107() {
    assert_eq!(Solution::predict_party_victory("RRRRDDDDDDDDRRRR".to_string()), "Radiant".to_string());
}

#[test]
fn test_108() {
    assert_eq!(Solution::predict_party_victory("AAAAAAAAAAAAAAAAAAAAADDDDDDDDDDDDDDDAAAAAAAAAAAAAAAAAAAAADDDDDDDDDDDDDDDAAAAAAAAAAAAAAAAAAAAADDDDDDDDDDDDDDDAAAAAAAAAAAAAAAAAAAAADDDDDDDDDDDDDDDAAAAAAAAAAAAAAAAAAAAA".to_string()), "Dire".to_string());
}

#[test]
fn test_109() {
    assert_eq!(Solution::predict_party_victory("DDDDRRRRDDD".to_string()), "Dire".to_string());
}

#[test]
fn test_110() {
    assert_eq!(Solution::predict_party_victory("DDRRRRRRRRDD".to_string()), "Radiant".to_string());
}

#[test]
fn test_111() {
    assert_eq!(Solution::predict_party_victory("DDDDRRRRRRRRRRRRRDDDDDDDD".to_string()), "Radiant".to_string());
}

#[test]
fn test_112() {
    assert_eq!(Solution::predict_party_victory("RRRDDDRDDRRRDDDD".to_string()), "Radiant".to_string());
}

#[test]
fn test_113() {
    assert_eq!(Solution::predict_party_victory("RRRRDDDSSSSSRRRRRR".to_string()), "Radiant".to_string());
}

#[test]
fn test_114() {
    assert_eq!(Solution::predict_party_victory("RRRRRRDDDDDDDDDDDD".to_string()), "Dire".to_string());
}

#[test]
fn test_115() {
    assert_eq!(Solution::predict_party_victory("RRRRRDDDDDRRRRDDDD".to_string()), "Radiant".to_string());
}

#[test]
fn test_116() {
    assert_eq!(Solution::predict_party_victory("DDDDRRRRRR".to_string()), "Dire".to_string());
}

#[test]
fn test_117() {
    assert_eq!(Solution::predict_party_victory("RRRRDDD-DDDDDDRRRR".to_string()), "Dire".to_string());
}

#[test]
fn test_118() {
    assert_eq!(Solution::predict_party_victory("RRRRRDDDDD".to_string()), "Radiant".to_string());
}

#[test]
fn test_119() {
    assert_eq!(Solution::predict_party_victory("RRDDDDRRDDRRDDRRDDRRDDRRDD".to_string()), "Dire".to_string());
}

#[test]
fn test_120() {
    assert_eq!(Solution::predict_party_victory("RRRRRRRRRRRRDDDDDD".to_string()), "Radiant".to_string());
}

#[test]
fn test_121() {
    assert_eq!(Solution::predict_party_victory("DRRDDDRRDDDRRDDDRRDDDRRDDDRRDD".to_string()), "Dire".to_string());
}

#[test]
fn test_122() {
    assert_eq!(Solution::predict_party_victory("RRRDDDDRRRDDDDRRRDDDDRRRDDDDRRRDDDD".to_string()), "Dire".to_string());
}

#[test]
fn test_123() {
    assert_eq!(Solution::predict_party_victory("DDDDDDDDRRRRRRRRDDDDDDDDRRRRRRRR".to_string()), "Dire".to_string());
}

#[test]
fn test_124() {
    assert_eq!(Solution::predict_party_victory("DDRRRRDDDDRRRRDD".to_string()), "Dire".to_string());
}

#[test]
fn test_125() {
    assert_eq!(Solution::predict_party_victory("RRRRRDDDRRDDDRRDDDRRDD".to_string()), "Radiant".to_string());
}

#[test]
fn test_126() {
    assert_eq!(Solution::predict_party_victory("RRDRRDDRRRDDRRDR".to_string()), "Radiant".to_string());
}

#[test]
fn test_127() {
    assert_eq!(Solution::predict_party_victory("RRRRRDDDDDRRRRDDDDDRRRRDDDDD".to_string()), "Radiant".to_string());
}

#[test]
fn test_128() {
    assert_eq!(Solution::predict_party_victory("DRRRRDDDDDDD".to_string()), "Dire".to_string());
}

#[test]
fn test_129() {
    assert_eq!(Solution::predict_party_victory("DDDDDDDDDDDDDDRRRRRRRRRRRRRR".to_string()), "Dire".to_string());
}

#[test]
fn test_130() {
    assert_eq!(Solution::predict_party_victory("RRRRRRRDDDDDDDDD".to_string()), "Radiant".to_string());
}

#[test]
fn test_131() {
    assert_eq!(Solution::predict_party_victory("RRRDRRDDDRRDRRDDDRRDRRDDDRRD".to_string()), "Radiant".to_string());
}

#[test]
fn test_132() {
    assert_eq!(Solution::predict_party_victory("DRRDDDRRDDDRRDDDRRDD".to_string()), "Dire".to_string());
}

#[test]
fn test_133() {
    assert_eq!(Solution::predict_party_victory("RDRDRDRDRDRDRDRDRDRDRDRDRDRDRDRDRDRDRDRDRDRDRDRDRDRDRDRDRDRDRDR".to_string()), "Radiant".to_string());
}

#[test]
fn test_134() {
    assert_eq!(Solution::predict_party_victory("RRRDDDDRRRDDDDRRRDDDDRRRDDDDRR".to_string()), "Dire".to_string());
}

#[test]
fn test_135() {
    assert_eq!(Solution::predict_party_victory("RDRDRDRDRDRDRDRDRDRD".to_string()), "Radiant".to_string());
}

#[test]
fn test_136() {
    assert_eq!(Solution::predict_party_victory("RRDRDRDRDRDRDRDRDRDRDRDRDRDRDRDRDR".to_string()), "Radiant".to_string());
}

#[test]
fn test_137() {
    assert_eq!(Solution::predict_party_victory("DRDRDRDRDRDRDRDRDRDRDRDR".to_string()), "Dire".to_string());
}

#[test]
fn test_138() {
    assert_eq!(Solution::predict_party_victory("RRRDRDDDRRDD".to_string()), "Radiant".to_string());
}

#[test]
fn test_139() {
    assert_eq!(Solution::predict_party_victory("DRDRDRDRDRDRDRDRDRDRDRDRDRDRDR".to_string()), "Dire".to_string());
}

#[test]
fn test_140() {
    assert_eq!(Solution::predict_party_victory("DDDDDDRRRRDDDDDDRRRRDDDDDDRRRR".to_string()), "Dire".to_string());
}

#[test]
fn test_141() {
    assert_eq!(Solution::predict_party_victory("RRRRRDDDDDDRRRRRDDDDDDRRRRRDDDDD".to_string()), "Radiant".to_string());
}

#[test]
fn test_142() {
    assert_eq!(Solution::predict_party_victory("RRRRDDDDRRRRDDDD".to_string()), "Radiant".to_string());
}

#[test]
fn test_143() {
    assert_eq!(Solution::predict_party_victory("RRRRRRRRRRRRRRRRRDDDDDDDDDDDDDDDDDRRRRRRRRRRRRRRRRDDDDDDDDDDDDDDDDDRRRRRRRRRRRRRRRRDDDDDDDDDDDDDDDDD".to_string()), "Radiant".to_string());
}

#[test]
fn test_144() {
    assert_eq!(Solution::predict_party_victory("RRDDRRDDRRDDRRDDRRDDRRDD".to_string()), "Radiant".to_string());
}

#[test]
fn test_145() {
    assert_eq!(Solution::predict_party_victory("RRDRDRDRDRDRDR".to_string()), "Radiant".to_string());
}

#[test]
fn test_146() {
    assert_eq!(Solution::predict_party_victory("DDDDDDDDDDDAAAAAAAAAAAAADDDDDDDDDAAAAAAAAAAAADDDDDDDDDAAAAAAAAAAADDDDDDDDDAAAAAAAAAAA".to_string()), "Dire".to_string());
}

#[test]
fn test_147() {
    assert_eq!(Solution::predict_party_victory("DRDDDRRDDDRRDDDRRDDDRRDDDRRDDDRRDD".to_string()), "Dire".to_string());
}

#[test]
fn test_148() {
    assert_eq!(Solution::predict_party_victory("DDDDDDRRRRRRDDDDDD".to_string()), "Dire".to_string());
}

#[test]
fn test_149() {
    assert_eq!(Solution::predict_party_victory("DDDDDDRRRRRRDDRRRRR".to_string()), "Dire".to_string());
}

#[test]
fn test_150() {
    assert_eq!(Solution::predict_party_victory("DDDRRRDDDDRRRRRDDDDDDRRRRRDDDDDDD".to_string()), "Dire".to_string());
}

#[test]
fn test_151() {
    assert_eq!(Solution::predict_party_victory("DDDDRRRRDDDDRRRRDDDDRRRRDDDDRRRRDDDDRRRR".to_string()), "Dire".to_string());
}

#[test]
fn test_152() {
    assert_eq!(Solution::predict_party_victory("RRDDRRDDRRDDRRDDRRDDRRDDRRDDRR".to_string()), "Radiant".to_string());
}

#[test]
fn test_153() {
    assert_eq!(Solution::predict_party_victory("DRRDDDRRDDDRRDDDRRDDDD".to_string()), "Dire".to_string());
}

#[test]
fn test_154() {
    assert_eq!(Solution::predict_party_victory("RRRRRDDDDDRRRRDDDDRRRRRDDDDDRR".to_string()), "Radiant".to_string());
}

#[test]
fn test_155() {
    assert_eq!(Solution::predict_party_victory("RRRRRDDDDDDDDDRRRRDDDDDDDDDRRRRDDDDDDDDDRRRRDDDDDDDDDRRRRDDDDDD".to_string()), "Dire".to_string());
}

#[test]
fn test_156() {
    assert_eq!(Solution::predict_party_victory("DDDDDDDDDDRRRRRRR".to_string()), "Dire".to_string());
}

#[test]
fn test_157() {
    assert_eq!(Solution::predict_party_victory("DDDDRRRRDDDDRRRR".to_string()), "Dire".to_string());
}

#[test]
fn test_158() {
    assert_eq!(Solution::predict_party_victory("DDDDRRDDDDRRDDDDRRDDDDRRDDDDRR".to_string()), "Dire".to_string());
}

#[test]
fn test_159() {
    assert_eq!(Solution::predict_party_victory("RRRRRDDDDDDDDDRRRDDDRRDDDD".to_string()), "Dire".to_string());
}

#[test]
fn test_160() {
    assert_eq!(Solution::predict_party_victory("RRRRDDDDRRRRDDDDRRRRDDDDRRRRDDDDRRRRDDDD".to_string()), "Radiant".to_string());
}

#[test]
fn test_161() {
    assert_eq!(Solution::predict_party_victory("RRRRRRRRRRRRRDDDDDDDDDDDDDRRRRRRRRRRRRDDDDDDDDDDDDDRRRRRRRRRRRRDDDDDDDDD".to_string()), "Radiant".to_string());
}

#[test]
fn test_162() {
    assert_eq!(Solution::predict_party_victory("DDDDDDDDRRRRRRRR".to_string()), "Dire".to_string());
}

#[test]
fn test_163() {
    assert_eq!(Solution::predict_party_victory("DDRRRDDDDRRRRDDDRRR".to_string()), "Dire".to_string());
}

#[test]
fn test_164() {
    assert_eq!(Solution::predict_party_victory("DDRRDDRRDDRRDDRR".to_string()), "Dire".to_string());
}

#[test]
fn test_165() {
    assert_eq!(Solution::predict_party_victory("RRDDDRRDDDRRDDDRRDDDRRDDDRRDDDRRDD".to_string()), "Dire".to_string());
}

#[test]
fn test_166() {
    assert_eq!(Solution::predict_party_victory("DDDDRRDDRRDDRRDDRRDD".to_string()), "Dire".to_string());
}

#[test]
fn test_167() {
    assert_eq!(Solution::predict_party_victory("DRDDRRDDRRDDRRDR".to_string()), "Dire".to_string());
}

#[test]
fn test_168() {
    assert_eq!(Solution::predict_party_victory("DRDRDRDRDRDRDRDR".to_string()), "Dire".to_string());
}

#[test]
fn test_169() {
    assert_eq!(Solution::predict_party_victory("RRRDDDDDRRRRDDDDDRRRRDDDDDRRRRDDDDDRRRRDDDD".to_string()), "Dire".to_string());
}

#[test]
fn test_170() {
    assert_eq!(Solution::predict_party_victory("RRRDDDDRRRDDDDRR".to_string()), "Radiant".to_string());
}

#[test]
fn test_171() {
    assert_eq!(Solution::predict_party_victory("RDDDRDDDRDDDRDDD".to_string()), "Dire".to_string());
}

#[test]
fn test_172() {
    assert_eq!(Solution::predict_party_victory("RRDRDRDRDRDRDRDRDRDRDRDRDRDR".to_string()), "Radiant".to_string());
}

#[test]
fn test_173() {
    assert_eq!(Solution::predict_party_victory("DDRRDDRRDDRRDDRRDDRRDDRRDDRRDDRR".to_string()), "Dire".to_string());
}

#[test]
fn test_174() {
    assert_eq!(Solution::predict_party_victory("DDRDRDRDRDRDRDRD".to_string()), "Dire".to_string());
}

#[test]
fn test_175() {
    assert_eq!(Solution::predict_party_victory("DDRDDDRRDDDRRDDDRRDDDRRDDDRRDDDR".to_string()), "Dire".to_string());
}

#[test]
fn test_176() {
    assert_eq!(Solution::predict_party_victory("RRRDDDRRDDDRRDDDRRDDRRRR".to_string()), "Radiant".to_string());
}

#[test]
fn test_177() {
    assert_eq!(Solution::predict_party_victory("RRRRRDDDDRRRDDDDRRRDDDDRRRDD".to_string()), "Radiant".to_string());
}
