mod two_sum;
mod pow;
mod next_permutation;
mod generate_parenthesis;
mod merge_two_lists;
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
