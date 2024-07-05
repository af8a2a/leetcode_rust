use crate::Solution;
///https://leetcode.cn/problems/maximum-subarray/description/
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut ans=nums[0];
        let mut prefix=0;
        for i in nums{
            if prefix>0{
                prefix+=i;
            }
            if prefix<=0{
                prefix=i;
            }
            ans=ans.max(prefix);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(6, Solution::max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]));
    }
    #[test]
    fn test_2() {
        assert_eq!(1, Solution::max_sub_array(vec![1]));
    }
    #[test]
    fn test_3() {
        assert_eq!(23, Solution::max_sub_array(vec![5,4,-1,7,8]));
    }
}   