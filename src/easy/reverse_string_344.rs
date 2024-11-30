// 344. Reverse String
// https://leetcode.com/problems/reverse-string/description/

#![allow(dead_code)]

struct Solution;

impl Solution {
    /// Reverses the characters in the given vector `s` in place.
    ///
    /// # Arguments
    ///
    /// * `s` - A mutable reference to a vector of characters that will be reversed.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut left = 0;
        let mut right = s.len() - 1;

        while left < right {
            (s[left], s[right]) = (s[right], s[left]);
            left += 1;
            right -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_string_1() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(&mut s);
        assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);
    }

    #[test]
    fn test_reverse_string_2() {
        let mut s = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        Solution::reverse_string(&mut s);
        assert_eq!(s, vec!['h', 'a', 'n', 'n', 'a', 'H']);
    }
}
