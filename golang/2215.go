func findDifference(nums1 []int, nums2 []int) [][]int {
	m := make(map[int]struct{})
	n := make(map[int]struct{})
	var a, b []int
	for _, v := range nums1 {
		m[v] = struct{}{}
	}
	for _, v := range nums2 {
		n[v] = struct{}{}
	}
	for k, _ := range n {
		if _, ok := m[k]; !ok {
			b = append(b, k)
		}
	}
	for k, _ := range m {
		if _, ok := n[k]; !ok {
			a = append(a, k)
		}
	}
	return [][]int{a, b}
}
