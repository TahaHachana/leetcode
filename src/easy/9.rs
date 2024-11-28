// 9. Palindrome Number
// https://leetcode.com/problems/palindrome-number/description/

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        // A negative number can't be a palindrome
        // If the last digit is zero the first one must be zero to which is not possible
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }

        let mut original_number = x;
        let mut reverted_number = 0;

        // Process half the number of digits only
        while original_number > reverted_number {
            reverted_number = reverted_number * 10 + original_number % 10;
            original_number /= 10;
        }

        // Omit the last digit from reverted_number when its number of digits is odd
        original_number == reverted_number || original_number == reverted_number / 10
    }
}
