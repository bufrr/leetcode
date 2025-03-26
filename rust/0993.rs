struct RecentCounter {
    window: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {

    fn new() -> Self {
        RecentCounter{ window: Vec::new(),}
    }
    
    fn ping(&mut self, t: i32) -> i32 {
        self.window.push(t);
        while let Some(&first) = self.window.first() {
            if first < t - 3000 {
                self.window.remove(0);
            } else {
                break;
            }
        } 
        self.window.len() as i32
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */
