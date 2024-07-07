use crate::Solution;
///https://leetcode.cn/problems/coin-change/description/
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![amount + 1; (amount + 1) as usize];
        dp[0] = 0;
        for i in 0..coins.len() {
            for j in 1..=amount as usize {
                if coins[i] <= j as i32 {
                    dp[j] = dp[j].min(dp[j - coins[i] as usize] + 1)
                }
            }
        }
        if dp[amount as usize] > amount {
            -1
        } else {
            dp[amount as usize]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::coin_change(vec![1, 2, 5],11), 3);
    }
    #[test]
    fn test_2(){
        assert_eq!(Solution::coin_change(vec![2],3), -1);
    }
    #[test]
    fn test_3(){
        assert_eq!(Solution::coin_change(vec![1],0), 0);
    }

}