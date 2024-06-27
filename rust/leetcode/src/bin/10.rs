pub struct Solution;
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s_list = s.chars().collect::<Vec<_>>();
        let p_list = match RegexItem::from_string(&p) {
            Some(p_list) => p_list,
            None => return false,
        };
        Solution::match_str(&s_list, &p_list)
    }
    fn match_str(s: &[char], p: &[RegexItem]) -> bool {
        // println!("{s:?} {p:?}");
        if s.is_empty() && p.is_empty() {
            return true;
        }
        if s.is_empty() {
            return p.iter().all(|p| p.is_some());
        }
        if p.is_empty() {
            return false;
        }
        let s_char = s.first().unwrap();
        let p_char = p.first().unwrap();

        match p_char {
            RegexItem::SomeAll => {
                Solution::match_str(&s[1..], &p[1..])
                    || Solution::match_str(s, &p[1..])
                    || Solution::match_str(&s[1..], p)
            }
            RegexItem::Some(p_char) => {
                if !Solution::match_char(*s_char, *p_char) {
                    Solution::match_str(s, &p[1..])
                } else {
                    Solution::match_str(s, &p[1..])
                        || Solution::match_str(&s[1..], &p[1..])
                        || Solution::match_str(&s[1..], p)
                }
            }
            RegexItem::All => Solution::match_str(&s[1..], &p[1..]),
            RegexItem::Chat(p_char) => {
                if Solution::match_char(*s_char, *p_char) {
                    Solution::match_str(&s[1..], &p[1..])
                } else {
                    false
                }
            }
        }
    }

    fn match_char(s: char, p: char) -> bool {
        s == p || p == '.'
    }
}

#[derive(Debug)]
enum RegexItem {
    Chat(char),
    All,
    Some(char),
    SomeAll,
}

impl RegexItem {
    fn is_some(&self) -> bool {
        matches!(self, RegexItem::Some(_) | RegexItem::SomeAll)
    }
    fn from_string(input: &str) -> Option<Vec<Self>> {
        let input = input.chars().collect::<Vec<_>>();
        let mut output = vec![];
        let mut index = 0;
        while index < input.len() - 1 {
            let first = input[index];
            let second = input[index + 1];
            match (first, second) {
                ('*', _) => return None,
                ('.', '*') => {
                    if !matches!(output.last(), Some(RegexItem::SomeAll)) {
                        output.push(RegexItem::SomeAll);
                    }
                    index += 2;
                }
                (_, '*') => {
                    if let Some(RegexItem::Some(last)) = output.last() {
                        if *last != first {
                            output.push(RegexItem::Some(first));
                        }
                    } else {
                        output.push(RegexItem::Some(first));
                    }
                    index += 2;
                }
                ('.', _) => {
                    output.push(RegexItem::All);
                    index += 1;
                }
                (_, _) => {
                    output.push(RegexItem::Chat(first));
                    index += 1;
                }
            }
        }
        let last = input.get(index);
        match last {
            Some('*') => return None,
            Some('.') => output.push(RegexItem::All),
            Some(last) => output.push(RegexItem::Chat(*last)),
            None => {}
        }
        Some(output)
    }
}

fn main() {
    assert!(!Solution::is_match("aa".to_string(), "a".to_string()));
    assert!(Solution::is_match("aa".to_string(), "a*".to_string()));
    assert!(Solution::is_match("aab".to_string(), "c*a*b".to_string()));
    assert!(!Solution::is_match("ab".to_string(), ".*c".to_string()));
    assert!(Solution::is_match(
        "aaa".to_string(),
        "ab*a*c*a".to_string()
    ));
    assert!(Solution::is_match(
        "aaca".to_string(),
        "ab*a*c*a".to_string()
    ));
    assert!(!Solution::is_match("a".to_string(), ".*..".to_string()));
    assert!(Solution::is_match(
        "abcaaaaaaabaabcabac".to_string(),
        ".*ab.a.*a*a*.*b*b*".to_string()
    ));
    assert!(!Solution::is_match(
        "aaaaaaaaaaaaaaaaaaab".to_string(),
        "a*a*a*a*a*a*a*a*a*a*".to_string()
    ));
}
