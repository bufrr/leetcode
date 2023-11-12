func compress(chars []byte) int {
	debug.SetGCPercent(1)
	res := 0
	i := 0
	length := len(chars)
	for i < length {
		groupLength := 1
		for i+groupLength < length && chars[i+groupLength] == chars[i] {
			groupLength++
		}
		chars[res] = chars[i]
		res++
		if groupLength > 1 {
			l := strconv.Itoa(groupLength)
			for _, c := range l {
				chars[res] = byte(c)
				res++
			}
		}
		i += groupLength
	}
	return res
}
