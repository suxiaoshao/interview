use std::fmt::Display;

pub struct Solution;
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl Display for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)?;
        if let Some(next) = &self.next {
            write!(f, " -> {}", next)?;
        }
        Ok(())
    }
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.val.partial_cmp(&self.val)
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl From<&Box<ListNode>> for ListNode {
    fn from(value: &Box<ListNode>) -> Self {
        ListNode {
            val: value.val,
            next: None,
        }
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let lists = lists.into_iter().flatten().collect::<Vec<_>>();
        let mut queue = std::collections::BinaryHeap::<Box<ListNode>>::new();
        let mut result: Option<Box<ListNode>> = None;
        let mut end: &mut Option<Box<ListNode>> = &mut result;
        queue.extend(lists.into_iter());
        while let Some(item) = queue.pop() {
            match end {
                None => {
                    result = Some(Box::new(ListNode::from(&item)));
                    end = &mut result;
                }
                Some(m) => {
                    m.next = Some(Box::new(ListNode::from(&item)));
                    end = &mut m.next;
                }
            }
            if let Some(next) = item.next {
                queue.push(next);
            }
        }
        result
    }
}

fn generate_list(array: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &x in array.iter().rev() {
        head = Some(Box::new(ListNode { next: head, val: x }));
    }
    head
}

fn from_list(list: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = vec![];
    let mut list = list.clone();
    while let Some(node) = list {
        result.push(node.val);
        list = node.next;
    }
    result
}

fn main() {
    let list = vec![
        generate_list(&[1, 4, 5]),
        generate_list(&[1, 3, 4]),
        generate_list(&[2, 6]),
    ];
    let list = Solution::merge_k_lists(list);
    println!("{:?}", from_list(&list));
    assert_eq!(list, generate_list(&[1, 1, 2, 3, 4, 4, 5, 6]));
}
