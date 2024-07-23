use crate::Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        for i in 0..matrix.len() {
            for i2 in 0..matrix.len() {
                let v2 = matrix[i][0];
                let v1len = matrix[i2].len() - i;
                matrix[i2].insert(v1len, v2);
                matrix[i].remove(0);
            }
        }
    }
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_1() {
        let mut matrix = vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, vec![vec![7,4,1],vec![8,5,2],vec![9,6,3]]);
    }
}