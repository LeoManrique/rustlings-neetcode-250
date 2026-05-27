include!("../src/lib.rs");

const INF: i32 = i32::MAX;

#[test]
fn test_canonical_example() {
    let mut rooms = vec![
        vec![INF, -1, 0, INF],
        vec![INF, INF, INF, -1],
        vec![INF, -1, INF, -1],
        vec![0, -1, INF, INF],
    ];
    Solution::walls_and_gates(&mut rooms);
    let expected = vec![
        vec![3, -1, 0, 1],
        vec![2, 2, 1, -1],
        vec![1, -1, 2, -1],
        vec![0, -1, 3, 4],
    ];
    assert_eq!(rooms, expected);
}

#[test]
fn test_all_gates() {
    let mut rooms = vec![vec![0, 0], vec![0, 0]];
    let expected = rooms.clone();
    Solution::walls_and_gates(&mut rooms);
    assert_eq!(rooms, expected);
}

#[test]
fn test_all_walls() {
    let mut rooms = vec![vec![-1, -1], vec![-1, -1]];
    let expected = rooms.clone();
    Solution::walls_and_gates(&mut rooms);
    assert_eq!(rooms, expected);
}

#[test]
fn test_single_gate_open_rooms() {
    let mut rooms = vec![vec![0, INF, INF]];
    Solution::walls_and_gates(&mut rooms);
    assert_eq!(rooms, vec![vec![0, 1, 2]]);
}

#[test]
fn test_unreachable_room() {
    // The INF in the corner is walled off entirely by -1
    let mut rooms = vec![
        vec![0, -1, INF],
        vec![-1, -1, -1],
        vec![INF, -1, INF],
    ];
    Solution::walls_and_gates(&mut rooms);
    let expected = vec![
        vec![0, -1, INF],
        vec![-1, -1, -1],
        vec![INF, -1, INF],
    ];
    assert_eq!(rooms, expected);
}

#[test]
fn test_empty_grid() {
    let mut rooms: Vec<Vec<i32>> = vec![];
    Solution::walls_and_gates(&mut rooms);
    assert!(rooms.is_empty());
}
