// 525. Contiguous Array
// https://leetcode.com/problems/contiguous-array/description/
// Time complexity: O(n)
// Space complexity: O(n)

// --------------------------------------------------

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        // HashMap to store the cumulative count and its corresponding index
        let mut map = HashMap::new();
        // Initialize with (0, -1) to handle subarrays starting from the beginning
        map.insert(0, -1);

        let mut maxlen = 0; // Variable to store the maximum length of the subarray
        let mut count = 0; // Variable to keep track of the cumulative count of 1s and 0s

        // Iterate through the array with index and value
        for (i, &num) in nums.iter().enumerate() {
            // Update the count: +1 for 1, -1 for 0
            count += if num == 1 { 1 } else { -1 };

            // Check if the current count has been seen before
            if let Some(&prev_index) = map.get(&count) {
                // If seen, calculate the length of the subarray and update maxlen if necessary
                maxlen = maxlen.max(i as i32 - prev_index);
            } else {
                // If not seen, store the current index with the current count
                map.insert(count, i as i32);
            }
        }

        maxlen
    }
}

// --------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max_length_1() {
        let nums = vec![0, 1];
        assert_eq!(Solution::find_max_length(nums), 2);
    }

    // --------------------------------------------------

    #[test]
    fn test_find_max_length_2() {
        let nums = vec![0, 1, 0];
        assert_eq!(Solution::find_max_length(nums), 2);
    }
}
