use core::net;

use crate::{ListNode, Solution};
//https://leetcode.cn/problems/remove-nth-node-from-end-of-list/description/
impl Solution {
    pub fn travel(cur: &mut Box<ListNode>, n: i32) -> i32 {
        if let Some(child) = cur.next.as_mut() {
            let num = 1 + Self::travel(child, n);
            if num == n {
                cur.next = child.next.take();
            }
            return num;
        }
        return 0;
    }
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut noop = ListNode::new(-1);
        noop.next = head;
        let mut noop = Box::new(noop);
        Self::travel(&mut noop, n);
        noop.next
    }

}
#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::remove_nth_from_end(ListNode::from_vec(vec![1,2,3,4,5]), 2), ListNode::from_vec(vec![1,2,3,5]));
    }
    
}