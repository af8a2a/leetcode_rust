use crate::Solution;
///https://leetcode.cn/problems/permutations/description/
impl Solution {
    fn permute_helper(nums: Vec<i32>, vis: &mut Vec<bool>, cur: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if cur.len() == nums.len() {
            res.push(cur.clone());
            return;
        }
        for i in 0..nums.len() {
            if !vis[i] {
                vis[i] = true;
                cur.push(nums[i]);
                Self::permute_helper(nums.clone(), vis, cur, res);
                cur.pop();
                vis[i] = false;
            }
        }
    }
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut vis = vec![false; nums.len()];
        let mut cur = vec![];
        let mut res = vec![];
        Self::permute_helper(nums, &mut vis, &mut cur, &mut res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::permute(vec![1, 2, 3]),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1],
            ]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::permute(vec![0,1]), vec![
            vec![0,1],
            vec![1,0],
        ]);
    }
}
