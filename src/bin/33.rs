struct Solution;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        Solution::search_slice(&nums, target).unwrap_or(-1)
    }
    fn search_slice(nums: &[i32], target: i32) -> Option<i32> {
        if nums.is_empty() {
            return None;
        }
        if nums.len() == 1 {
            return if nums[0] == target { Some(0) } else { None };
        }
        let right = nums.len() - 1;
        let mid = right / 2;
        if nums[mid] == target {
            return Some(mid as i32);
        }
        let first = nums[0];
        let last = nums[right];
        let output = if first <= nums[mid] {
            if first <= target && target < nums[mid] {
                Solution::find_sort(&nums[..mid], target)?
            } else {
                mid as i32 + 1 + Solution::search_slice(&nums[mid + 1..], target)?
            }
        } else if nums[mid] < target && target <= last {
            mid as i32 + 1 + Solution::find_sort(&nums[mid + 1..], target)?
        } else {
            Solution::search_slice(&nums[..mid], target)?
        };
        Some(output)
    }
    fn find_sort(nums: &[i32], target: i32) -> Option<i32> {
        let left = 0;
        if nums.is_empty() {
            return None;
        }
        if nums.len() == 1 {
            return if nums[0] == target { Some(0) } else { None };
        }
        let right = nums.len() - 1;

        let mid = (left + right) / 2;
        let output = match nums[mid].cmp(&target) {
            std::cmp::Ordering::Less => {
                mid as i32 + 1 + (Solution::find_sort(&nums[mid + 1..], target)?)
            }
            std::cmp::Ordering::Equal => mid as i32,
            std::cmp::Ordering::Greater => Solution::find_sort(&nums[..mid], target)?,
        };
        Some(output)
    }
}

fn main() {
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    assert_eq!(Solution::search(vec![1], 0), -1);
    assert_eq!(Solution::find_sort(&[1, 2, 3, 4, 5], 4), Some(3));
    assert_eq!(Solution::find_sort(&[1, 2, 3, 4, 5], 5), Some(4));
}
