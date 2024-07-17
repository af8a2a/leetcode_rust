use std::{cell::RefCell, rc::Rc};

use crate::{Solution, TreeNode};
///https://leetcode.cn/problems/flatten-binary-tree-to-linked-list/description/
impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut curr = root.as_ref().cloned();
        while let Some(curr_node) = curr {
            let mut curr_node = curr_node.borrow_mut();
            if let Some(next_node) = curr_node.left.take() {
                let mut predecessor = next_node.clone();
                let mut predecessor_right = predecessor.borrow().right.clone();
                while let Some(node) = predecessor_right {
                    predecessor_right = node.borrow().right.clone();
                    predecessor = node;
                }
                predecessor.borrow_mut().right = curr_node.right.take();
                curr_node.right = Some(next_node);
            }
            curr = curr_node.right.clone();
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
        let mut res=tree![1,2,5,3,4,null,6];
        Solution::flatten(&mut res);
        assert_eq!(tree![1,null,2,null,3,null,4,null,5,null,6],res);
    }
    #[test]
    fn test_2() {
        let mut res=tree![];
        Solution::flatten(&mut res);
        assert_eq!(tree![],res);
    }
    #[test]
    fn test_3() {
        let mut res=tree![0];
        Solution::flatten(&mut res);
        assert_eq!(tree![0],res);
    }
}
