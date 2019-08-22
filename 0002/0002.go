package main

import (
	"fmt"
	"math/big"
)

type ListNode struct {
	Val  int
	Next *ListNode
}

func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
	dummyHead := &ListNode{0, nil}
	p, q, curr := l1, l2, dummyHead
	x, y, carry := 0, 0, 0
	for q != nil || p != nil {
		if p == nil {
			x = 0
		} else {
			x = p.Val
		}
		if q == nil {
			y = 0
		} else {
			y = q.Val
		}
		sum := x + y + carry
		carry = sum / 10
		curr.Next = &ListNode{sum % 10, nil}
		curr = curr.Next
		if p != nil {
			p = p.Next
		}
		if q != nil {
			q = q.Next
		}
		if carry > 0 {
			curr.Next = &ListNode{carry, nil}
		}
	}
	return dummyHead.Next
}

func addTwoNumbers2(l1 *ListNode, l2 *ListNode) *ListNode {
	res := big.NewInt(0).Add(listToInt(l1), listToInt(l2))
	return intToList(res)
}

func listToInt(l *ListNode) *big.Int {
	res := big.NewInt(0)
	for i := 0; ; i++ {
		res.Add(big.NewInt(0).Mul(big.NewInt(0).Exp(big.NewInt(10), big.NewInt(int64(i)), nil), big.NewInt(int64(l.Val))), res)
		if l.Next == nil {
			break
		}
		l = l.Next
	}
	//fmt.Println("res:", res.String())
	return res
}

func intToList(v *big.Int) *ListNode {
	//fmt.Println("v:", v.String())
	tmp := big.NewInt(0).Mod(v, big.NewInt(10))
	//fmt.Println("tmp:", tmp)
	cur := &ListNode{int(tmp.Int64()), nil}
	last := cur
	first := cur
	v.Div(v, big.NewInt(10))
	for v.Cmp(big.NewInt(0)) > 0 {
		tmp := big.NewInt(0).Mod(v, big.NewInt(10))
		cur = &ListNode{int(tmp.Int64()), nil}
		last.Next = cur
		last = cur
		v.Div(v, big.NewInt(10))
	}
	return first
}

func pp(l *ListNode) {
	for l != nil {
		fmt.Print(l.Val)
		l = l.Next
	}
}

func main() {
	l3 := ListNode{3, nil}
	l2 := ListNode{0, &l3}
	l1 := ListNode{3, &l2}

	l6 := ListNode{4, nil}
	l5 := ListNode{6, &l6}
	l4 := ListNode{5, &l5}

	pp(addTwoNumbers2(&l1, &l4))
	//ll2 := intToList(big.NewInt(10000000000000001))
	//pp(addTwoNumbers(ll2, &l1))
}
