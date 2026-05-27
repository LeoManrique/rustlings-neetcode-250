include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::exist(vec![vec!['A', 'B'], vec!['C', 'D']], "BD".to_string()), true);
}

#[test]
fn test_2() {
    assert_eq!(Solution::exist(vec![vec!['A', 'B'], vec!['C', 'D']], "AC".to_string()), true);
}

#[test]
fn test_3() {
    assert_eq!(Solution::exist(vec![vec!['A', 'B', 'C', 'E'], vec!['S', 'F', 'C', 'S'], vec!['A', 'D', 'E', 'E']], "ABCB".to_string()), false);
}

#[test]
fn test_4() {
    assert_eq!(Solution::exist(vec![vec!['A', 'B', 'C', 'E'], vec!['S', 'F', 'C', 'S'], vec!['A', 'D', 'E', 'E']], "ASADB".to_string()), false);
}

#[test]
fn test_5() {
    assert_eq!(Solution::exist(vec![vec!['A', 'B', 'C', 'E'], vec!['S', 'F', 'C', 'S'], vec!['A', 'D', 'E', 'E']], "ABCB".to_string()), false);
}

#[test]
fn test_6() {
    assert_eq!(Solution::exist(vec![vec!['A', 'B', 'C', 'E'], vec!['S', 'F', 'E', 'S'], vec!['A', 'D', 'E', 'E']], "ABCESEEEFS".to_string()), true);
}

#[test]
fn test_7() {
    assert_eq!(Solution::exist(vec![vec!['A', 'B', 'C', 'E'], vec!['S', 'F', 'C', 'S'], vec!['A', 'D', 'E', 'E']], "Z".to_string()), false);
}

#[test]
fn test_8() {
    assert_eq!(Solution::exist(vec![vec!['A', 'B', 'C', 'E'], vec!['S', 'F', 'C', 'S'], vec!['A', 'D', 'E', 'E']], "AB".to_string()), true);
}

#[test]
fn test_9() {
    assert_eq!(Solution::exist(vec![vec!['A', 'B', 'C', 'E'], vec!['S', 'F', 'C', 'S'], vec!['A', 'D', 'E', 'E']], "ABCCED".to_string()), true);
}

#[test]
fn test_10() {
    assert_eq!(Solution::exist(vec![vec!['A', 'B', 'C', 'E'], vec!['S', 'F', 'C', 'S'], vec!['A', 'D', 'E', 'E']], "SEE".to_string()), true);
}

#[test]
fn test_11() {
    assert_eq!(Solution::exist(vec![vec!['A', 'B', 'C', 'E'], vec!['S', 'F', 'C', 'S'], vec!['A', 'D', 'E', 'E']], "E".to_string()), true);
}

#[test]
fn test_12() {
    assert_eq!(Solution::exist(vec![vec!['A', 'B', 'C', 'E'], vec!['S', 'F', 'C', 'S'], vec!['A', 'D', 'E', 'E']], "SEE".to_string()), true);
}

#[test]
fn test_13() {
    assert_eq!(Solution::exist(vec![vec!['A', 'B'], vec!['C', 'D']], "AB".to_string()), true);
}

#[test]
fn test_14() {
    assert_eq!(Solution::exist(vec![vec!['A', 'B'], vec!['C', 'D']], "CD".to_string()), true);
}

#[test]
fn test_15() {
    assert_eq!(Solution::exist(vec![vec!['A', 'B', 'C', 'E'], vec!['S', 'F', 'C', 'S'], vec!['A', 'D', 'E', 'E']], "ASAD".to_string()), true);
}

#[test]
fn test_16() {
    assert_eq!(Solution::exist(vec![vec!['A']], "A".to_string()), true);
}

#[test]
fn test_17() {
    assert_eq!(Solution::exist(vec![vec!['A', 'B', 'C', 'E'], vec!['S', 'F', 'C', 'S'], vec!['A', 'D', 'E', 'E']], "ABCCED".to_string()), true);
}
