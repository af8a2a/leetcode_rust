
use crate::Solution;
///https://leetcode.cn/problems/valid-parentheses/submissions/544096330/
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];

        for char in s.chars() {
            if char == '(' {
                stack.push(')');
            } else if char == '[' {
                stack.push(']');
            } else if char == '{' {
                stack.push('}');
            } else if stack.pop() != Some(char) {
                return false;
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::is_valid("()".to_string()), true);
    }
    #[test]
    fn test_2() {
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
    }
    #[test]
    fn test_3() {
        assert_eq!(Solution::is_valid("(]".to_string()), false);
    }
    #[test]
    fn test_4() {
        assert_eq!(Solution::is_valid("{[]}".to_string()), true);
    }
}
