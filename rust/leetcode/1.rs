/*
    Problem: Two Sum

    Description:

    Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

    You may assume that each input would have exactly one solution, and you may not use the same element twice.

    You can return the answer in any order.

    Example 1:
    Input: nums = [2,7,11,15], target = 9
    Output: [0,1]
    Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
    
    Example 2:
    Input: nums = [3,2,4], target = 6
    Output: [1,2]

    Example 3:
    Input: nums = [3,3], target = 6
    Output: [0,1]
*/

use std::collections::HashMap;

fn main() {
    let nums: Vec<i32> = vec![2, 7, 11, 15];
    let target: i32 = 9;

    println!("{:?}", two_sum_bruteforce(&nums, target));
    println!("{:?}", two_sum(&nums, target));
}

// Time Complexity = (O(n^2))
fn two_sum_bruteforce(nums: &[i32], target: i32) -> Vec<i32> {
    let n = nums.len();

    for i in 0..n {
        for j in i+1..n {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }

    vec![]
}

// Time Complexity = (O(n))
fn two_sum(nums: &[i32], target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;

        if let Some(&j) = map.get(&complement) {
            return vec![j as i32, i as i32];
        }
        
        map.insert(num, i);
    }

    vec![]
}
