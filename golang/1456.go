func maxVowels(s string, k int) int {
	maxx := 0
	length := len(s)

	count := 0
	for i := 0; i < k; i++ {
		if vowelCheck(s[i]) {
			count++
		}
	}
	maxx = count

	for i := k; i < length; i++ {
		if vowelCheck(s[i-k]) {
			count--
		}
		if vowelCheck(s[i]) {
			count++
		}
		if count > maxx {
			maxx = count
		}
	}

	return maxx
}

func vowelCheck(c uint8) bool {
	if c == 'a' || c == 'i' || c == 'e' || c == 'u' || c == 'o' {
		return true
	}
	return false
}
