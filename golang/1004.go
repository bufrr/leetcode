func longestOnes(nums []int, k int) int {
	left := 0
	right := 0
	length := len(nums)

	for right < length {
		if nums[right] == 0 {
			k--
		}
		if k < 0 {
			if nums[left] == 0 {
				k++
			}
			left++
		}
		right++
	}
	return right - left
}
