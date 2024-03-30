impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, target_sum: i64) -> i32 {
            if let Some(node) = node {
                let node = node.borrow();
                let tt = target_sum - node.val as i64;
                let mut count = 0;
                if node.val as i64 == target_sum {
                    count += 1;
                }
                count += dfs(&node.left, tt);
                count += dfs(&node.right, tt);
                count
            } else {
                0
            }
        }
        if let Some(root) = root {
            let root_clone = Rc::clone(&root);
            dfs(&Some(Rc::clone(&root_clone)), target_sum as i64) + Self::path_sum(Rc::clone(&root_clone).borrow().left.clone(), target_sum) + Self::path_sum(Rc::clone(&root_clone).borrow().right.clone(), target_sum)
        } else {
            0
        }
    }
}
