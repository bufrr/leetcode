func findMaxAverage(nums []int, k int) float64 {
	length := len(nums)
	sum := 0
	for i := 0; i < k; i++ {
		sum += nums[i]
	}

	maxx := sum
	for i := k; i < length; i++ {
		sum = sum - nums[i-k] + nums[i]
		if sum > maxx {
			maxx = sum
		}
	}
	return float64(maxx) / float64(k)
}
