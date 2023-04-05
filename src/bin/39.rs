use std::{collections::VecDeque, ops::Deref};

struct Solution;

#[derive(Default, Clone, Debug)]
struct Item {
    value: Vec<i32>,
    count: i32,
}

impl Deref for Item {
    type Target = Vec<i32>;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Item {
    fn push(&mut self, value: i32) {
        self.value.push(value);
        self.count += value;
    }
    fn next(&self, candidates: &[i32], target: i32) -> (Vec<Item>, Vec<Vec<i32>>) {
        let mut next = Vec::new();
        let mut result = Vec::new();
        let max = self.last();
        for i in candidates
            .iter()
            .filter(|x| max.map(|max| max <= x).unwrap_or(true))
        {
            match (i + self.count).cmp(&target) {
                std::cmp::Ordering::Less => {
                    let mut item = self.clone();
                    item.push(*i);
                    next.push(item);
                }
                std::cmp::Ordering::Equal => {
                    let mut item = self.value.clone();
                    item.push(*i);
                    result.push(item);
                }
                std::cmp::Ordering::Greater => {}
            }
        }
        (next, result)
    }
}

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut queue = VecDeque::new();
        queue.push_back(Item {
            value: Vec::new(),
            count: 0,
        });
        while let Some(item) = queue.pop_front() {
            let (next, new_result) = item.next(&candidates, target);
            queue.extend(next);
            result.extend(new_result);
        }
        result
    }
}

fn main() {
    assert_eq!(
        Solution::combination_sum(vec![2, 3, 6, 7], 7),
        vec![vec![7], vec![2, 2, 3]]
    );
    assert_eq!(
        Solution::combination_sum(vec![2, 3, 5], 8),
        vec![vec![3, 5], vec![2, 3, 3], vec![2, 2, 2, 2]]
    );
    assert_eq!(
        Solution::combination_sum(vec![2], 1),
        Vec::<Vec<i32>>::new()
    );
}
