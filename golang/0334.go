func increasingTriplet(nums []int) bool {
	first := math.MaxInt
	second := math.MaxInt

	for _, n := range nums {
		if first >= n {
			first = n
		} else if second >= n {
			second = n
		} else {
			return true
		}
	}
	return false
}
