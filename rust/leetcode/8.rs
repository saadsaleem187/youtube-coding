/*
    Problem: Valid Parentheses

    Description:

    Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

    An input string is valid if:

    1. Open brackets must be closed by the same type of brackets.
    2. Open brackets must be closed in the correct order.
    3. Every close bracket has a corresponding open bracket of the same type.

    Example 1:

    Input: s = "()"

    Output: true

    Example 2:

    Input: s = "()[]{}"

    Output: true

    Example 3:

    Input: s = "(]"

    Output: false

    Example 4:

    Input: s = "([])"

    Output: true

    Example 5:

    Input: s = "([)]"

    Output: false
*/

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();

        for ch in s.chars() {
            match ch {
                '(' | '{' | '[' => stack.push(ch),

                ')' => {
                    if stack.pop() != Some('(') {
                        return false;
                    }
                }

                '}' => {
                    if stack.pop() != Some('{') {
                        return false;
                    }
                }

                ']' => {
                    if stack.pop() != Some('[') {
                        return false;
                    }
                }
                _ => {}
            }
        }

        stack.is_empty()
    }
}
