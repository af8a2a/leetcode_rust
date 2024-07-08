use crate::Solution;
///https://leetcode.cn/problems/combination-sum/description/
impl Solution {
    fn combination_sum_helper(candidates: Vec<i32>, target: i32, path: &mut Vec<i32>,start_pos:usize, result: &mut Vec<Vec<i32>>) {
        if target==0{
            result.push(path.clone());
            return;
        }
        for i in start_pos..candidates.len(){
            if target-candidates[i]>=0{
                path.push(candidates[i]);
                Self::combination_sum_helper(candidates.clone(),target-candidates[i],path,i,result);
                path.pop();
            }
        }
    }
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut path = vec![];
        let mut result = vec![];
        Self::combination_sum_helper(candidates,target,&mut path,0,&mut result);
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::combination_sum(vec![2, 3, 6, 7], 7), vec![vec![2, 2, 3], vec![7]]);
    }   
    #[test]
    fn test_2() {
        assert_eq!(Solution::combination_sum(vec![2,3,5], 8), vec![vec![2,2,2,2], vec![2,3,3],vec![3,5]]);
    }   
    #[test]
    fn test_3() {
        assert_eq!(Solution::combination_sum(vec![2], 1), Vec::<Vec<i32>>::new());
    }   

}