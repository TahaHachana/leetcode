// 20. Valid Parentheses
// https://leetcode.com/problems/valid-parentheses/description/
// Time complexity: O(n)
// Space complexity: O(n)

// --------------------------------------------------

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        // Initialize an empty stack to keep track of opening brackets
        let mut stack = Vec::new();

        // Iterate over each character in the input string
        for c in s.chars() {
            match c {
                // If the character is an opening bracket, push it onto the stack
                '(' | '{' | '[' => stack.push(c),
                // If the character is a closing bracket, check if it matches the top of the stack
                ')' => {
                    // If the top of the stack is not the corresponding opening bracket, return false
                    if stack.pop() != Some('(') {
                        return false;
                    }
                }
                '}' => {
                    // If the top of the stack is not the corresponding opening bracket, return false
                    if stack.pop() != Some('{') {
                        return false;
                    }
                }
                ']' => {
                    // If the top of the stack is not the corresponding opening bracket, return false
                    if stack.pop() != Some('[') {
                        return false;
                    }
                }
                _ => {}
            }
        }
        // If the stack is empty, all opening brackets had matching closing brackets
        stack.is_empty()
    }
}

// --------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_1() {
        assert_eq!(Solution::is_valid(String::from("()")), true);
    }

    // --------------------------------------------------

    #[test]
    fn test_is_valid_2() {
        assert_eq!(Solution::is_valid(String::from("()[]{}")), true);
    }

    // --------------------------------------------------

    #[test]
    fn test_is_valid_3() {
        assert_eq!(Solution::is_valid(String::from("(]")), false);
    }
}
