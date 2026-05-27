include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::open_lock(vec![], "1111".to_string()), 4);
}

#[test]
fn test_2() {
    assert_eq!(Solution::open_lock(vec!["1111".to_string(), "2222".to_string(), "3333".to_string()], "4444".to_string()), 16);
}

#[test]
fn test_3() {
    assert_eq!(Solution::open_lock(vec!["0001".to_string(), "0002".to_string(), "0003".to_string(), "0004".to_string()], "0005".to_string()), 5);
}

#[test]
fn test_4() {
    assert_eq!(Solution::open_lock(vec!["0001".to_string(), "0011".to_string(), "0101".to_string(), "1001".to_string(), "1101".to_string(), "0111".to_string(), "0110".to_string(), "1010".to_string(), "1110".to_string(), "1100".to_string()], "1111".to_string()), 6);
}

#[test]
fn test_5() {
    assert_eq!(Solution::open_lock(vec!["1110".to_string(), "1101".to_string(), "1011".to_string(), "0111".to_string()], "1111".to_string()), 6);
}

#[test]
fn test_6() {
    assert_eq!(Solution::open_lock(vec!["9998".to_string(), "9997".to_string(), "9996".to_string(), "9995".to_string()], "9999".to_string()), 4);
}

#[test]
fn test_7() {
    assert_eq!(Solution::open_lock(vec!["0000".to_string()], "1111".to_string()), -1);
}

#[test]
fn test_8() {
    assert_eq!(Solution::open_lock(vec!["1111".to_string(), "2222".to_string(), "3333".to_string(), "4444".to_string()], "5555".to_string()), 20);
}

#[test]
fn test_9() {
    assert_eq!(Solution::open_lock(vec!["1234".to_string()], "0000".to_string()), 0);
}

#[test]
fn test_10() {
    assert_eq!(Solution::open_lock(vec!["8888".to_string()], "0009".to_string()), 1);
}

#[test]
fn test_11() {
    assert_eq!(Solution::open_lock(vec!["0002".to_string(), "0020".to_string(), "0200".to_string(), "2000".to_string()], "0000".to_string()), 0);
}

#[test]
fn test_12() {
    assert_eq!(Solution::open_lock(vec!["0001".to_string(), "0010".to_string(), "0100".to_string(), "1000".to_string()], "1111".to_string()), 6);
}

#[test]
fn test_13() {
    assert_eq!(Solution::open_lock(vec!["0201".to_string(), "0101".to_string(), "0102".to_string(), "1212".to_string(), "2002".to_string()], "0202".to_string()), 6);
}

#[test]
fn test_14() {
    assert_eq!(Solution::open_lock(vec!["1110".to_string(), "1101".to_string(), "1011".to_string(), "0111".to_string()], "0000".to_string()), 0);
}

#[test]
fn test_15() {
    assert_eq!(Solution::open_lock(vec!["1111".to_string()], "0000".to_string()), 0);
}

#[test]
fn test_16() {
    assert_eq!(Solution::open_lock(vec!["1111".to_string()], "1112".to_string()), 5);
}

#[test]
fn test_17() {
    assert_eq!(Solution::open_lock(vec!["0001".to_string(), "0010".to_string(), "0100".to_string(), "1000".to_string()], "0000".to_string()), 0);
}

#[test]
fn test_18() {
    assert_eq!(Solution::open_lock(vec!["8887".to_string(), "8889".to_string(), "8878".to_string(), "8898".to_string(), "8788".to_string(), "8988".to_string(), "7888".to_string(), "9888".to_string()], "8888".to_string()), -1);
}

#[test]
fn test_19() {
    assert_eq!(Solution::open_lock(vec![], "0000".to_string()), 0);
}

#[test]
fn test_20() {
    assert_eq!(Solution::open_lock(vec!["0000".to_string()], "8888".to_string()), -1);
}

#[test]
fn test_21() {
    assert_eq!(Solution::open_lock(vec!["9999".to_string()], "0000".to_string()), 0);
}

#[test]
fn test_22() {
    assert_eq!(Solution::open_lock(vec!["1234".to_string(), "2345".to_string(), "3456".to_string(), "4567".to_string(), "5678".to_string(), "6789".to_string(), "7890".to_string(), "8901".to_string(), "9012".to_string(), "0123".to_string()], "5555".to_string()), 20);
}

#[test]
fn test_23() {
    assert_eq!(Solution::open_lock(vec!["1234".to_string(), "4321".to_string(), "2345".to_string(), "5432".to_string(), "3456".to_string(), "6543".to_string(), "4567".to_string(), "7654".to_string(), "5678".to_string()], "8765".to_string()), 14);
}

#[test]
fn test_24() {
    assert_eq!(Solution::open_lock(vec!["1234".to_string(), "4321".to_string(), "2143".to_string(), "3412".to_string(), "0987".to_string(), "7890".to_string(), "8709".to_string(), "9087".to_string()], "0000".to_string()), 0);
}

#[test]
fn test_25() {
    assert_eq!(Solution::open_lock(vec!["1111".to_string(), "2222".to_string(), "3333".to_string(), "4444".to_string(), "5555".to_string(), "6666".to_string(), "7777".to_string(), "8888".to_string(), "9999".to_string(), "0000".to_string()], "1234".to_string()), -1);
}

#[test]
fn test_26() {
    assert_eq!(Solution::open_lock(vec!["0011".to_string(), "0110".to_string(), "1100".to_string(), "1001".to_string(), "1112".to_string(), "1211".to_string(), "2111".to_string()], "2222".to_string()), 8);
}

#[test]
fn test_27() {
    assert_eq!(Solution::open_lock(vec!["1000".to_string(), "0100".to_string(), "0010".to_string(), "0001".to_string(), "0011".to_string(), "0110".to_string(), "1010".to_string(), "1100".to_string(), "1110".to_string()], "1111".to_string()), 6);
}

#[test]
fn test_28() {
    assert_eq!(Solution::open_lock(vec!["0009".to_string(), "0018".to_string(), "0108".to_string(), "0117".to_string(), "0126".to_string(), "0135".to_string(), "0144".to_string(), "0153".to_string(), "0162".to_string(), "0171".to_string()], "0000".to_string()), 0);
}

#[test]
fn test_29() {
    assert_eq!(Solution::open_lock(vec!["1234".to_string(), "2345".to_string(), "3456".to_string(), "4567".to_string(), "5678".to_string(), "6789".to_string(), "7890".to_string(), "8901".to_string(), "9012".to_string(), "0123".to_string()], "9999".to_string()), 4);
}

#[test]
fn test_30() {
    assert_eq!(Solution::open_lock(vec!["1111".to_string(), "2222".to_string(), "3333".to_string(), "4444".to_string(), "5555".to_string(), "6666".to_string(), "7777".to_string(), "8888".to_string(), "9999".to_string()], "0000".to_string()), 0);
}

#[test]
fn test_31() {
    assert_eq!(Solution::open_lock(vec!["0101".to_string(), "1010".to_string(), "1100".to_string(), "1111".to_string()], "0011".to_string()), 2);
}

#[test]
fn test_32() {
    assert_eq!(Solution::open_lock(vec!["0000".to_string(), "1111".to_string(), "2222".to_string(), "3333".to_string(), "4444".to_string()], "5555".to_string()), -1);
}

#[test]
fn test_33() {
    assert_eq!(Solution::open_lock(vec!["0001".to_string(), "0010".to_string(), "1000".to_string(), "0100".to_string(), "0011".to_string(), "1100".to_string(), "0110".to_string(), "1010".to_string()], "1111".to_string()), 6);
}

#[test]
fn test_34() {
    assert_eq!(Solution::open_lock(vec!["0001".to_string(), "1112".to_string(), "2223".to_string(), "3334".to_string(), "4445".to_string(), "5556".to_string(), "6667".to_string(), "7778".to_string(), "8889".to_string()], "9999".to_string()), 4);
}

#[test]
fn test_35() {
    assert_eq!(Solution::open_lock(vec!["0123".to_string(), "9876".to_string(), "5432".to_string(), "1357".to_string(), "2468".to_string(), "0909".to_string(), "1111".to_string(), "2222".to_string(), "3333".to_string()], "4444".to_string()), 16);
}

#[test]
fn test_36() {
    assert_eq!(Solution::open_lock(vec!["9998".to_string(), "9989".to_string(), "9899".to_string(), "8999".to_string(), "0000".to_string(), "1111".to_string(), "2222".to_string(), "3333".to_string(), "4444".to_string(), "5555".to_string()], "6789".to_string()), -1);
}

#[test]
fn test_37() {
    assert_eq!(Solution::open_lock(vec!["0001".to_string(), "0002".to_string(), "0003".to_string(), "0004".to_string(), "0005".to_string(), "0006".to_string(), "0007".to_string(), "0008".to_string(), "0009".to_string()], "1111".to_string()), 4);
}

#[test]
fn test_38() {
    assert_eq!(Solution::open_lock(vec!["0123".to_string(), "1234".to_string(), "2345".to_string(), "3456".to_string(), "4567".to_string(), "5678".to_string(), "6789".to_string(), "7890".to_string(), "8901".to_string(), "9012".to_string()], "0123".to_string()), 6);
}

#[test]
fn test_39() {
    assert_eq!(Solution::open_lock(vec!["0000".to_string(), "1111".to_string(), "2222".to_string(), "3333".to_string(), "4444".to_string(), "5555".to_string(), "6666".to_string(), "7777".to_string(), "8888".to_string(), "9999".to_string()], "9999".to_string()), -1);
}

#[test]
fn test_40() {
    assert_eq!(Solution::open_lock(vec!["0001".to_string(), "0002".to_string(), "0003".to_string(), "0004".to_string(), "0005".to_string(), "0006".to_string(), "0007".to_string(), "0008".to_string(), "0009".to_string(), "0100".to_string()], "0011".to_string()), 2);
}

#[test]
fn test_41() {
    assert_eq!(Solution::open_lock(vec!["9999".to_string(), "8888".to_string(), "7777".to_string(), "6666".to_string(), "5555".to_string(), "4444".to_string(), "3333".to_string(), "2222".to_string(), "1111".to_string()], "0000".to_string()), 0);
}

#[test]
fn test_42() {
    assert_eq!(Solution::open_lock(vec!["1234".to_string(), "2345".to_string(), "3456".to_string(), "4567".to_string(), "5678".to_string(), "6789".to_string(), "7890".to_string(), "8901".to_string(), "9012".to_string(), "0123".to_string()], "1000".to_string()), 1);
}

#[test]
fn test_43() {
    assert_eq!(Solution::open_lock(vec!["1111".to_string(), "2222".to_string(), "3333".to_string(), "4444".to_string(), "5555".to_string(), "6666".to_string(), "7777".to_string(), "8888".to_string(), "9999".to_string(), "1234".to_string(), "4321".to_string(), "2345".to_string(), "5432".to_string(), "3456".to_string(), "6543".to_string(), "4567".to_string(), "7654".to_string(), "5678".to_string(), "8765".to_string(), "6789".to_string(), "9876".to_string(), "7890".to_string(), "0987".to_string(), "8790".to_string(), "9087".to_string(), "0879".to_string()], "0000".to_string()), 0);
}

#[test]
fn test_44() {
    assert_eq!(Solution::open_lock(vec!["0000".to_string(), "9999".to_string(), "1111".to_string(), "2222".to_string(), "3333".to_string(), "4444".to_string(), "5555".to_string(), "6666".to_string(), "7777".to_string(), "8888".to_string()], "1234".to_string()), -1);
}

#[test]
fn test_45() {
    assert_eq!(Solution::open_lock(vec!["0011".to_string(), "0101".to_string(), "1001".to_string(), "1100".to_string(), "1110".to_string(), "1101".to_string()], "1111".to_string()), 4);
}

#[test]
fn test_46() {
    assert_eq!(Solution::open_lock(vec!["0001".to_string(), "0002".to_string(), "0003".to_string(), "0004".to_string(), "0005".to_string(), "0006".to_string(), "0007".to_string(), "0008".to_string(), "0009".to_string(), "0100".to_string()], "9999".to_string()), 4);
}

#[test]
fn test_47() {
    assert_eq!(Solution::open_lock(vec!["1000".to_string(), "0100".to_string(), "0010".to_string(), "0001".to_string(), "2000".to_string(), "0200".to_string(), "0020".to_string(), "0002".to_string()], "0000".to_string()), 0);
}

#[test]
fn test_48() {
    assert_eq!(Solution::open_lock(vec!["0000".to_string(), "0009".to_string(), "0090".to_string(), "0900".to_string(), "9000".to_string(), "1111".to_string(), "2222".to_string(), "3333".to_string(), "4444".to_string()], "5555".to_string()), -1);
}

#[test]
fn test_49() {
    assert_eq!(Solution::open_lock(vec!["0010".to_string(), "0100".to_string(), "0001".to_string(), "0110".to_string(), "1001".to_string(), "1100".to_string(), "1111".to_string(), "0000".to_string()], "0101".to_string()), -1);
}

#[test]
fn test_50() {
    assert_eq!(Solution::open_lock(vec!["0001".to_string(), "0002".to_string(), "0010".to_string(), "0020".to_string(), "0100".to_string(), "0200".to_string(), "1000".to_string(), "2000".to_string(), "0003".to_string(), "0030".to_string(), "0300".to_string(), "3000".to_string()], "0004".to_string()), 6);
}

#[test]
fn test_51() {
    assert_eq!(Solution::open_lock(vec!["1234".to_string(), "2345".to_string(), "3456".to_string(), "4567".to_string(), "5678".to_string(), "6789".to_string(), "7890".to_string(), "8901".to_string(), "9012".to_string()], "0123".to_string()), 6);
}

#[test]
fn test_52() {
    assert_eq!(Solution::open_lock(vec!["0001".to_string(), "0010".to_string(), "0100".to_string(), "1000".to_string(), "0111".to_string(), "1110".to_string(), "1101".to_string(), "1011".to_string(), "2220".to_string(), "2202".to_string(), "2022".to_string(), "0222".to_string()], "0202".to_string()), 6);
}

#[test]
fn test_53() {
    assert_eq!(Solution::open_lock(vec!["1234".to_string(), "4321".to_string(), "2143".to_string(), "3412".to_string()], "5678".to_string()), 14);
}

#[test]
fn test_54() {
    assert_eq!(Solution::open_lock(vec!["1234".to_string(), "2345".to_string(), "3456".to_string(), "4567".to_string(), "5678".to_string(), "6789".to_string(), "7890".to_string(), "8901".to_string(), "9012".to_string(), "0123".to_string(), "0234".to_string(), "1345".to_string(), "2456".to_string(), "3567".to_string(), "4678".to_string(), "5789".to_string(), "6890".to_string(), "7901".to_string(), "8012".to_string(), "9123".to_string()], "4567".to_string()), 16);
}

#[test]
fn test_55() {
    assert_eq!(Solution::open_lock(vec!["0001".to_string(), "0010".to_string(), "0100".to_string(), "1000".to_string(), "1234".to_string(), "4321".to_string(), "5678".to_string(), "8765".to_string(), "9999".to_string()], "8888".to_string()), 8);
}

#[test]
fn test_56() {
    assert_eq!(Solution::open_lock(vec!["0001".to_string(), "0002".to_string(), "0003".to_string(), "0004".to_string(), "0005".to_string(), "0006".to_string(), "0007".to_string(), "0008".to_string(), "0009".to_string()], "9999".to_string()), 4);
}

#[test]
fn test_57() {
    assert_eq!(Solution::open_lock(vec!["9999".to_string(), "8888".to_string(), "7777".to_string(), "6666".to_string(), "5555".to_string()], "4444".to_string()), 16);
}

#[test]
fn test_58() {
    assert_eq!(Solution::open_lock(vec!["0002".to_string(), "0020".to_string(), "0200".to_string(), "2000".to_string(), "0013".to_string(), "0130".to_string(), "1300".to_string(), "3000".to_string(), "0112".to_string(), "1120".to_string(), "1201".to_string(), "2011".to_string(), "1011".to_string(), "0110".to_string(), "0101".to_string(), "1010".to_string(), "0011".to_string()], "1111".to_string()), 4);
}

#[test]
fn test_59() {
    assert_eq!(Solution::open_lock(vec!["1111".to_string(), "1110".to_string(), "1101".to_string(), "1011".to_string(), "0111".to_string(), "2222".to_string(), "2221".to_string(), "2212".to_string(), "2122".to_string(), "1222".to_string(), "3333".to_string(), "3332".to_string(), "3323".to_string(), "3233".to_string(), "2333".to_string(), "4444".to_string(), "4443".to_string(), "4434".to_string(), "4344".to_string(), "3444".to_string()], "5555".to_string()), 20);
}

#[test]
fn test_60() {
    assert_eq!(Solution::open_lock(vec!["9998".to_string(), "9989".to_string(), "9899".to_string(), "8999".to_string(), "0001".to_string(), "0010".to_string(), "0100".to_string(), "1000".to_string()], "9999".to_string()), 4);
}

#[test]
fn test_61() {
    assert_eq!(Solution::open_lock(vec!["1110".to_string(), "1101".to_string(), "1011".to_string(), "0111".to_string(), "2220".to_string(), "2202".to_string(), "2022".to_string(), "0222".to_string(), "3330".to_string(), "3303".to_string(), "3033".to_string(), "0333".to_string()], "4444".to_string()), 16);
}

#[test]
fn test_62() {
    assert_eq!(Solution::open_lock(vec!["0009".to_string(), "0018".to_string(), "0108".to_string(), "0117".to_string(), "0126".to_string(), "0135".to_string(), "0144".to_string(), "0153".to_string(), "0162".to_string(), "0171".to_string(), "0800".to_string(), "0900".to_string(), "1800".to_string(), "1900".to_string(), "2800".to_string(), "2900".to_string(), "3800".to_string(), "3900".to_string(), "4800".to_string(), "4900".to_string(), "5800".to_string(), "5900".to_string(), "6800".to_string(), "6900".to_string(), "7800".to_string(), "7900".to_string(), "8800".to_string(), "8900".to_string(), "9800".to_string(), "9900".to_string()], "0000".to_string()), 0);
}

#[test]
fn test_63() {
    assert_eq!(Solution::open_lock(vec!["0001".to_string(), "0010".to_string(), "0100".to_string(), "1000".to_string(), "1110".to_string(), "1101".to_string(), "1011".to_string(), "0111".to_string()], "1111".to_string()), 8);
}

#[test]
fn test_64() {
    assert_eq!(Solution::open_lock(vec!["0000".to_string(), "1234".to_string(), "5678".to_string(), "9999".to_string()], "4321".to_string()), -1);
}

#[test]
fn test_65() {
    assert_eq!(Solution::open_lock(vec!["0010".to_string(), "0020".to_string(), "0030".to_string(), "0040".to_string(), "0050".to_string()], "0060".to_string()), 4);
}

#[test]
fn test_66() {
    assert_eq!(Solution::open_lock(vec!["1110".to_string(), "1101".to_string(), "1011".to_string(), "0111".to_string(), "0011".to_string(), "0101".to_string(), "0110".to_string()], "1111".to_string()), 6);
}

#[test]
fn test_67() {
    assert_eq!(Solution::open_lock(vec!["0001".to_string(), "0010".to_string(), "0100".to_string(), "1000".to_string(), "1110".to_string(), "1101".to_string(), "1011".to_string(), "0111".to_string(), "0002".to_string(), "0020".to_string()], "3333".to_string()), 14);
}

#[test]
fn test_68() {
    assert_eq!(Solution::open_lock(vec!["0011".to_string(), "1100".to_string(), "2211".to_string(), "3322".to_string(), "4433".to_string(), "5544".to_string(), "6655".to_string(), "7766".to_string(), "8877".to_string()], "9988".to_string()), 6);
}

#[test]
fn test_69() {
    assert_eq!(Solution::open_lock(vec!["0000".to_string(), "0010".to_string(), "0100".to_string(), "0110".to_string(), "1000".to_string(), "1010".to_string(), "1100".to_string(), "1110".to_string(), "0001".to_string(), "0011".to_string(), "0101".to_string(), "0111".to_string(), "1001".to_string(), "1011".to_string(), "1101".to_string()], "1111".to_string()), -1);
}

#[test]
fn test_70() {
    assert_eq!(Solution::open_lock(vec!["0010".to_string(), "0100".to_string(), "0111".to_string(), "1000".to_string(), "1001".to_string(), "1010".to_string(), "1100".to_string(), "1111".to_string(), "0001".to_string(), "0011".to_string(), "0110".to_string(), "1101".to_string(), "0101".to_string(), "1011".to_string()], "0000".to_string()), 0);
}

#[test]
fn test_71() {
    assert_eq!(Solution::open_lock(vec!["0001".to_string(), "0011".to_string(), "0111".to_string(), "1111".to_string(), "0002".to_string(), "0020".to_string(), "0200".to_string(), "2000".to_string(), "0003".to_string(), "0030".to_string(), "0300".to_string(), "3000".to_string()], "0101".to_string()), 2);
}

#[test]
fn test_72() {
    assert_eq!(Solution::open_lock(vec!["1112".to_string(), "1121".to_string(), "1211".to_string(), "2111".to_string(), "2221".to_string(), "2212".to_string(), "2122".to_string(), "1222".to_string()], "2222".to_string()), 10);
}

#[test]
fn test_73() {
    assert_eq!(Solution::open_lock(vec!["8888".to_string(), "8887".to_string(), "8878".to_string(), "8788".to_string(), "7888".to_string(), "9999".to_string(), "9998".to_string(), "9989".to_string(), "9899".to_string(), "8999".to_string()], "0000".to_string()), 0);
}

#[test]
fn test_74() {
    assert_eq!(Solution::open_lock(vec!["1234".to_string(), "2345".to_string(), "3456".to_string(), "4567".to_string(), "5678".to_string(), "6789".to_string(), "7890".to_string(), "8901".to_string(), "9012".to_string()], "0123".to_string()), 6);
}

#[test]
fn test_75() {
    assert_eq!(Solution::open_lock(vec!["0010".to_string(), "0100".to_string(), "1000".to_string(), "0001".to_string(), "0002".to_string(), "0020".to_string(), "0200".to_string(), "2000".to_string()], "0003".to_string()), 5);
}

#[test]
fn test_76() {
    assert_eq!(Solution::open_lock(vec!["0000".to_string(), "0011".to_string(), "0101".to_string(), "0110".to_string(), "1001".to_string(), "1010".to_string(), "1100".to_string(), "1111".to_string(), "2222".to_string()], "3333".to_string()), -1);
}

#[test]
fn test_77() {
    assert_eq!(Solution::open_lock(vec!["0001".to_string(), "0009".to_string(), "0010".to_string(), "0090".to_string(), "0100".to_string(), "0900".to_string(), "1000".to_string(), "9000".to_string(), "0002".to_string(), "0020".to_string(), "0200".to_string(), "2000".to_string(), "0008".to_string(), "0080".to_string(), "0800".to_string(), "8000".to_string()], "0000".to_string()), 0);
}

#[test]
fn test_78() {
    assert_eq!(Solution::open_lock(vec!["0009".to_string(), "0018".to_string(), "0027".to_string(), "0036".to_string(), "0045".to_string(), "0054".to_string(), "0063".to_string(), "0072".to_string(), "0081".to_string()], "0090".to_string()), 1);
}

#[test]
fn test_79() {
    assert_eq!(Solution::open_lock(vec!["8888".to_string(), "9999".to_string(), "0000".to_string(), "1111".to_string(), "2222".to_string(), "3333".to_string(), "4444".to_string(), "5555".to_string(), "6666".to_string(), "7777".to_string()], "0001".to_string()), -1);
}

#[test]
fn test_80() {
    assert_eq!(Solution::open_lock(vec!["0101".to_string(), "1010".to_string(), "1111".to_string(), "0011".to_string(), "1100".to_string(), "1001".to_string(), "0110".to_string(), "0001".to_string(), "0010".to_string(), "0100".to_string()], "9999".to_string()), 4);
}

#[test]
fn test_81() {
    assert_eq!(Solution::open_lock(vec!["0000".to_string(), "1111".to_string(), "2222".to_string(), "3333".to_string(), "4444".to_string(), "5555".to_string(), "6666".to_string(), "7777".to_string(), "8888".to_string(), "9999".to_string()], "1234".to_string()), -1);
}

#[test]
fn test_82() {
    assert_eq!(Solution::open_lock(vec!["0000".to_string(), "1111".to_string(), "2222".to_string(), "3333".to_string(), "4444".to_string(), "5555".to_string(), "6666".to_string(), "7777".to_string(), "8888".to_string()], "9999".to_string()), -1);
}

#[test]
fn test_83() {
    assert_eq!(Solution::open_lock(vec!["0001".to_string(), "1112".to_string(), "2223".to_string(), "3334".to_string(), "4445".to_string(), "5556".to_string(), "6667".to_string(), "7778".to_string(), "8889".to_string()], "4321".to_string()), 10);
}

#[test]
fn test_84() {
    assert_eq!(Solution::open_lock(vec!["0001".to_string(), "0002".to_string(), "0003".to_string(), "0004".to_string(), "0005".to_string(), "0006".to_string(), "0007".to_string(), "0008".to_string(), "0009".to_string()], "1000".to_string()), 1);
}

#[test]
fn test_85() {
    assert_eq!(Solution::open_lock(vec!["1111".to_string(), "2222".to_string(), "3333".to_string(), "4444".to_string(), "5555".to_string(), "6666".to_string(), "7777".to_string(), "8888".to_string()], "0000".to_string()), 0);
}

#[test]
fn test_86() {
    assert_eq!(Solution::open_lock(vec!["0000".to_string(), "0010".to_string(), "0020".to_string(), "0030".to_string(), "0040".to_string(), "0050".to_string(), "0060".to_string(), "0070".to_string(), "0080".to_string(), "0090".to_string()], "0009".to_string()), -1);
}

#[test]
fn test_87() {
    assert_eq!(Solution::open_lock(vec!["0123".to_string(), "4567".to_string(), "8901".to_string(), "2345".to_string(), "6789".to_string(), "0987".to_string(), "5432".to_string(), "1098".to_string(), "3210".to_string()], "1111".to_string()), 4);
}

#[test]
fn test_88() {
    assert_eq!(Solution::open_lock(vec!["1110".to_string(), "1101".to_string(), "1011".to_string(), "0111".to_string(), "2222".to_string(), "3333".to_string(), "4444".to_string(), "5555".to_string(), "6666".to_string(), "7777".to_string()], "1234".to_string()), 10);
}

#[test]
fn test_89() {
    assert_eq!(Solution::open_lock(vec!["0123".to_string(), "1234".to_string(), "2345".to_string(), "3456".to_string(), "4567".to_string(), "5678".to_string(), "6789".to_string(), "7890".to_string()], "9876".to_string()), 10);
}

#[test]
fn test_90() {
    assert_eq!(Solution::open_lock(vec!["9998".to_string(), "9989".to_string(), "9899".to_string(), "8999".to_string(), "0000".to_string(), "0001".to_string(), "0010".to_string(), "0100".to_string(), "1000".to_string()], "1111".to_string()), -1);
}

#[test]
fn test_91() {
    assert_eq!(Solution::open_lock(vec!["9998".to_string(), "9989".to_string(), "9899".to_string(), "8999".to_string(), "0001".to_string(), "0010".to_string(), "0100".to_string(), "1000".to_string()], "0000".to_string()), 0);
}

#[test]
fn test_92() {
    assert_eq!(Solution::open_lock(vec!["1234".to_string(), "2345".to_string(), "3456".to_string(), "4567".to_string(), "5678".to_string(), "6789".to_string(), "7890".to_string(), "8901".to_string(), "9012".to_string(), "0123".to_string()], "5678".to_string()), 14);
}

#[test]
fn test_93() {
    assert_eq!(Solution::open_lock(vec!["1234".to_string(), "4321".to_string(), "2345".to_string(), "5432".to_string(), "3456".to_string(), "6543".to_string(), "4567".to_string(), "7654".to_string(), "5678".to_string(), "8765".to_string()], "9876".to_string()), 10);
}

#[test]
fn test_94() {
    assert_eq!(Solution::open_lock(vec!["1234".to_string(), "2345".to_string(), "3456".to_string(), "4567".to_string(), "5678".to_string(), "6789".to_string(), "7890".to_string(), "8901".to_string(), "9012".to_string(), "0123".to_string()], "5432".to_string()), 14);
}

#[test]
fn test_95() {
    assert_eq!(Solution::open_lock(vec!["0101".to_string(), "1010".to_string(), "1100".to_string(), "1110".to_string(), "1101".to_string(), "0111".to_string(), "1001".to_string(), "0011".to_string()], "1111".to_string()), 6);
}

#[test]
fn test_96() {
    assert_eq!(Solution::open_lock(vec!["0123".to_string(), "1230".to_string(), "2301".to_string(), "3012".to_string(), "1023".to_string(), "0231".to_string(), "2310".to_string(), "3102".to_string(), "2013".to_string(), "0132".to_string(), "1320".to_string(), "3201".to_string(), "0213".to_string(), "2130".to_string(), "3021".to_string(), "1203".to_string()], "9999".to_string()), 4);
}

#[test]
fn test_97() {
    assert_eq!(Solution::open_lock(vec!["1110".to_string(), "1101".to_string(), "1011".to_string(), "0111".to_string(), "1112".to_string(), "1121".to_string(), "1211".to_string(), "2111".to_string(), "1113".to_string(), "1131".to_string(), "1311".to_string(), "3111".to_string()], "1114".to_string()), 7);
}

#[test]
fn test_98() {
    assert_eq!(Solution::open_lock(vec!["0123".to_string(), "1234".to_string(), "2345".to_string(), "3456".to_string(), "4567".to_string(), "5678".to_string(), "6789".to_string(), "7890".to_string(), "8901".to_string(), "9012".to_string(), "0013".to_string(), "0103".to_string(), "0130".to_string(), "1030".to_string(), "1300".to_string(), "3000".to_string()], "0000".to_string()), 0);
}

#[test]
fn test_99() {
    assert_eq!(Solution::open_lock(vec!["0001".to_string(), "0012".to_string(), "0102".to_string(), "0201".to_string(), "0210".to_string(), "1002".to_string(), "1020".to_string(), "1200".to_string(), "2001".to_string(), "2010".to_string(), "2100".to_string()], "2222".to_string()), 8);
}

#[test]
fn test_100() {
    assert_eq!(Solution::open_lock(vec!["1000".to_string(), "2000".to_string(), "3000".to_string(), "4000".to_string(), "5000".to_string(), "6000".to_string(), "7000".to_string(), "8000".to_string(), "9000".to_string()], "0000".to_string()), 0);
}

#[test]
fn test_101() {
    assert_eq!(Solution::open_lock(vec!["1111".to_string(), "2222".to_string(), "3333".to_string(), "4444".to_string(), "5555".to_string(), "6666".to_string(), "7777".to_string(), "8888".to_string(), "9999".to_string()], "8888".to_string()), 8);
}

#[test]
fn test_102() {
    assert_eq!(Solution::open_lock(vec!["0101".to_string(), "1212".to_string(), "2323".to_string(), "3434".to_string(), "4545".to_string(), "5656".to_string(), "6767".to_string(), "7878".to_string(), "8989".to_string(), "9090".to_string()], "9090".to_string()), 2);
}

#[test]
fn test_103() {
    assert_eq!(Solution::open_lock(vec!["0000".to_string(), "1111".to_string(), "2222".to_string(), "3333".to_string(), "4444".to_string(), "5555".to_string(), "6666".to_string(), "7777".to_string(), "8888".to_string(), "9999".to_string()], "0000".to_string()), 0);
}

#[test]
fn test_104() {
    assert_eq!(Solution::open_lock(vec!["0009".to_string(), "0090".to_string(), "0900".to_string(), "9000".to_string(), "1111".to_string()], "2222".to_string()), 8);
}

#[test]
fn test_105() {
    assert_eq!(Solution::open_lock(vec!["0001".to_string(), "0010".to_string(), "0100".to_string(), "1000".to_string(), "0011".to_string(), "0110".to_string(), "1100".to_string(), "1010".to_string(), "0101".to_string(), "1001".to_string(), "0111".to_string(), "1110".to_string(), "1101".to_string(), "1011".to_string(), "1111".to_string()], "0000".to_string()), 0);
}

#[test]
fn test_106() {
    assert_eq!(Solution::open_lock(vec!["0001".to_string(), "0011".to_string(), "0111".to_string(), "1111".to_string(), "2222".to_string(), "3333".to_string(), "4444".to_string(), "5555".to_string(), "6666".to_string(), "7777".to_string(), "8888".to_string(), "9999".to_string()], "0000".to_string()), 0);
}

#[test]
fn test_107() {
    assert_eq!(Solution::open_lock(vec!["1110".to_string(), "2220".to_string(), "3330".to_string(), "4440".to_string(), "5550".to_string(), "6660".to_string(), "7770".to_string(), "8880".to_string(), "9990".to_string()], "0001".to_string()), 1);
}

#[test]
fn test_108() {
    assert_eq!(Solution::open_lock(vec!["1111".to_string(), "2222".to_string(), "3333".to_string(), "4444".to_string(), "5555".to_string()], "6666".to_string()), 16);
}

#[test]
fn test_109() {
    assert_eq!(Solution::open_lock(vec!["1001".to_string(), "2002".to_string(), "3003".to_string(), "4004".to_string(), "5005".to_string(), "6006".to_string(), "7007".to_string(), "8008".to_string(), "9009".to_string()], "9999".to_string()), 4);
}

#[test]
fn test_110() {
    assert_eq!(Solution::open_lock(vec!["1234".to_string(), "2345".to_string(), "3456".to_string(), "4567".to_string(), "5678".to_string(), "6789".to_string(), "7890".to_string(), "8901".to_string(), "9012".to_string()], "0000".to_string()), 0);
}
