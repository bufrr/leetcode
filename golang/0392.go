func isSubsequence(s string, t string) bool {
	left := 0
	right := 0
	leftBound := len(s)
	rightBound := len(t)

	for left < leftBound && right < rightBound {
		if s[left] == t[right] {
			left++
		}
		right++
	}
	return left == leftBound
}
