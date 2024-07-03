use crate::{ListNode, Solution};
///https://leetcode.cn/problems/add-two-numbers/description/
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result = None;
        let mut tail = &mut result;
        let mut t = (l1, l2, 0, 0); // (list1, list2, sum, carry)
        loop {
            t = match t {
                (None, None, _, 0) => break,
                (None, None, _, carry) => (None, None, carry, 0),
                (Some(list), None, _, carry) | (None, Some(list), _, carry)
                    if list.val + carry >= 10 =>
                {
                    (list.next, None, list.val + carry - 10, 1)
                }
                (Some(list), None, _, carry) | (None, Some(list), _, carry) => {
                    (list.next, None, list.val + carry, 0)
                }
                (Some(l1), Some(l2), _, carry) if l1.val + l2.val + carry >= 10 => {
                    (l1.next, l2.next, l1.val + l2.val + carry - 10, 1)
                }
                (Some(l1), Some(l2), _, carry) => (l1.next, l2.next, l1.val + l2.val + carry, 0),
            };

            *tail = Some(Box::new(ListNode::new(t.2)));
            tail = &mut tail.as_mut().unwrap().next;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let list1 = ListNode::from_vec(vec![2, 4, 3]);
        let list2 = ListNode::from_vec(vec![5, 6, 4]);
        assert_eq!(
            Solution::add_two_numbers(list1, list2),
            ListNode::from_vec(vec![7, 0, 8])
        );
    }

    #[test]
    fn test_2() {
        let list1 = ListNode::from_vec(vec![0]);
        let list2 = ListNode::from_vec(vec![0]);
        assert_eq!(Solution::add_two_numbers(list1, list2), ListNode::from_vec(vec![0]));
    }

    #[test]
    fn test_3(){
        let list1 = ListNode::from_vec(vec![9,9,9,9,9,9,9]);
        let list2 = ListNode::from_vec(vec![9,9,9,9]);
        assert_eq!(Solution::add_two_numbers(list1, list2), ListNode::from_vec(vec![8,9,9,9,0,0,0,1]));
    }
}
