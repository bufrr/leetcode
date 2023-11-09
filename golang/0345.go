func reverseVowels(s string) string {
	length := len(s)
	res := []byte(s)
	n := length - 1

	for i, c := range s {
		if n < i {
			break
		}
		if vowelCheck(uint8(c)) {
			for j := n; j > i; j-- {
				if vowelCheck(s[j]) {
					res[i] = s[j]
					res[j] = s[i]
					n = j - 1
					break
				} else {
					res[j] = s[j]
				}
			}
		} else {
			res[i] = byte(c)
		}
	}
	return string(res)
}

func vowelCheck(c uint8) bool {
	if c == 'a' || c == 'i' || c == 'e' || c == 'u' || c == 'o' ||
		c == 'A' || c == 'E' || c == 'I' || c == 'O' || c == 'U' {
		return true
	}
	return false
}
