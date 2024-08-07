use std::collections::HashSet;

use crate::Solution;
///https://leetcode.cn/problems/word-break/
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let set: HashSet<String> = HashSet::from_iter(word_dict.into_iter());

        let mut dp = vec![false; s.len() + 1];

        dp[0] = true;

        for i in 1..=s.len() {
            for j in 0..i {
                if dp[j] && set.contains(&s[j..i]) {
                    dp[i] = true;
                    break;
                }
            }
        }
        dp[s.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(
            Solution::word_break(
                "leetcode".to_string(),
                vec!["leet".to_string(), "code".to_string()]
            ),
            true
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            Solution::word_break(
                "applepenapple".to_string(),
                vec!["apple".to_string(), "pen".to_string()]
            ),
            true
        );
    }
    #[test]
    fn test_3() {
        assert_eq!(
            Solution::word_break(
                "catsandog".to_string(),
                vec![
                    "cats".to_string(),
                    "dog".to_string(),
                    "sand".to_string(),
                    "and".to_string(),
                    "cat".to_string()
                ]
            ),
            false
        );
    }
}
