use crate::Solution;
///https://leetcode.cn/problems/add-binary/description/
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut left = a.chars().rev();
        let mut right = b.chars().rev();
        let mut carry = Option::<()>::None;
        let mut result = String::new();
        loop {
            
            let lhs_iter = left.next();
            let rhs_iter = right.next();
            if let (None, None,None) = (lhs_iter, rhs_iter,carry) {
                return result.chars().rev().collect();
            }

            let lhs=lhs_iter.unwrap_or('0');
            let rhs=rhs_iter.unwrap_or('0');

            match (lhs, rhs, carry) {
                ('0', '0', None) => {
                    result.push('0');
                    carry = None;

                }
                ('0', '0', Some(_)) => {
                    result.push('1');
                    carry = None;
                }
                ('0', '1', None) => {
                    result.push('1');
                }
                ('0', '1', Some(_)) => {
                    result.push('0');
                    carry = Some(());
                }
                ('1', '0', None) => {
                    result.push('1');
                }
                ('1', '0', Some(_)) => {
                    result.push('0');
                }
                ('1', '1', None) => {
                    result.push('0');
                    carry = Some(());
                }
                ('1', '1', Some(_)) => {
                    result.push('1');
                }
                _ => unreachable!(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(
            Solution::add_binary("1010".to_string(), "1011".to_string()),
            "10101".to_string()
        )
    }
    #[test]
    fn test_2() {
        assert_eq!(
            Solution::add_binary("100".to_string(), "110010".to_string()),
            "110110".to_string()
        )
    }
}
