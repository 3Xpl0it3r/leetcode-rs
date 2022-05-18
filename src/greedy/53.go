package greedy


func maxSubArray(nums []int) int {
	if len(nums)  == 0{
		return 0
	}
	if len(nums) < 2 {
		return nums[0]
	}
	maxsum := nums[0]
	sum := 0
	for i := 0; i < len(nums) ; i++{
		sum += nums[i]
		if sum > maxsum{
			maxsum = sum
		}
		if sum <= 0{
			sum = 0
		}
	}
	return maxsum
}
