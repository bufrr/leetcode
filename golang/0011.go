func maxArea(height []int) int {
	max := 0
	left := 0
	right := len(height) - 1

	for left < right {
		width := right - left
		max = Max(max, Min(height[left], height[right])*width)

		if height[left] < height[right] {
			left++
		} else {
			right--
		}
	}
	return max
}

func Min(x, y int) int {
	if x < y {
		return x
	}
	return y
}

func Max(x, y int) int {
	if x < y {
		return y
	}
	return x
}
