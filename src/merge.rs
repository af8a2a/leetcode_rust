use crate::Solution;
///https://leetcode.cn/problems/merge-intervals/
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut merged: Vec<Vec<i32>> = vec![];
        for range in intervals.iter() {
            let left = range[0];
            let right = range[1];
            if merged.is_empty() || merged.last().unwrap()[1] < left {
                merged.push(vec![left, right]);
            } else {
                merged.last_mut().unwrap()[1] = right.max(merged.last().unwrap()[1]);
            }
        }
        merged
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![2, 5]]),
            vec![vec![1, 5]]
        );
    }
}
