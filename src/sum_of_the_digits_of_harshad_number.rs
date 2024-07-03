use crate::Solution;
///https://leetcode.cn/problems/harshad-number/?envType=daily-question&envId=2024-07-03
impl Solution {
    pub fn sum_of_the_digits_of_harshad_number(x: i32) -> i32 { 
        let sum=x.to_string().chars().map(|c| c.to_digit(10).unwrap() as i32).sum();
        if x%sum==0 {
            sum
        } else {
            -1
        }
    }   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::sum_of_the_digits_of_harshad_number(18), 9);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::sum_of_the_digits_of_harshad_number(23), -1);   
    }
}
