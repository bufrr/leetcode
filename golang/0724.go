func pivotIndex(nums []int) int {
    sum := 0
	leftSum := 0
	for _, v := range nums {
		sum += v
	}
	for i:= 0; i < len(nums); i++ {
		if leftSum == sum - leftSum - nums[i] {
			return i
		}
		leftSum+=nums[i]
	}
	return -1
}
