use crate::Solution;

impl Solution {
    fn helper(n: i32, m: i32, s: String, result: &mut Vec<String>) {
        if n > 0 {
            Self::helper(n - 1, m, s.clone() + "(", result);
        }
        if m > 0&&n<m {
            Self::helper(n, m - 1, s.clone() + ")", result);
        }
        if n == 0 && m == 0 {
            result.push(s);
        }
    }
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res=vec![];
        Self::helper(n.clone(), n.clone(), String::new(), &mut res);
        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::generate_parenthesis(3), vec!["((()))","(()())","(())()","()(())","()()()"]);
    }
    #[test]
    fn test_2(){
        assert_eq!(Solution::generate_parenthesis(1), vec!["()"]);
    }
}