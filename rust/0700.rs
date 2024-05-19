impl Solution {
    pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let n = node.borrow();
            if n.val == val {
                return Some(node.clone());
            } else {
                Self::search_bst(n.left.clone(), val).or_else(|| Self::search_bst(n.right.clone(), val))
            }
        } else {
            None
        }
    }
}
