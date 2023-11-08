func mergeAlternately(word1 string, word2 string) string {
    len1 := len(word1)
    len2 := len(word2)
    b := make([]byte,0)

    for i := 0; i < len1; i++ {
        if i >= len2 {
            break
        }
        b = append(b, word1[i], word2[i])
    }
    if len1 > len2 {
        b = append(b, word1[len(word2):]...)
    } 
    if len1 < len2 {
        b = append(b, word2[len(word1):]...)
    }
    return string(b)
}
