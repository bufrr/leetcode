func closeStrings(word1 string, word2 string) bool {
	if len(word1) != len(word2) {
		return false
	}


	m := make(map[int32]int)
	n := make(map[int32]int)

	for _, v := range word1 {
		m[v]++
	}
	for _, v := range word2 {
		n[v]++
	}

	var mk []int
	var mv []int

	var nk []int
	var nv []int

	for k, v := range m {
		mk = append(mk, int(k))
		mv = append(mv, v)
	}
	for k, v := range n {
		nk = append(nk, int(k))
		nv = append(nv, v)
	}

	sort.Ints(mk)
	sort.Ints(mv)
	sort.Ints(nk)
	sort.Ints(nv)

	return slices.Equal(mk, nk) && slices.Equal(mv, nv)
}

