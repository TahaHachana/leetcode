// 1189. Maximum Number of Balloons
// https://leetcode.com/problems/maximum-number-of-balloons/description/
// Time complexity: O(n + m)
// Space complexity: O(1)

// --------------------------------------------------

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn find_max_number_of_pattern(text: String, pattern: String) -> i32 {
        const OFFSET: usize = 'a' as usize;
        let mut answer = i32::MAX;
        let mut freq_in_text = [0; 26];
        let mut freq_in_pattern = [0; 26];

        // Calculate frequency of characters in text and pattern.
        for ch in text.chars() {
            freq_in_text[(ch as usize) - OFFSET] += 1;
        }
        for ch in pattern.chars() {
            freq_in_pattern[(ch as usize) - OFFSET] += 1;
        }

        // Compare the maximum string that can be produced
        // considering one character at a time.
        for i in 0..26 {
            if freq_in_pattern[i] > 0 {
                answer = answer.min(freq_in_text[i] / freq_in_pattern[i]);
            }
        }

        answer
    }

    pub fn max_number_of_balloons(text: String) -> i32 {
        let pattern = "balloon".to_string();
        Self::find_max_number_of_pattern(text, pattern)
    }
}

// --------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_number_of_balloons_1() {
        assert_eq!(Solution::max_number_of_balloons("nlaebolko".to_string()), 1);
    }

    // --------------------------------------------------

    #[test]
    fn test_max_number_of_balloons_2() {
        assert_eq!(
            Solution::max_number_of_balloons("loonbalxballpoon".to_string()),
            2
        );
    }

    // --------------------------------------------------

    #[test]
    fn test_max_number_of_balloons_3() {
        assert_eq!(Solution::max_number_of_balloons("leetcode".to_string()), 0);
    }
}
