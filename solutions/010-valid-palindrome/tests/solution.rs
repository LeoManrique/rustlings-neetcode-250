include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::is_palindrome("race a car".to_string()), false);
}

#[test]
fn test_2() {
    assert_eq!(Solution::is_palindrome(" ".to_string()), true);
}

#[test]
fn test_3() {
    assert_eq!(Solution::is_palindrome("Able was I ere I saw Elba".to_string()), true);
}

#[test]
fn test_4() {
    assert_eq!(Solution::is_palindrome("__Level__, __level__".to_string()), true);
}

#[test]
fn test_5() {
    assert_eq!(Solution::is_palindrome("No lemon, no melon".to_string()), true);
}

#[test]
fn test_6() {
    assert_eq!(Solution::is_palindrome("0P".to_string()), false);
}

#[test]
fn test_7() {
    assert_eq!(Solution::is_palindrome("_a!a_".to_string()), true);
}

#[test]
fn test_8() {
    assert_eq!(Solution::is_palindrome("Never odd or even".to_string()), true);
}

#[test]
fn test_9() {
    assert_eq!(Solution::is_palindrome("12345678987654321".to_string()), true);
}

#[test]
fn test_10() {
    assert_eq!(Solution::is_palindrome("Was it a car or a cat I saw?".to_string()), true);
}

#[test]
fn test_11() {
    assert_eq!(Solution::is_palindrome("No 'x' in Nixon".to_string()), true);
}

#[test]
fn test_12() {
    assert_eq!(Solution::is_palindrome("Able was I, I saw Elba".to_string()), true);
}

#[test]
fn test_13() {
    assert_eq!(Solution::is_palindrome("12321".to_string()), true);
}

#[test]
fn test_14() {
    assert_eq!(Solution::is_palindrome("Not a palindrome".to_string()), false);
}

#[test]
fn test_15() {
    assert_eq!(Solution::is_palindrome("Able was I, ere I saw Elba".to_string()), true);
}

#[test]
fn test_16() {
    assert_eq!(Solution::is_palindrome("123abcba321".to_string()), true);
}

#[test]
fn test_17() {
    assert_eq!(Solution::is_palindrome("Step on no pets".to_string()), true);
}

#[test]
fn test_18() {
    assert_eq!(Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()), true);
}

#[test]
fn test_19() {
    assert_eq!(Solution::is_palindrome("Madam, in Eden, I'm Adam".to_string()), true);
}

#[test]
fn test_20() {
    assert_eq!(Solution::is_palindrome("123abccba321".to_string()), true);
}

#[test]
fn test_21() {
    assert_eq!(Solution::is_palindrome("__^_^__".to_string()), true);
}
