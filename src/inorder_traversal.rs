use std::cell::RefCell;
use std::rc::Rc;

use crate::{Solution, TreeNode};
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root{
            Some(node) => {
                let left = node.borrow().left.clone();
                let right = node.borrow().right.clone();
                let mut res = vec![];
                if let Some(left) = left{
                    res.extend(Solution::inorder_traversal(Some(left)));
                }
                res.push(node.borrow().val);
                if let Some(right) = right{
                    res.extend(Solution::inorder_traversal(Some(right)));
                }
                res
            },
            None => {
                vec![]
            },
        }
        
    }
}

#[cfg(test)]    
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::inorder_traversal(None), vec![]);
    }
    #[test]
    fn test_2() {
        assert_eq!(Solution::inorder_traversal(Some(Rc::new(RefCell::new(TreeNode::new(1))))), vec![1]);
    }
}
