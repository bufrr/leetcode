use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

struct SmallestInfiniteSet {
    pub is_present: HashSet<i32>,
    pub smallest_integer: i32,
    pub added_integers: BinaryHeap<Reverse<i32>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SmallestInfiniteSet {

    fn new() -> Self {
        SmallestInfiniteSet {
            is_present: HashSet::new(),
            smallest_integer: 1,
            added_integers: BinaryHeap::new(),
        }
    }

    fn pop_smallest(&mut self) -> i32 {
        // Your code here
        let mut answer: i32 = 0;
        if !self.added_integers.is_empty() {
            answer = self.added_integers.pop().unwrap().0;
            self.is_present.remove(&answer);
        } else {
            answer = self.smallest_integer;
            self.smallest_integer += 1;
        }
        answer
    }

    fn add_back(&mut self, num: i32) {
        if self.is_present.contains(&num) || num >= self.smallest_integer {
            return;
        }
        self.added_integers.push(Reverse(num));
        self.is_present.insert(num);
    }
}

