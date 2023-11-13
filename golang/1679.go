func maxOperations(nums []int, k int) int {
	count := 0
	m := make(map[int]int)
	for _, num := range nums {
		if num > k {
			continue
		}
		if v, ok := m[k-num]; ok {
			if v > 0 {
				m[k-num]--
				m[num]--
				count++
			}
		}
		m[num]++
	}
	return count
}
