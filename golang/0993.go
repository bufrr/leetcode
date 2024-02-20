type RecentCounter struct {
    l []int
}


func Constructor() RecentCounter {
    return RecentCounter{
        l : make([]int, 0),
    }
}


func (this *RecentCounter) Ping(t int) int {
    this.l = append(this.l, t)
    for i, n := range this.l {
        if n >= t-3000 {
            this.l = this.l[i:]
            break
        }
    } 
    return len(this.l)
}

