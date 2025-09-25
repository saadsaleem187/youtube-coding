/*
    Problem: Palindrome Number

    Description:
    Given an integer x, return true if x is a palindrome, and false otherwise.

    Example 1:
    Input: x = 121
    Output: true
    Explanation: 121 reads as 121 from left to right and from right to left.

    Example 2:
    Input: x = -121
    Output: false
    Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.

    Example 3:
    Input: x = 10
    Output: false
    Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
*/

fn main() {
    let nums: Vec<i32> = vec![121, -121, 10, 12321];

    for &n in &nums {
        println!("{} -> {}", n, is_palindrome(n));
    }
}

fn is_palindrome(mut x: i32) -> bool {
    if x < 0 {
        return false;
    }

    if x < 10 {
        return true;
    }

    let original = x;
    let mut reversed = 0;

    while x > 0 {
        reversed = reversed * 10 + x % 10;
        x /= 10;
    }

    original == reversed
}
