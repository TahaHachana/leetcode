// 1426. Counting Elements
// https://leetcode.com/problems/counting-elements/description/
// Time complexity: O(n)
// Space complexity: O(n)

// --------------------------------------------------

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn count_elements(arr: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        // Create a HashSet from the array for O(1) average-time complexity lookups
        let hash_set = HashSet::<i32>::from_iter(arr.iter().copied());

        let mut count = 0;

        // Iterate over each element in the array
        for x in arr {
            // Check if x + 1 exists in the HashSet
            if hash_set.contains(&(x + 1)) {
                count += 1;
            }
        }

        count
    }
}

// --------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_elements_1() {
        assert_eq!(Solution::count_elements(vec![1, 2, 3]), 2);
    }

    // --------------------------------------------------

    #[test]
    fn test_count_elements_2() {
        assert_eq!(Solution::count_elements(vec![1, 1, 3, 3, 5, 5, 7, 7]), 0);
    }
}
