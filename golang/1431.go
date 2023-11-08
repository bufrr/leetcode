func kidsWithCandies(candies []int, extraCandies int) []bool {
    largest := candies[0]
	for i:=0; i<len(candies); i++ {
		if largest<candies[i] {
			largest = candies[i]
		}
	}
	b := make([]bool, len(candies))
	for i:=0; i<len(candies); i++ {
		b[i] = false
		if candies[i] + extraCandies >= largest {
			b[i] = true
		}
	}
	return b
}
