func oddEvenList(head *ListNode) *ListNode {
	if head == nil {
		return nil
	}
	res := head
	evenHeader := head.Next
	curOdd := head
	curEven := evenHeader
	for curEven != nil && curEven.Next != nil {
		curOdd.Next = curEven.Next
		curOdd = curOdd.Next
		curEven.Next = curOdd.Next
		curEven = curEven.Next
	}
	curOdd.Next = evenHeader
	return res
}
