func largestAltitude(gain []int) int {
    maxx := 0
	cur := 0
	length := len(gain)

	for i:=0; i < length; i++ {
		cur += gain[i]
		if cur > maxx {
			maxx = cur
		}
	}
	if maxx < 0 {
		maxx = 0
	}
	return maxx
}
