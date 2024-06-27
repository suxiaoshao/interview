use std::cmp::Ordering;

pub struct Solution;
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        Self::find(&nums1, &nums2)
    }
    fn find(num1: &[i32], num2: &[i32]) -> f64 {
        if num1.is_empty() && num2.is_empty() {
            return 0.0;
        }
        if num1.is_empty() {
            return Self::find_mid(num2);
        }
        if num2.is_empty() {
            return Self::find_mid(num1);
        }
        if num1.len() + num2.len() <= 2 {
            if num1.is_empty() && num2.len() == 1 {
                return num2[0] as f64;
            }
            if num1.len() == 1 && num2.is_empty() {
                return num1[0] as f64;
            }
            if num1.len() == 1 && num2.len() == 1 {
                return Self::average(num1[0], num2[0]);
            }
            if num1.is_empty() && num2.len() == 2 {
                return (num2[0] + num2[1]) as f64 / 2.0;
            }
            if num1.len() == 2 && num2.is_empty() {
                return (num1[0] + num1[1]) as f64 / 2.0;
            }
        }
        let len1 = num1.len();
        let len2 = num2.len();
        let first1 = num1.first().unwrap();
        let first2 = num2.first().unwrap();
        let last1 = num1.last().unwrap();
        let last2 = num2.last().unwrap();
        match (first1.cmp(first2), last1.cmp(last2)) {
            (Ordering::Less, Ordering::Greater) => Self::find(&num1[1..len1 - 1], num2),
            (Ordering::Less, Ordering::Less) => Self::find(&num1[1..], &num2[0..len2 - 1]),
            (Ordering::Greater, Ordering::Greater) => Self::find(&num1[0..len1 - 1], &num2[1..]),
            (Ordering::Greater, Ordering::Less) => Self::find(num1, &num2[1..len2 - 1]),
            (Ordering::Less, Ordering::Equal) => Self::find(&num1[1..], &num2[0..len2 - 1]),
            (Ordering::Equal, Ordering::Less) => Self::find(&num1[1..], &num2[0..len2 - 1]),
            (Ordering::Equal, Ordering::Equal) => Self::find(&num1[1..], &num2[0..len2 - 1]),
            (Ordering::Equal, Ordering::Greater) => Self::find(&num1[1..len1 - 1], num2),
            (Ordering::Greater, Ordering::Equal) => Self::find(num1, &num2[1..len2 - 1]),
        }
    }
    fn find_mid(num: &[i32]) -> f64 {
        let len = num.len();
        if len % 2 == 0 {
            Self::average(num[len / 2 - 1], num[len / 2])
        } else {
            num[len / 2] as f64
        }
    }
    fn average(num1: i32, num2: i32) -> f64 {
        let sum = num1 + num2;
        if sum == 0 {
            0.0
        } else {
            sum as f64 / 2.0
        }
    }
}
fn main() {
    assert!(Solution::find_median_sorted_arrays(vec![1, 3], vec![2]) == 2.0);
    assert!(Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]) == 2.5);
    assert!(
        Solution::find_median_sorted_arrays(vec![0, 0, 0, 0, 0], vec![-1, 0, 0, 0, 0, 0, 1]) == 0.0
    );
    assert!(Solution::find_median_sorted_arrays(vec![2, 2, 4, 4], vec![2, 2, 4, 4]) == 3.0);
    assert!(Solution::find_median_sorted_arrays(vec![1, 2], vec![1, 2, 3]) == 2.0);
}
