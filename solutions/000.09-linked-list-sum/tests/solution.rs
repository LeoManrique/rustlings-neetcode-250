include!("../src/lib.rs");

fn make_list(vals: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for &v in vals.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

#[test]
fn test_1() { assert_eq!(Solution::list_sum(make_list(vec![1,2,3])), 6); }
#[test]
fn test_2() { assert_eq!(Solution::list_sum(make_list(vec![])), 0); }
#[test]
fn test_3() { assert_eq!(Solution::list_sum(make_list(vec![42])), 42); }
#[test]
fn test_4() { assert_eq!(Solution::list_sum(make_list(vec![-1,1])), 0); }
#[test]
fn test_5() { assert_eq!(Solution::list_sum(make_list(vec![10,20,30])), 60); }
