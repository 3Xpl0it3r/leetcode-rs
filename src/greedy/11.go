package greedy

func maxArea(height []int) int {
	maxCapacity := 0
	curCapacity := 0
	left := 0
	right := len(height) - 1
	for left < right {
		if height[left] < height[right]{
			curCapacity = height[left] * (right - left)
			left ++
		}else {
			curCapacity = height[right] * (right - left)
			right --
		}
		if curCapacity > maxCapacity{
			maxCapacity = curCapacity
		}
	}
	return maxCapacity
}
