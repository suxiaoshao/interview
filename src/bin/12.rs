pub struct Solution;
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        match num {
            0..=3 => generate_i_string(num as usize, 'I'),
            4 => "IV".to_string(),
            5..=8 => format!("V{}", Solution::int_to_roman(num - 5)),
            9 => "IX".to_string(),
            10..=39 => {
                let quotient = num / 10;
                let remainder = num % 10;
                format!(
                    "{}{}",
                    generate_i_string(quotient as usize, 'X'),
                    Solution::int_to_roman(remainder)
                )
            }
            40..=49 => {
                let remainder = num % 10;
                format!("XL{}", Solution::int_to_roman(remainder))
            }
            50..=89 => format!("L{}", Solution::int_to_roman(num - 50)),
            90..=99 => format!("XC{}", Solution::int_to_roman(num - 90)),
            100..=399 => {
                let quotient = num / 100;
                let remainder = num % 100;
                format!(
                    "{}{}",
                    generate_i_string(quotient as usize, 'C'),
                    Solution::int_to_roman(remainder)
                )
            }
            400..=499 => {
                let remainder = num % 100;
                format!("CD{}", Solution::int_to_roman(remainder))
            }
            500..=899 => format!("D{}", Solution::int_to_roman(num - 500)),
            900..=999 => format!("CM{}", Solution::int_to_roman(num - 900)),
            1000..=3999 => {
                let quotient = num / 1000;
                let remainder = num % 1000;
                format!(
                    "{}{}",
                    generate_i_string(quotient as usize, 'M'),
                    Solution::int_to_roman(remainder)
                )
            }
            _ => panic!(),
        }
    }
}

fn generate_i_string(n: usize, c: char) -> String {
    let mut result = String::with_capacity(n);
    for _ in 0..n {
        result.push(c);
    }
    result
}
fn main() {
    assert_eq!(Solution::int_to_roman(3), "III");
    assert_eq!(Solution::int_to_roman(36), "XXXVI");
    assert_eq!(Solution::int_to_roman(9), "IX");
    assert_eq!(Solution::int_to_roman(58), "LVIII");
    assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
}
