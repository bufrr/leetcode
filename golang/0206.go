func reverseList(head *ListNode) *ListNode {
    if head == nil {
        return nil
    }

    var prevHead *ListNode
    for head != nil {
        tmp := head.Next
        head.Next = prevHead
        prevHead = head
        head = tmp
    }
    return prevHead
}
