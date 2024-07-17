///https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-iii/description/
pub(super) fn max_profit_impl_iii(prices: Vec<i32>) -> i32 {
    let mut sell_1 = 0;
    let mut sell_2 = 0;
    let mut buy_1 = -prices[0];
    let mut buy_2 = -prices[0];
    for i in 1..prices.len() {
        buy_1 = buy_1.max(-prices[i]);
        sell_1 = sell_1.max(prices[i] + buy_1);
        buy_2 = buy_2.max(sell_1 - prices[i]);
        sell_2 = sell_2.max(prices[i] + buy_2);
    }
    sell_2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(max_profit_impl_iii(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
    }
    #[test]
    fn test_2() {
        assert_eq!(max_profit_impl_iii(vec![1, 2, 3, 4, 5]), 4);
    }
    #[test]
    fn test_3() {
        assert_eq!(max_profit_impl_iii(vec![7,6,4,3,1]), 0);
    }
}
