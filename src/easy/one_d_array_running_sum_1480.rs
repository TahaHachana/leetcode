// 1480. Running Sum of 1d Array
// https://leetcode.com/problems/running-sum-of-1d-array/description/

#![allow(dead_code)]

struct Solution;

impl Solution {
    /// Calculates the running sum of a vector of integers in place.
    ///
    /// # Arguments
    ///
    /// * `nums` - A vector of integers.
    ///
    /// # Returns
    ///
    /// The same vector with its elements modified to represent the running sum.
    pub fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_running_sum_1() {
        assert_eq!(Solution::running_sum(vec![1, 2, 3, 4]), vec![1, 3, 6, 10]);
    }

    #[test]
    fn test_running_sum_2() {
        assert_eq!(
            Solution::running_sum(vec![1, 1, 1, 1, 1]),
            vec![1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn test_running_sum_3() {
        assert_eq!(
            Solution::running_sum(vec![3, 1, 2, 10, 1]),
            vec![3, 4, 6, 16, 17]
        );
    }
}