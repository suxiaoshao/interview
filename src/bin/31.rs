struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if nums.len() < 2 {
            return;
        }
        let mut left_index = nums.len() - 2;
        while left_index > 0usize {
            if !Solution::next(&mut nums[left_index..]) {
                left_index -= 1;
            } else {
                return;
            }
        }
        if Solution::next(&mut nums[left_index..]) {
            return;
        }
        nums.reverse();
    }
    pub fn next(nums: &mut [i32]) -> bool {
        let first = nums[0];
        let mut second_index = None::<usize>;
        for (index, value) in nums.iter().skip(1).enumerate() {
            match second_index {
                Some(s_index) => {
                    let second = nums[s_index];
                    if second > *value && *value > first {
                        second_index = Some(index + 1)
                    }
                }
                None => {
                    if value > &first {
                        second_index = Some(index + 1)
                    }
                }
            }
        }
        match second_index {
            Some(second) => {
                nums.swap(0, second);
                nums[1..].sort();
                true
            }
            None => false,
        }
    }
}

fn main() {
    let v = &mut vec![1, 2, 3];
    Solution::next_permutation(v);
    assert_eq!(v, &mut vec![1, 3, 2]);

    let v = &mut vec![3, 2, 1];
    Solution::next_permutation(v);
    assert_eq!(v, &mut vec![1, 2, 3]);

    let v = &mut vec![1, 1, 5];
    Solution::next_permutation(v);
    assert_eq!(v, &mut vec![1, 5, 1]);

    let v = &mut vec![1, 3, 2];
    Solution::next_permutation(v);
    assert_eq!(v, &mut vec![2, 1, 3]);

    let v = &mut vec![2, 3, 1, 3, 3];
    Solution::next_permutation(v);
    assert_eq!(v, &mut vec![2, 3, 3, 1, 3]);
}
