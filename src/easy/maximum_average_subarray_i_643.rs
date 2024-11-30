// 643. Maximum Average Subarray I
// https://leetcode.com/problems/maximum-average-subarray-i/description/

#![allow(dead_code)]

struct Solution;

impl Solution {
    /// Finds the maximum average of any contiguous subarray of length `k` in the given array `nums`.
    ///
    /// # Arguments
    ///
    /// * `nums` - A vector of integers representing the array.
    /// * `k` - An integer representing the length of the subarray.
    ///
    /// # Returns
    ///
    /// A floating-point number representing the maximum average of any contiguous subarray of length `k`.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        // Convert k to usize for easier indexing
        let k = k as usize;

        // Calculate the sum of the first k elements
        let mut sum: i32 = nums.iter().take(k).sum();
        // Initialize max_sum with the sum of the first k elements
        let mut max_sum = sum;

        // Iterate through the array starting from the k-th element
        for i in k..nums.len() {
            // Update the sum by adding the current element and subtracting the element that is k positions behind
            sum += nums[i] - nums[i - k];
            // Update max_sum if the current sum is greater
            if sum > max_sum {
                max_sum = sum;
            }
        }

        // Return the maximum average as a floating-point number
        max_sum as f64 / k as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max_average_1() {
        let nums = vec![1, 12, -5, -6, 50, 3];
        let k = 4;
        assert_eq!(Solution::find_max_average(nums, k), 12.75);
    }

    #[test]
    fn test_find_max_average_2() {
        let nums = vec![5];
        let k = 1;
        assert_eq!(Solution::find_max_average(nums, k), 5.0);
    }
}
