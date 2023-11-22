func equalPairs(grid [][]int) int {
  length := len(grid)
  m := make(map[[200]int]int)
  arr:=[200]int{}
  for i:=0; i<length; i++ {
     copy(arr[:], grid[i])
     m[arr]++
  }

  res := 0

  for i:=0; i<length; i++ {
      arr = [200]int{}
      for j:=0; j<length; j++ {
        arr[j] = grid[j][i]
      }
      if v, ok:=m[arr]; ok {
        res += v
      }
  }
  return res
}
