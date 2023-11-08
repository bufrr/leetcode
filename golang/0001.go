package main

import "fmt"

// Violent cracking
func twoSum(nums []int, target int) []int {
	res := make([]int, 2)
	for index, num := range nums {
		res[0] = index
		v := target - num
		for i := index + 1; i < len(nums); i++ {
			if nums[i] == v {
				res[1] = i
				return res
			}
		}
	}
	return nil
}

// Using HashMap
func twoSum1(nums []int, target int) []int {
	res := make([]int, 2)
	m := make(map[int]int)

	for index, num := range nums {
		v := target - num
		if val, ok := m[v]; ok {
			res = []int{val, index}
			return res
		}
		m[num] = index
	}
	return nil
}

func main() {
	num := []int{2, 7, 11, 15}
	fmt.Println(twoSum1(num, 26))
}
