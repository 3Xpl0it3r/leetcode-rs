struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty(){
            return 0;
        }
        let mut max_earn = 0;
        let mut prev_price = prices[0];
        let mut day_earn = 0;
        for i in 0..prices.len(){
            day_earn = prices[i] - prev_price;
            prev_price = prices[i];
            if day_earn > 0{
                max_earn += day_earn;
            }
        }
        max_earn
    }
}