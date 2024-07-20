impl Solution {
    pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let mut node_ref = node.borrow_mut();
            if key < node_ref.val {
                node_ref.left = Self::delete_node(node_ref.left.take(), key);
            } else if key > node_ref.val {
                node_ref.right = Self::delete_node(node_ref.right.take(), key);
            } else {
                // Node to delete found
                if node_ref.left.is_none() {
                    return node_ref.right.take();
                } else if node_ref.right.is_none() {
                    return node_ref.left.take();
                } else {
                    // Node has both left and right children
                    let successor_val = Self::find_min(&node_ref.right);
                    node_ref.val = successor_val;
                    node_ref.right = Self::delete_node(node_ref.right.take(), successor_val);
                }
            }
            drop(node_ref);
            Some(node)
        } else {
            None
        }
    }

    fn find_min(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut current = node.clone();
        while let Some(n) = current {
            let n_ref = n.borrow();
            if n_ref.left.is_none() {
                return n_ref.val;
            }
            current = n_ref.left.clone();
        }
        unreachable!("The node should always have a value")
    }
}
