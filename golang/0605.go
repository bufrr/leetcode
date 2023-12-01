func canPlaceFlowers(flowerbed []int, n int) bool {
	for i := range flowerbed {
		if flowerbed[i] == 1 {
			continue
		}
		if (i==0 || flowerbed[i-1]==0) && (i == len(flowerbed)-1 || flowerbed[i+1] ==0) {
			flowerbed[i] = 1
			n--
		}
	}
	return n<=0
}
