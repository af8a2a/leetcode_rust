use crate::Solution;
///https://leetcode.cn/problems/first-missing-positive/
impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let l = nums.len() as i32;
        for i in 0..nums.len() {
            let mut n = nums[i];
            while n > 0 && n <= l && nums[(n-1) as usize] != n {
                std::mem::swap(&mut nums[(n-1) as usize], &mut n);
            }

        }

        for i in 0..l {
            if nums[i as usize] != i + 1 {
                return i + 1;
            }
        }
        l + 1       
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::first_missing_positive(vec![1,2,0]), 3);
    }   
    #[test]
    fn test_2() {
        assert_eq!(Solution::first_missing_positive(vec![3,4,-1,1]), 2);
    }   
    #[test]
    fn test_3() {
        assert_eq!(Solution::first_missing_positive(vec![7,8,9,11,12]), 1);
    }   
    #[test]
    fn test_4() {
        assert_eq!(Solution::first_missing_positive(vec![1]), 2);
    }   
    #[test]
    fn test_5() {
        assert_eq!(Solution::first_missing_positive(vec![1,1]), 2);
    }
}