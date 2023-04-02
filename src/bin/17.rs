use std::collections::VecDeque;

pub struct Solution;

#[derive(Debug, Clone, Copy)]
enum PhoneKey {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl From<char> for PhoneKey {
    fn from(value: char) -> Self {
        match value {
            '2' => PhoneKey::Two,
            '3' => PhoneKey::Three,
            '4' => PhoneKey::Four,
            '5' => PhoneKey::Five,
            '6' => PhoneKey::Six,
            '7' => PhoneKey::Seven,
            '8' => PhoneKey::Eight,
            '9' => PhoneKey::Nine,
            _ => panic!(),
        }
    }
}

impl PhoneKey {
    fn get_letter(&self) -> &[char] {
        match self {
            PhoneKey::Two => &['a', 'b', 'c'],
            PhoneKey::Three => &['d', 'e', 'f'],
            PhoneKey::Four => &['g', 'h', 'i'],
            PhoneKey::Five => &['j', 'k', 'l'],
            PhoneKey::Six => &['m', 'n', 'o'],
            PhoneKey::Seven => &['p', 'q', 'r', 's'],
            PhoneKey::Eight => &['t', 'u', 'v'],
            PhoneKey::Nine => &['w', 'x', 'y', 'z'],
        }
    }
}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let digits = digits.chars().map(PhoneKey::from).collect::<Vec<_>>();
        let mut result = vec![];
        let mut queue = VecDeque::<(String, usize)>::new();
        queue.push_back((String::new(), 0));
        while let Some((str, index)) = queue.pop_front() {
            let key = match digits.get(index) {
                Some(key) => key,
                None => continue,
            };
            for letter in key.get_letter() {
                let new_str = format!("{}{}", str, letter);
                if index == digits.len() - 1 {
                    result.push(new_str);
                } else {
                    queue.push_back((new_str.clone(), index + 1));
                }
            }
        }
        result
    }
}
fn main() {
    assert_eq!(
        Solution::letter_combinations("23".to_string()),
        vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
    );
    assert_eq!(Solution::letter_combinations("".to_string()), vec![""; 0]);
    assert_eq!(
        Solution::letter_combinations("2".to_string()),
        vec!["a", "b", "c"]
    );
}
