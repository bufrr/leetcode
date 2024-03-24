use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root, i32::MIN)
    }

    pub fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, max: i32) -> i32{
        if let Some(node) = node {
            let node = node.borrow();
            let max_till_now = std::cmp::max(node.val, max);
            Self::dfs(&node.left, max_till_now) + Self::dfs(&node.right, max_till_now) + (node.val >= max) as i32
        } else {
            0
        }
    }
}
