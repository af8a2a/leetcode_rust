use crate::{ListNode, Solution};

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (None, Some(l2)) => Some(l2),
            (Some(l1), None) => Some(l1),
            (Some(mut l1), Some(mut l2)) => {
                if l1.val < l2.val {
                    l1.next = Self::merge_two_lists(l1.next, Some(l2));
                    Some(l1)
                } else {
                    l2.next = Self::merge_two_lists(Some(l1), l2.next);
                    Some(l2)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let list1 = ListNode::from_vec(vec![1, 2, 4]);
        let list2 = ListNode::from_vec(vec![1, 3, 4]);
        assert_eq!(Solution::merge_two_lists(list1, list2), ListNode::from_vec(vec![1, 1, 2, 3, 4, 4]));
    }

    #[test]
    fn test_2() {
        let list1 = ListNode::from_vec(vec![]);
        let list2 = ListNode::from_vec(vec![]);
        assert_eq!(Solution::merge_two_lists(list1, list2), ListNode::from_vec(vec![]));
    }

    #[test]
    fn test_3(){
        let list1 = ListNode::from_vec(vec![]);
        let list2 = ListNode::from_vec(vec![0]);
        assert_eq!(Solution::merge_two_lists(list1, list2), ListNode::from_vec(vec![0]));
    }
}
