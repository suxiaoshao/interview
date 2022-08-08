use std::collections::{HashSet, VecDeque};

pub struct Solution;

#[derive(Clone)]
struct Item {
    value: i32,
    len: i32,
}

impl Item {
    fn gen(&self, coins: &Vec<i32>) -> Vec<Item> {
        let mut result = vec![];
        for coin in coins {
            if self.value - coin >= 0 {
                let mut item = self.clone();
                item.value -= *coin;
                item.len += 1;
                result.push(item);
            }
        }
        result
    }
}

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut coins = coins;
        coins.sort_by(|a, b| b.cmp(a));
        let item = Item {
            value: amount,
            len: 0,
        };
        let mut queue = VecDeque::new();
        let mut set = HashSet::new();
        queue.push_back(item);
        while let Some(item) = queue.pop_front() {
            if item.value == 0 {
                return item.len;
            }
            let items = item.gen(&coins);
            for item in items {
                if !set.contains(&item.value) {
                    set.insert(item.value);
                    queue.push_back(item);
                }
            }
        }
        -1
    }
}
fn main() {
    assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
}
