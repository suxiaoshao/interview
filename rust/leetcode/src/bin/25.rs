// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn get_kth(&mut self) -> Option<Vec<i32>> {
        let mut p = self;
        let mut v = Vec::new();
        while let Some(next) = p.next.as_mut() {
            v.push(p.val);
            p = next;
        }
        v.push(p.val);
        Some(v)
    }
}
pub struct Solution;
impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = head?;
        let list = head.get_kth()?;
        let mut iter = list.chunks_exact(k as usize);
        let mut rest = vec![];
        for chunk in iter.by_ref() {
            let mut chunk = chunk.to_vec();
            chunk.reverse();
            rest.append(&mut chunk);
        }
        println!("{:?}", iter.remainder().to_vec());
        rest.append(&mut iter.remainder().to_vec());
        println!("{:?}", rest);
        generate_list(&rest)
    }
}
fn generate_list(array: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &x in array.iter().rev() {
        head = Some(Box::new(ListNode { next: head, val: x }));
    }
    head
}

fn main() {
    let list = generate_list(&[1, 2, 3, 4, 5]);
    let list = Solution::reverse_k_group(list, 2);
    assert_eq!(list, generate_list(&[2, 1, 4, 3, 5]));
}
