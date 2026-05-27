use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    pub val: bool,
    pub is_leaf: bool,
    pub top_left: Option<Rc<RefCell<Node>>>,
    pub top_right: Option<Rc<RefCell<Node>>>,
    pub bottom_left: Option<Rc<RefCell<Node>>>,
    pub bottom_right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: bool, is_leaf: bool) -> Self {
        Self {
            val,
            is_leaf,
            top_left: None,
            top_right: None,
            bottom_left: None,
            bottom_right: None,
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn construct(grid: Vec<Vec<i32>>) -> Option<Rc<RefCell<Node>>> {
        let n = grid.len();
        Self::build(&grid, 0, 0, n)
    }

    fn build(
        grid: &Vec<Vec<i32>>,
        row: usize,
        col: usize,
        size: usize,
    ) -> Option<Rc<RefCell<Node>>> {
        let first = grid[row][col];
        let mut uniform = true;
        'outer: for r in row..row + size {
            for c in col..col + size {
                if grid[r][c] != first {
                    uniform = false;
                    break 'outer;
                }
            }
        }
        if uniform {
            return Some(Rc::new(RefCell::new(Node::new(first == 1, true))));
        }
        let half = size / 2;
        let node = Rc::new(RefCell::new(Node::new(true, false)));
        {
            let mut n = node.borrow_mut();
            n.top_left = Self::build(grid, row, col, half);
            n.top_right = Self::build(grid, row, col + half, half);
            n.bottom_left = Self::build(grid, row + half, col, half);
            n.bottom_right = Self::build(grid, row + half, col + half, half);
        }
        Some(node)
    }
}
