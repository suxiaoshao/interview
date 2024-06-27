pub struct Solution;
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let list = RomeNumber::from_string(&s);

        list.into_iter().fold(0, |a, b| a + (b as i32))
    }
}

enum RomeNumber {
    I = 1,
    V = 5,
    IV = 4,
    X = 10,
    IX = 9,
    L = 50,
    XL = 40,
    C = 100,
    XC = 90,
    D = 500,
    CD = 400,
    M = 1000,
    CM = 900,
}

impl RomeNumber {
    fn from_string(s: &str) -> Vec<Self> {
        let s = s.chars().collect::<Vec<_>>();
        let mut index = 0;
        let mut result = vec![];
        while index < s.len() - 1 {
            let first = s[index];
            let second = s[index + 1];
            match (first, second) {
                ('I', 'V') => {
                    result.push(Self::IV);
                    index += 2;
                }
                ('I', 'X') => {
                    result.push(Self::IX);
                    index += 2;
                }
                ('I', _) => {
                    result.push(Self::I);
                    index += 1;
                }
                ('V', _) => {
                    result.push(Self::V);
                    index += 1;
                }
                ('X', 'L') => {
                    result.push(Self::XL);
                    index += 2;
                }
                ('X', 'C') => {
                    result.push(Self::XC);
                    index += 2;
                }
                ('X', _) => {
                    result.push(Self::X);
                    index += 1;
                }
                ('L', _) => {
                    result.push(Self::L);
                    index += 1;
                }
                ('C', 'D') => {
                    result.push(Self::CD);
                    index += 2;
                }
                ('C', 'M') => {
                    result.push(Self::CM);
                    index += 2;
                }
                ('C', _) => {
                    result.push(Self::C);
                    index += 1;
                }
                ('D', _) => {
                    result.push(Self::D);
                    index += 1;
                }
                ('M', _) => {
                    result.push(Self::M);
                    index += 1;
                }
                _ => panic!(),
            }
        }
        match s.get(index) {
            Some('I') => {
                result.push(Self::I);
            }
            Some('V') => {
                result.push(Self::V);
            }
            Some('X') => {
                result.push(Self::X);
            }
            Some('L') => {
                result.push(Self::L);
            }
            Some('C') => {
                result.push(Self::C);
            }
            Some('D') => {
                result.push(Self::D);
            }
            Some('M') => {
                result.push(Self::M);
            }
            None => {}
            _ => panic!(),
        }
        result
    }
}

fn main() {
    assert_eq!(Solution::roman_to_int("III".to_string()), 3);
    assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
    assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
    assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
    assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
}
