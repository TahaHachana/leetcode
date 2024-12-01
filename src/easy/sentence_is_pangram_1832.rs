// 1832. Check if the Sentence Is Pangram
// https://leetcode.com/problems/check-if-the-sentence-is-pangram/description/
// Time complexity: O(n)
// Space complexity: O(1)

// --------------------------------------------------

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        use std::collections::HashSet;

        // Create a HashSet from the characters of the sentence
        let seen = HashSet::<char>::from_iter(sentence.chars());

        // All letter are present if the length of the HashSet is 26
        seen.len() == 26
    }
}

// --------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_if_pangram_1() {
        let sentence = "thequickbrownfoxjumpsoverthelazydog".to_string();
        assert_eq!(Solution::check_if_pangram(sentence), true);
    }

    #[test]
    fn test_check_if_pangram_2() {
        let sentence = "leetcode".to_string();
        assert_eq!(Solution::check_if_pangram(sentence), false);
    }
}
