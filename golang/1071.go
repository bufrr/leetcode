func gcdOfStrings(str1 string, str2 string) string {
    if str1 + str2 != str2 + str1 {
		return ""
	}
	len1 := len(str1)
	len2 := len(str2)
	for len2 != 0 {
		len1, len2 = len2, len1 % len2
	}
	return str1[:len1]
}
