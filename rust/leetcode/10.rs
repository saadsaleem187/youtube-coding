/*
    Problem: Search Insert Position

    Description:

    Given a sorted array of distinct integers and a target value,
    return the index if the target is found.
    If not, return the index where it would be if it were inserted in order.

    You must write an algorithm with O(log n) runtime complexity.

    Example 1:

    Input: nums = [1,3,5,6], target = 5
    Output: 2

    Example 2:

    Input: nums = [1,3,5,6], target = 2
    Output: 1

    Example 3:

    Input: nums = [1,3,5,6], target = 7
    Output: 4
*/
struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();

        while left < right {
            let mid = left + (right - left) / 2;

            if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left as i32
    }
}
