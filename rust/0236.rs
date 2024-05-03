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
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        dfs(root, p, q)
    }
}

pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
        return None;
    }
    if root == p || root == q {
        return root;
    }
    let left = dfs(root.as_ref().unwrap().borrow().left.clone(), p.clone(), q.clone());
    let right = dfs(root.as_ref().unwrap().borrow().right.clone(), p.clone(), q.clone());
    if left.is_none() {
        return right;
    }
    if right.is_none() {
        return left;
    }
    return root;
}

fn main() {
    println!("Hello, world!");
}
