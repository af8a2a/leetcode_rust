use std::collections::HashSet;

use crate::Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut hash_set = HashSet::new();
        let mut l = 0;
        let mut r = 0;
        let mut ans = 0;
        loop {
            if r >= s.len() {
                break;
            }
            if l >= s.len() {
                break;
            }

            let rhs = s.chars().nth(r).unwrap();
            match hash_set.contains(&rhs) {
                true => {
                    loop {
                        hash_set.remove(&s.chars().nth(l).unwrap());
                        l += 1;
                        if !hash_set.contains(&rhs) {
                            break;
                        }
                    }
                    hash_set.insert(rhs);
                }
                false => {
                    hash_set.insert(rhs);
                }
            }
            r += 1;
            ans = ans.max(hash_set.len());
        }
        ans as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
    }
    #[test]

    fn test2() {
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
    }
    #[test]

    fn test3() {
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }
    #[test]
    fn test4() {
        assert_eq!(Solution::length_of_longest_substring("".to_string()), 0);
    }
    #[test]
    fn test5() {
        assert_eq!(Solution::length_of_longest_substring("aab".to_string()), 2);
    }
}
