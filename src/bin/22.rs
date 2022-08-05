use std::collections::HashSet;

pub struct Solution;

#[derive(Clone, Hash, PartialEq, Eq)]
enum Brackets {
    Open,
    Close,
}
#[derive(Clone, Hash, PartialEq, Eq)]
struct Item {
    brackets: Vec<Brackets>,
    open_remain: i32,
    close_remain: i32,
}

impl Item {
    fn is_complete(&self) -> bool {
        self.open_remain == 0 && self.close_remain == 0
    }
    fn gen(&self) -> Vec<Item> {
        let mut result = vec![];
        if self.open_remain > 0 {
            let mut item = self.clone();
            item.open_remain -= 1;
            item.brackets.push(Brackets::Open);
            result.push(item);
        }
        if self.open_remain < self.close_remain {
            let mut item = self.clone();
            item.close_remain -= 1;
            item.brackets.push(Brackets::Close);
            result.push(item);
        }
        result
    }
    fn string(&self) -> String {
        let mut result = String::new();
        for bracket in &self.brackets {
            match bracket {
                Brackets::Open => result.push('('),
                Brackets::Close => result.push(')'),
            }
        }
        result
    }
}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut map = HashSet::new();
        let mut result = vec![];
        let item = Item {
            brackets: vec![Brackets::Open],
            open_remain: n - 1,
            close_remain: n,
        };
        Solution::dfs(&mut map, &item, &mut result);
        result
    }
    fn dfs(map: &mut HashSet<Item>, item: &Item, result: &mut Vec<String>) {
        let items = item.gen();
        for item in items {
            if item.is_complete() {
                result.push(item.string());
            } else if !map.contains(&item) {
                map.insert(item.clone());
                Solution::dfs(map, &item, result);
                map.remove(&item);
            }
        }
    }
}

fn main() {
    let grid = vec!["((()))", "(()())", "(())()", "()(())", "()()()"];
    assert_eq!(Solution::generate_parenthesis(3), grid);
}
