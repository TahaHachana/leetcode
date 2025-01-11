// 3. Longest Substring Without Repeating Characters
// https://leetcode.com/problems/longest-substring-without-repeating-characters/description/
// Time complexity: O(n)
// Space complexity: O(m)

// --------------------------------------------------

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashMap;

        let n = s.len();
        let mut res = 0;
        let mut char_to_next_index = HashMap::new();
        let s_chars: Vec<char> = s.chars().collect();

        let mut i = 0;
        for j in 0..n {
            // If the character s_chars[j] has been seen before and is within
            // the current window, update the start of the window (i) to the
            // maximum of its current value and the next index of the character.
            if let Some(&next_index) = char_to_next_index.get(&s_chars[j]) {
                i = i.max(next_index);
            }
            // Update the result to the maximum length of the current window.
            res = res.max((j - i + 1) as i32);
            // Update the HashMap with the next index of the current character.
            char_to_next_index.insert(s_chars[j], j + 1);
        }

        res
    }
}

// --------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "abcabcbb".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 3);
    }

    // --------------------------------------------------

    #[test]
    fn test_2() {
        let s = "bbbbb".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 1);
    }

    // --------------------------------------------------

    #[test]
    fn test_3() {
        let s = "pwwkew".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 3);
    }
}
