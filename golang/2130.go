func pairSum(head *ListNode) int {
    cur := head
    var values []int
    
    for cur != nil {
        values = append(values, cur.Val)
        cur = cur.Next
    }

    max := 0
    i,j  := 0, len(values) - 1
    for i<j {
        if values[i] + values[j] > max {
            max = values[i] + values[j]
        }
        i++
        j--
    }
    return max
}
