include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::reverse_bits(0), 0);
}

#[test]
fn test_2() {
    assert_eq!(Solution::reverse_bits(11111111111111111111111111111111), 3817748707);
}

#[test]
fn test_3() {
    assert_eq!(Solution::reverse_bits(0), 0);
}

#[test]
fn test_4() {
    assert_eq!(Solution::reverse_bits(10000000000000000000000000000000), 1);
}

#[test]
fn test_5() {
    assert_eq!(Solution::reverse_bits(2147483648), 1);
}

#[test]
fn test_6() {
    assert_eq!(Solution::reverse_bits(43261596), 964176192);
}

#[test]
fn test_7() {
    assert_eq!(Solution::reverse_bits(4294967293), 3221225471);
}

#[test]
fn test_8() {
    assert_eq!(Solution::reverse_bits(1), 2147483648);
}

#[test]
fn test_9() {
    assert_eq!(Solution::reverse_bits(11111111111111111111111111111101), 3180214499);
}

#[test]
fn test_10() {
    assert_eq!(Solution::reverse_bits(10100101001010010100101001010010), 1524246408);
}

#[test]
fn test_11() {
    assert_eq!(Solution::reverse_bits(11110000111100001111000011110000), 242192076);
}

#[test]
fn test_12() {
    assert_eq!(Solution::reverse_bits(11110000000011111111000000001111), 3928946530);
}

#[test]
fn test_13() {
    assert_eq!(Solution::reverse_bits(11111111111111110000000000000000), 57352);
}

#[test]
fn test_14() {
    assert_eq!(Solution::reverse_bits(11011011011011011011011011011011), 3280443572);
}

#[test]
fn test_15() {
    assert_eq!(Solution::reverse_bits(10011100100001011011100110000110), 2012287233);
}

#[test]
fn test_16() {
    assert_eq!(Solution::reverse_bits(11100000111000001110000011100000), 117259735);
}

#[test]
fn test_17() {
    assert_eq!(Solution::reverse_bits(11100001111000011110000111100001), 2248527424);
}

#[test]
fn test_18() {
    assert_eq!(Solution::reverse_bits(10010010010010010010010010010010), 1501692052);
}

#[test]
fn test_19() {
    assert_eq!(Solution::reverse_bits(10101010101010100101010101010101), 2910733234);
}

#[test]
fn test_20() {
    assert_eq!(Solution::reverse_bits(10101010101010101010101010101010), 1221553761);
}

#[test]
fn test_21() {
    assert_eq!(Solution::reverse_bits(11111111000000001111111100000000), 15193787);
}

#[test]
fn test_22() {
    assert_eq!(Solution::reverse_bits(11110000000000000000000000000000), 14);
}

#[test]
fn test_23() {
    assert_eq!(Solution::reverse_bits(10001000100010001000100010001000), 373324907);
}

#[test]
fn test_24() {
    assert_eq!(Solution::reverse_bits(11001100110011001100110011001100), 815197725);
}
