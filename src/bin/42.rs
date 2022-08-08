pub struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let max_index = Self::max_index(&height);
        Self::trap_left(&height[0..=max_index]) + Self::trap_right(&height[max_index..])
    }
    fn trap_left(array: &[i32]) -> i32 {
        if array.len() <= 2 {
            return 0;
        }
        let max_index = Self::max_index(&array[0..(array.len() - 1)]);
        Self::trap_range(&array[max_index..]) + Self::trap_left(&array[0..=max_index])
    }
    fn trap_right(array: &[i32]) -> i32 {
        if array.len() <= 2 {
            return 0;
        }
        let max_index = Self::max_index(&array[1..]) + 1;
        Self::trap_range(&array[0..=max_index]) + Self::trap_right(&array[max_index..])
    }
    fn trap_range(array: &[i32]) -> i32 {
        if array.len() <= 2 {
            return 0;
        }
        let first = array.first().unwrap();
        let last = array.last().unwrap();
        let min = *if first < last { first } else { last };

        array[1..(array.len() - 1)]
            .iter()
            .fold(0, |acc, &x| acc + (min - x))
    }
    fn max_index(array: &[i32]) -> usize {
        let mut max_index = 0;
        for (index, value) in array.iter().enumerate() {
            if *value > array[max_index] {
                max_index = index;
            }
        }
        max_index
    }
}
fn main() {
    assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
}
