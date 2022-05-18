package greedy


func maxProfit(prices []int) int {
	if len(prices) < 1{
		return 0
	}
	earn := 0
	prevPrice := prices[0]

	for i := 0 ; i< len(prices) ;i ++{
		dayEarn := prices[i] - prevPrice
		prevPrice = prices[i]
		if dayEarn > 0 {
			earn += dayEarn
		}
	}
	return earn
}
