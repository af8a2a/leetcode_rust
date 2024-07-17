use std::cell::RefCell;
use std::rc::Rc;

use crate::{Solution, TreeNode};
///https://leetcode.cn/problems/symmetric-tree/description/
impl Solution {
    fn is_symmetric_helper(
        left: &Option<Rc<RefCell<TreeNode>>>,
        right: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (left, right) {
            (None, None) => true,
            (None, Some(_)) => false,
            (Some(_), None) => false,
            (Some(a), Some(b)) => {
                let a_val = a.borrow().val;
                let b_val = b.borrow().val;
                a_val == b_val
                    && Self::is_symmetric_helper(&a.borrow().left, &b.borrow().right)
                    && Self::is_symmetric_helper(&a.borrow().right, &b.borrow().left)
            }
        }
    }
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            Self::is_symmetric_helper(&root.borrow().left, &root.borrow().right)
        } else {
            false
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::tree;
    use crate::to_tree;
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(true,Solution::is_symmetric(tree![1, 2, 2, 3, 4, 4, 3]));
    }
    #[test]
    fn test_2() {
        assert_eq!(
            false,
            Solution::is_symmetric(tree![1, 2, 2, null, 3, null, 3]),
        );
    }
}
