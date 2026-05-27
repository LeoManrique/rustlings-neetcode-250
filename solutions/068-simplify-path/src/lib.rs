pub struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack: Vec<&str> = Vec::new();
        for part in path.split('/') {
            match part {
                "" | "." => {}
                ".." => {
                    stack.pop();
                }
                other => stack.push(other),
            }
        }
        if stack.is_empty() {
            "/".to_string()
        } else {
            let mut out = String::with_capacity(path.len());
            for s in &stack {
                out.push('/');
                out.push_str(s);
            }
            out
        }
    }
}
