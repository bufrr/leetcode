use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        dfs(root.clone(), true, 0, &mut res);
        //dfs(root.clone(), false, 0, &mut res);
        res
    }
}

pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>, is_left: bool, mut len: i32, res:&mut i32) {
    if let Some(node) = root {
        let node = node.borrow();
        if is_left {
            dfs(node.right.clone(), false, len+1, res);
            dfs(node.left.clone(), true, 1, res);           
        } else {
            dfs(node.right.clone(), false, 1, res);
            dfs(node.left.clone(), true, len+ 1, res);
        }
        *res = (*res).max(len);
    }
}
