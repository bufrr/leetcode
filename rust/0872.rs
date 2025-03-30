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
    pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, leaves: &mut Vec<i32>) {
            if root.is_none() {
                return 
            }
            let node = root.as_ref().unwrap().borrow();
            if node.left.is_none() && node.right.is_none() {
                leaves.push(node.val);
            }
            dfs(node.left.clone(), leaves);
            dfs(node.right.clone(), leaves);
        }

        let mut vec1: Vec<i32> = Vec::new();
        let mut vec2: Vec<i32> = Vec::new();

        dfs(root1, &mut vec1);
        dfs(root2, &mut vec2);

        vec1 == vec2
    }
}
