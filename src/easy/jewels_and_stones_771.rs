// 771. Jewels and Stones
// https://leetcode.com/problems/jewels-and-stones/description/
// Time complexity: O(n + m)
// Space complexity: O(1)

// --------------------------------------------------

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        use std::collections::HashSet;

        // Create a HashSet from the characters in the jewels string
        let jewels_set: HashSet<char> = jewels.chars().collect();

        // Count the number of characters in stones that are also in jewels_set
        stones.chars().filter(|c| jewels_set.contains(c)).count() as i32
    }
}

// --------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_jewels_in_stones_1() {
        let jewels = "aA".to_string();
        let stones = "aAAbbbb".to_string();
        assert_eq!(Solution::num_jewels_in_stones(jewels, stones), 3);
    }

    // --------------------------------------------------

    #[test]
    fn test_num_jewels_in_stones_2() {
        let jewels = "z".to_string();
        let stones = "ZZ".to_string();
        assert_eq!(Solution::num_jewels_in_stones(jewels, stones), 0);
    }
}
