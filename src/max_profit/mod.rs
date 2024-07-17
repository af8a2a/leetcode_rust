use crate::Solution;
mod max_profit_impl_iii;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        max_profit_impl_iii::max_profit_impl_iii(prices)
    }

}
