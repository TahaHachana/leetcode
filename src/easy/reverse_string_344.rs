// 344. Reverse String
// https://leetcode.com/problems/reverse-string/description/
// Time complexity: O(n)
// Space complexity: O(1)

// --------------------------------------------------

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        // Two pointers
        let mut left = 0;
        let mut right = s.len() - 1;

        // Swap characters while left < right
        while left < right {
            (s[left], s[right]) = (s[right], s[left]);
            left += 1;
            right -= 1;
        }
    }
}

// --------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_string_1() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(&mut s);
        assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);
    }

    // --------------------------------------------------

    #[test]
    fn test_reverse_string_2() {
        let mut s = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        Solution::reverse_string(&mut s);
        assert_eq!(s, vec!['h', 'a', 'n', 'n', 'a', 'H']);
    }
}
