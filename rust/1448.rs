// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, max_so_far: i32, good: &mut i32) {
            if root.is_none() {
                return
            }
            let node = root.as_ref().unwrap().borrow();
            if node.val >= max_so_far {
                *good += 1;
            } 
            let new_max = max_so_far.max(node.val);
            dfs(node.left.clone(), new_max, good);
            dfs(node.right.clone(), new_max, good);
        }
        if root.is_none() {
            return 0;
        }
        let mut root_val = root.as_ref().unwrap().borrow().val;
        let mut good = 0;
        dfs(root, root_val, &mut good);
        good
    }
}
