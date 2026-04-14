/*
    Problem: Longest Common Prefix

    Description:

    Write a function to find the longest common prefix string amongst an array of strings.

    If there is no common prefix, return an empty string "".

    Example 1:

    Input: strs = ["flower","flow","flight"]
    Output: "fl"

    Example 2:

    Input: strs = ["dog","racecar","car"]
    Output: ""
*/

struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }

        let mut prefix = strs[0].clone();

        for s in strs.iter() {
            while !s.starts_with(&prefix) {
                prefix.pop();

                if prefix.is_empty() {
                    return String::new();
                }
            }
        }

        prefix
    }
}

fn main() {
    let strs: Vec<String> = vec!["dog", "racecar", "car"]
        .into_iter()
        .map(String::from)
        .collect();

    let result: String = Solution::longest_common_prefix(strs);

    println!("{}", result);
}
