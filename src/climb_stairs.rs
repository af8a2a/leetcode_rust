use crate::Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut dp = vec![0; n as usize + 3];
        dp[0] = 1;
        dp[1] = 1;
        dp[2] = 2;
        for i in 3..=n as usize {
            dp[i]=dp[i-1]+dp[i-2];
        }
        dp[n as usize].clone()
    }
    fn climb_stairs_rescursive(n: i32) -> i32 {
        if n <= 2 {
            return n;
        }
        Self::climb_stairs_rescursive(n - 1) + Self::climb_stairs_rescursive(n - 2)
    }
}

#[cfg(test)]    
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::climb_stairs(3), 3);
    }
    #[test]
    fn test_2() {
        assert_eq!(Solution::climb_stairs(2), 2);
    }
}
