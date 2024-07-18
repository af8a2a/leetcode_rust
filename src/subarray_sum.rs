use std::collections::HashMap;

use crate::Solution;
///https://leetcode.cn/problems/subarray-sum-equals-k/
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut prefix = 0;
        let mut ans = 0;
        let mut hash = HashMap::new();
        hash.insert(0, 1);
        for i in nums.iter() {
            prefix += i;
            if hash.contains_key(&(prefix - k)) {
                ans += hash[&(prefix - k)];
            }
            let x = hash.entry(prefix).or_insert(0);
            *x += 1;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
    }
    #[test]
    fn test_2() {
        assert_eq!(Solution::subarray_sum(vec![1, 2, 3], 3), 2);
    }
    #[test]
    fn test_3() {
        assert_eq!(Solution::subarray_sum(vec![1], 0), 0);
    }
}
