use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        fn pre_order(
            node: Option<Rc<RefCell<TreeNode>>>,
            target_sum: i64,
            current_sum: i64,
            m: &mut HashMap<i64, i32>,
            count: &mut i32,
        ) {
            if node.is_none() {
                return;
            }
            let node = node.unwrap();
            let node_val = node.borrow().val as i64;
            let new_sum = current_sum + node_val;

            if new_sum == target_sum {
                *count += 1;
            }

            if let Some(occur) = m.get(&(new_sum - target_sum)) {
                *count += *occur;
            }

            *m.entry(new_sum).or_insert(0) += 1;

            pre_order(node.borrow().left.clone(), target_sum, new_sum, m, count);
            pre_order(node.borrow().right.clone(), target_sum, new_sum, m, count);

            *m.get_mut(&new_sum).unwrap() -= 1;
        }

        let mut m = HashMap::new();
        let mut count = 0;
        pre_order(root, target_sum as i64, 0, &mut m, &mut count);

        count
    }
}
