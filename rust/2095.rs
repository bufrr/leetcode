#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return None;
        }
        let mut length = 0;
        let mut p = head.as_ref();
        while let Some(node) = p {
            length += 1;
            p = node.next.as_ref();
        }

        let mid = length / 2;
        let mut current = &mut head;

        for _ in 0..mid - 1 {
            current = &mut current.as_mut().unwrap().next;
        }

        let next = current.as_mut().unwrap().next.as_mut().unwrap().next.take();
        current.as_mut().unwrap().next = next;

        head
    }
}
