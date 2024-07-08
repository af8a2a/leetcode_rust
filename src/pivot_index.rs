use crate::Solution;
///https://leetcode.cn/problems/find-pivot-index/?envType=daily-question&envId=2024-07-08
impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let sum = nums.iter().sum::<i32>();
        let mut left = 0;
        for i in 0..nums.len() {
            if left == (sum - left - nums[i]) {
                return i as i32;
            }
            left += nums[i];
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
    }
    #[test]
    fn test_2() {
        assert_eq!(Solution::pivot_index(vec![1, 2, 3]), -1);
    }
    #[test]
    fn test_3() {
        assert_eq!(Solution::pivot_index(vec![2, 1, -1]), 0);
    }
}
