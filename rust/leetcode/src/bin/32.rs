use std::fmt::{Debug, Display};

struct Solution;

#[derive(Clone, Copy)]
enum Parentheses {
    Left,
    Right,
}

impl Display for Parentheses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Parentheses::Left => write!(f, "("),
            Parentheses::Right => write!(f, ")"),
        }
    }
}

impl Debug for Parentheses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left => write!(f, "（"),
            Self::Right => write!(f, "）"),
        }
    }
}

impl Parentheses {
    pub fn from_char(c: char) -> Self {
        match c {
            '(' => Self::Left,
            ')' => Self::Right,
            _ => panic!(),
        }
    }
}

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut result = vec![false; s.len()];
        let mut stack = Vec::new();
        for (new_index, c) in s.chars().enumerate() {
            let new = Parentheses::from_char(c);
            let old = stack.last();
            match (new, old) {
                (Parentheses::Left, _) => stack.push((new_index, new)),
                (Parentheses::Right, Some((old_index, Parentheses::Left))) => {
                    result[new_index] = true;
                    result[*old_index] = true;
                    stack.pop();
                }
                (Parentheses::Right, _) => {
                    stack.push((new_index, new));
                }
            }
        }
        let mut max = 0;
        let mut now_count = 0;
        for i in result {
            if i {
                now_count += 1;
            } else {
                max = std::cmp::max(max, now_count);
                now_count = 0;
            }
        }
        max.max(now_count)
    }
}

fn main() {
    assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);
    assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);
    assert_eq!(Solution::longest_valid_parentheses("".to_string()), 0);
    assert_eq!(Solution::longest_valid_parentheses("()(()".to_string()), 2);
    assert_eq!(Solution::longest_valid_parentheses("()(())".to_string()), 6);
}
