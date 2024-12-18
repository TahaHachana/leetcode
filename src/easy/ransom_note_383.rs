// 383. Ransom Note
// https://leetcode.com/problems/ransom-note/description/
// Time complexity: O(n)
// Space complexity: O(1)

// --------------------------------------------------

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        // If the ransom note is longer than the magazine, it's impossible to construct it
        if ransom_note.len() > magazine.len() {
            return false;
        }

        use std::collections::HashMap;

        // Create a HashMap to count the occurrences of each character in the magazine
        let mut magazine_counts = HashMap::new();

        // Count each character in the magazine
        for c in magazine.chars() {
            magazine_counts
                .entry(c)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }

        // Check each character in the ransom note
        for c in ransom_note.chars() {
            // Get the count of the character in the magazine (default to 0 if not found)
            let count_in_magazine = magazine_counts.entry(c).or_default();
            // If the character is not available in the required quantity, return false
            if *count_in_magazine == 0 {
                return false;
            }
            // Decrement the count of the character in the magazine
            magazine_counts.entry(c).and_modify(|e| *e -= 1);
        }

        // If all characters are available in the required quantities, return true
        true
    }
}

// --------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_construct_1() {
        let ransom_note = "a".to_string();
        let magazine = "b".to_string();
        assert_eq!(Solution::can_construct(ransom_note, magazine), false);
    }

    // --------------------------------------------------

    #[test]
    fn test_can_construct_2() {
        let ransom_note = "aa".to_string();
        let magazine = "ab".to_string();
        assert_eq!(Solution::can_construct(ransom_note, magazine), false);
    }

    // --------------------------------------------------

    #[test]
    fn test_can_construct_3() {
        let ransom_note = "aa".to_string();
        let magazine = "aab".to_string();
        assert_eq!(Solution::can_construct(ransom_note, magazine), true);
    }
}
