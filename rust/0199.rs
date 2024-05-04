use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        let mut queue = vec![];
        if let Some(node) = root {
            queue.push(node);
        }
        while !queue.is_empty() {
            let mut next_level = vec![];
            let mut last = 0;
            for node in queue {
                let node = node.borrow();
                last = node.val;
                if let Some(left) = &node.left {
                    next_level.push(left.clone());
                }
                if let Some(right) = &node.right {
                    next_level.push(right.clone());
                }
            }
            res.push(last);
            queue = next_level;
        }
        res
    }
}
