func longestSubarray(nums []int) int {
	left := 0
	right := 0
	zeroCount := 0
	length := len(nums)
	longest := 0

	for right < length {
		if nums[right] == 0 {
			zeroCount++
		}
		for zeroCount > 1 {
			if nums[left] == 0 {
				zeroCount--
			}
			left++
		}
		longest = max(longest, right-left)
		right++
	}
	return longest
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}
