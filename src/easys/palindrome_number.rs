#![allow(dead_code)]
#![allow(unused)]

// Palindrome Number
//
// Hint
// Given an integer x, return true if x is a palindrome, and false otherwise.
//
//
// Example 1:
//
// Input: x = 121
// Output: true
// Explanation: 121 reads as 121 from left to right and from right to left.
//
// Example 2:
//
// Input: x = -121
// Output: false
// Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
//
// Example 3:
//
// Input: x = 10
// Output: false
// Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
//
//
// Constraints:
//
// -231 <= x <= 231 - 1

pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let digits: Vec<char> = x.to_string().chars().collect();
    let length = digits.len();

    let mut left = 0;
    let mut right = length - 1;

    while left < right {
        if digits[left] != digits[right] {
            return false;
        }

        left += 1;
        right -= 1;
    }

    true
}

pub fn solution() -> bool {
    let x = -123;

    for i in x.to_string().len()..0 {
        println!("{}", i);
    }

    is_palindrome(x)
}
