func findMedianSortedArrays(nums1 []int, nums2 []int) float64 {
	len1 := len(nums1)
	len2 := len(nums2)
	allLen := len1 + len2
	isOdd := allLen % 2
	medianIdx := float64(allLen) / 2
	current, last, i, j, ctr := 0, 0, 0, 0, 0
	for {
		last = current
		if j == len2 || (i < len1 && nums1[i] <= nums2[j]) {
			current = nums1[i]
			i++
		} else {
			current = nums2[j]
			j++
		}
		ctr++
		if float64(ctr) > medianIdx {
			if isOdd == 1 {
				return float64(current)
			} else {
				return float64(last+current) / 2
			}
		}
	}
}
