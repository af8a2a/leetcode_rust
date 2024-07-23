use std::{cell::RefCell, rc::Rc};

mod two_sum;
mod pow;
mod next_permutation;
mod generate_parenthesis;
mod merge_two_lists;
mod climb_stairs;
mod inorder_traversal;
mod sum_of_the_digits_of_harshad_number;
mod add_two_numbers;
mod is_valid;
mod search_range;
mod max_sub_array;
mod can_jump;
mod my_atoi;
mod add_binary;
mod permute;
mod coin_change;
mod pivot_index;
mod combination_sum;
mod first_missing_positive;
mod trap;
mod is_symmetric;
mod flatten;
mod max_profit;
mod merge;
mod subarray_sum;
mod remove_nth_from_end;
mod rotate;
pub struct Solution;


#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
  #[inline]
  fn from_vec(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = ListNode::new(0);
    let mut tail = &mut head;
    for num in nums {
      tail.next = Some(Box::new(ListNode::new(num)));
      tail = tail.next.as_mut().unwrap();
    }
    head.next
  }
}


#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
///from https://github.com/aylei/leetcode-rust/blob/master/src/util/tree.rs
pub fn to_tree(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    use std::collections::VecDeque;
    let head = Some(Rc::new(RefCell::new(TreeNode::new(vec[0].unwrap()))));
    let mut queue = VecDeque::new();
    queue.push_back(head.as_ref().unwrap().clone());

    for children in vec[1..].chunks(2) {
        let parent = queue.pop_front().unwrap();
        if let Some(v) = children[0] {
            parent.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(v))));
            queue.push_back(parent.borrow().left.as_ref().unwrap().clone());
        }
        if children.len() > 1 {
            if let Some(v) = children[1] {
                parent.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(v))));
                queue.push_back(parent.borrow().right.as_ref().unwrap().clone());
            }
        }
    }
    head
}
///from https://github.com/aylei/leetcode-rust/blob/master/src/util/tree.rs
#[macro_export]
macro_rules! tree {
    () => {
        None
    };
    ($($e:expr),*) => {
        {
            let vec = vec![$(stringify!($e)), *];
            let vec = vec.into_iter().map(|v| v.parse::<i32>().ok()).collect::<Vec<_>>();
            to_tree(vec)
        }
    };
    ($($e:expr,)*) => {(tree![$($e),*])};
}