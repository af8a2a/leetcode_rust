use crate::Solution;

impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        let mut res: i32 = 0;
        for (idx, char) in s.chars().enumerate() {
            match idx {
                0 => {
                    res += 0;
                }
                _ => {
                    let ascii_val = char as u8;
                    let prev_val = s.chars().nth(idx - 1).unwrap() as u8;
                    res += (ascii_val as i32 - prev_val as i32).abs();
                }
            }
        }
        res
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(
            13,
            Solution::score_of_string("hello".to_string())
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            50,
            Solution::score_of_string("zaz".to_string())
        );
    }

}