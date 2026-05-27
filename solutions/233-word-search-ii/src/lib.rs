pub struct Solution;

// Trie node indexed by 0..26 for lowercase ASCII.
pub struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    word_index: Option<usize>, // Index into the original `words` list when this node ends a word.
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: Default::default(),
            word_index: None,
        }
    }
}

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        if board.is_empty() || board[0].is_empty() || words.is_empty() {
            return Vec::new();
        }
        let rows = board.len();
        let cols = board[0].len();

        // Build the trie.
        let mut root = Box::new(TrieNode::new());
        for (i, w) in words.iter().enumerate() {
            let mut node = root.as_mut();
            for &b in w.as_bytes() {
                let idx = (b - b'a') as usize;
                node = node.children[idx].get_or_insert_with(|| Box::new(TrieNode::new()));
            }
            node.word_index = Some(i);
        }

        // Convert board to bytes once for fast indexing.
        let grid: Vec<Vec<u8>> = board
            .into_iter()
            .map(|row| row.into_iter().map(|c| c as u8).collect())
            .collect();
        let mut visited = vec![vec![false; cols]; rows];
        let mut found = vec![false; words.len()];

        for r in 0..rows {
            for c in 0..cols {
                dfs(&grid, r, c, &mut root, &mut visited, &mut found);
            }
        }

        // Preserve original input order.
        words
            .into_iter()
            .enumerate()
            .filter_map(|(i, w)| if found[i] { Some(w) } else { None })
            .collect()
    }
}

pub fn dfs(
    grid: &[Vec<u8>],
    r: usize,
    c: usize,
    node: &mut Box<TrieNode>,
    visited: &mut [Vec<bool>],
    found: &mut [bool],
) {
    let ch = grid[r][c];
    let idx = (ch - b'a') as usize;
    let Some(mut child) = node.children[idx].take() else {
        return;
    };

    if let Some(wi) = child.word_index.take() {
        found[wi] = true;
    }

    visited[r][c] = true;
    let rows = grid.len();
    let cols = grid[0].len();
    let dirs = [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)];
    for (dr, dc) in dirs {
        let nr = r as i32 + dr;
        let nc = c as i32 + dc;
        if nr < 0 || nc < 0 || nr >= rows as i32 || nc >= cols as i32 {
            continue;
        }
        let (nr, nc) = (nr as usize, nc as usize);
        if visited[nr][nc] {
            continue;
        }
        dfs(grid, nr, nc, &mut child, visited, found);
    }
    visited[r][c] = false;

    // Pruning: if the child has no remaining children, drop it; otherwise restore.
    let has_children = child.children.iter().any(|c| c.is_some());
    if has_children || child.word_index.is_some() {
        node.children[idx] = Some(child);
    }
}
