use crate::Solution;
///https://leetcode.cn/problems/string-to-integer-atoi/description/
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut sign: i64 = 1; //sign,start,sign_inverted
        let mut sum: i64 = 0;
        for (index, c) in s.trim().chars().enumerate() {
            match c {
                ' ' => {
                    break;
                }
                '+' => {
                    if index != 0 {
                        break;
                    }
                    sign = 1;
                }
                '-' => {
                    if index != 0 {
                        break;
                    }
                    sign = -1;
                }
                _ => {
                    if c.is_digit(10) {
                        sum = sum * 10 + (c.to_digit(10).unwrap() as i64);
                        if sum > i32::MAX as i64 {
                            match sign {
                                1 => return i32::MAX,
                                -1 => return i32::MIN,
                                _ => unreachable!(),
                            }
                        }
                    } else {
                        break;
                    }
                }
            }
        }

        (sum * sign) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
        assert_eq!(Solution::my_atoi("-042".to_string()), -42);
        assert_eq!(Solution::my_atoi("1337c0d3".to_string()), 1337);
        assert_eq!(Solution::my_atoi("0-1".to_string()), 0);
        assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
        assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
        assert_eq!(Solution::my_atoi("-91283472332".to_string()), -2147483648);
        assert_eq!(
            Solution::my_atoi("9223372036854775808".to_string()),
            2147483647
        );
    }
}
