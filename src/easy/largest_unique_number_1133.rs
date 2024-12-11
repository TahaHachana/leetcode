// 1133. Largest Unique Number
// https://leetcode.com/problems/largest-unique-number/description/
// Time complexity: O(n)
// Space complexity: O(n)

// --------------------------------------------------

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn largest_unique_number(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut frequency_map = HashMap::<i32, i32>::new();

        // Count the frequency of each number in the input vector
        for num in nums {
            *frequency_map.entry(num).or_insert(0) += 1;
        }

        let mut largest_unique = -1;

        // Find the largest number with a frequency of exactly 1
        for (key, value) in frequency_map {
            if value == 1 && key > largest_unique {
                largest_unique = key;
            }
        }

        largest_unique
    }
}

// --------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_unique_number_1() {
        assert_eq!(
            Solution::largest_unique_number(vec![5, 7, 3, 9, 4, 9, 8, 3, 1]),
            8
        );
    }

    // --------------------------------------------------

    #[test]
    fn test_largest_unique_number_2() {
        assert_eq!(Solution::largest_unique_number(vec![9, 9, 8, 8]), -1);
    }
}
