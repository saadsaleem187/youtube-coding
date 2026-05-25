/*
    Problem: Find the Index of the First Occurrence in a String

    Description:

    Given two strings needle and haystack, return the index of the
    first occurrence of needle in haystack,
    or -1 if needle is not part of haystack.

    Example 1:

    Input: haystack = "sadbutsad", needle = "sad"
    Output: 0
    Explanation: "sad" occurs at index 0 and 6.
    The first occurrence is at index 0, so we return 0.

    Example 2:

    Input: haystack = "leetcode", needle = "leeto"
    Output: -1
    Explanation: "leeto" did not occur in "leetcode", so we return -1.

*/

struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let h = haystack.as_bytes();
        let n = needle.as_bytes();

        if n.len() > h.len() {
            return -1;
        }

        for i in 0..=h.len() - n.len() {
            if &h[i..i + n.len()] == n {
                return i as i32;
            }
        }

        -1
    }
}
