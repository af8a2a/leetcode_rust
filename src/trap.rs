use crate::Solution;
///https://leetcode.cn/problems/trapping-rain-water/
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut sum=0;
        let mut left_max=0;
        let mut right_max=0;
        let mut left=0;
        let mut right=height.len()-1;
        while left<right{
            left_max=left_max.max(height[left]);
            right_max=right_max.max(height[right]);
            if height[left]<height[right]{
                sum+=left_max-height[left];
                left+=1;
            }else{
                sum+=right_max-height[right];
                right-=1;
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]), 6);
    }   
    #[test]
    fn test_2() {
        assert_eq!(Solution::trap(vec![4,2,0,3,2,5]), 9);
    }   
}