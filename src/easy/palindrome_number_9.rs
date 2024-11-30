// 9. Palindrome Number
// https://leetcode.com/problems/palindrome-number/description/
// Time complexity: O(log(n))
// Space complexity: O(1)

// --------------------------------------------------

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn is_palindrome(mut x: i32) -> bool {
        // Return false if the number is negative or ends with a zero (but is not zero itself)
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }

        let mut reversed = 0;

        // Reverse the number until the original number is less than or equal to the reversed number
        while x > reversed {
            reversed = reversed * 10 + x % 10;
            x /= 10;
        }

        // Check if the original number is equal to the reversed number
        // or if the original number is equal to the reversed number divided by 10
        // (this accounts for the case where the number of digits is odd)
        x == reversed || x == reversed / 10
    }
}

// --------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome_1() {
        assert_eq!(Solution::is_palindrome(121), true);
    }

    // --------------------------------------------------

    #[test]
    fn test_is_palindrome_2() {
        assert_eq!(Solution::is_palindrome(-121), false);
    }

    // --------------------------------------------------

    #[test]
    fn test_is_palindrome_3() {
        assert_eq!(Solution::is_palindrome(10), false);
    }
}
