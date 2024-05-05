#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_sum = std::i32::MIN;
        let mut max_level = 0;
        let mut level = 0;
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root);
        while !queue.is_empty() {
            level += 1;
            let mut sum = 0;
            let mut size = queue.len();
            for _ in 0..size {
                if let Some(Some(node)) = queue.pop_front() {
                    let node = node.borrow();
                    sum += node.val;
                    if node.left.is_some() {
                        queue.push_back(node.left.clone());
                    }
                    if node.right.is_some() {
                        queue.push_back(node.right.clone());
                    }
                }
            }
            if sum > max_sum {
                max_sum = sum;
                max_level = level;
            }
        }
        max_level
    }
}
