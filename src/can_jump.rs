use crate::Solution;
impl Solution {
    /// https://leetcode.cn/problems/jump-game/description/
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut step:i32 = 0;
        for i in 0..nums.len() as i32 {
            if i>step{
                return false;
            }
            step = step.max(nums[i as usize] + i);
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
    }
    #[test]
    fn test_2() {
        assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
    }
    #[test]
    fn test_3() {
        assert_eq!(Solution::can_jump(vec![0,1]), false);
    }
}
