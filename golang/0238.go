func productExceptSelf(nums []int) []int {
	length := len(nums)
	left := make([]int, length)
	right := make([]int, length)
	res := make([]int, length)

	left[0] = 1
	right[length-1] = 1

	for i := 1; i < length; i++ {
		left[i] = left[i-1] * nums[i-1]
	}

	for i := length - 2; i >= 0; i-- {
		right[i] = right[i+1] * nums[i+1]
	}

	for i := 0; i < length; i++ {
		res[i] = left[i] * right[i]
	}

	return res
}
