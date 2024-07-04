use crate::Solution;
///https://leetcode.cn/problems/find-first-and-last-position-of-element-in-sorted-array/description/
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ans = vec![0, 0];
        if let Ok(i) = nums.binary_search(&target) {
            ans[0] = i as i32;
            ans[1] = i as i32;
        } else {
            return vec![-1, -1];
        }
        while ans[0] > 0 && nums[ans[0] as usize - 1] == target {
            ans[0] -= 1;
        }
        while ans[1] < nums.len() as i32 - 1 && nums[ans[1] as usize + 1] == target {
            ans[1] += 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(
            vec![3, 4],
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8)
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            vec![-1, -1],
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6)
        );
    }
    #[test]
    fn test_3() {
        assert_eq!(vec![-1, -1], Solution::search_range(vec![], 0));
    }
    #[test]
    fn test_4() {
        assert_eq!(vec![-1, -1], Solution::search_range(vec![1], 0));
    }
}
