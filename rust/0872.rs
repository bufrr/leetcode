use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut leaf1 = vec![];
        let mut leaf2 = vec![];
        dfs(&root1, &mut leaf1);
        dfs(&root2, &mut leaf2);
        leaf1 == leaf2
    }
}

pub fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, leaf: &mut Vec<i32>) {
    if let Some(node) = root {
        let node = node.borrow();
        if node.left.is_none() && node.right.is_none() {
            leaf.push(node.val);
        }
        dfs(&node.left, leaf);
        dfs(&node.right, leaf);
    }
}
