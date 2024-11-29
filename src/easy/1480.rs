// 1480. Running Sum of 1d Array
// https://leetcode.com/problems/running-sum-of-1d-array/

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
