fn main() {
    let profit = max_profit(vec![7,1,5,3,6,4]);
    println!("{profit}");
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit = 0;
    let mut min_index = 0;
    for index in 1..prices.len() {
        if prices[index] < prices[min_index] {
            min_index = index;
        } else {
            let profit = prices[index] - prices[min_index];
            if profit > max_profit {
                max_profit = profit
            }
        }
    }
    return max_profit;
}

