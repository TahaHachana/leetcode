// 1413. Minimum Value to Get Positive Step by Step Sum
// https://leetcode.com/problems/minimum-value-to-get-positive-step-by-step-sum/description/

#![allow(dead_code)]

struct Solution;

impl Solution {
    /// This function calculates the minimum starting value such that the step-by-step sum
    /// of the given list of integers is never less than 1.
    ///
    /// # Arguments
    ///
    /// * `nums` - A vector of integers representing the sequence of numbers.
    ///
    /// # Returns
    ///
    /// * An integer representing the minimum starting value to ensure the step-by-step sum is always positive.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        // Initialize min_val to 0 to keep track of the minimum value of the running total
        let mut min_val = 0;
        // Initialize total to 0 to keep track of the running total of the numbers
        let mut total = 0;

        // Iterate through each number in the input vector
        for num in nums {
            // Add the current number to the running total
            total += num;

            // Update min_val if the current running total is less than min_val
            if total < min_val {
                min_val = total;
            }
        }

        // The minimum start value to ensure the step-by-step sum is always positive
        // is the absolute value of min_val plus 1
        -min_val + 1
    }
}

// --------------------------------------------------

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_min_start_value_1() {
        assert_eq!(Solution::min_start_value(vec![-3, 2, -3, 4, 2]), 5);
    }

    // --------------------------------------------------

    #[test]
    fn test_min_start_value_2() {
        assert_eq!(Solution::min_start_value(vec![1, 2]), 1);
    }

    // --------------------------------------------------

    #[test]
    fn test_min_start_value_3() {
        assert_eq!(Solution::min_start_value(vec![1, -2, -3]), 5);
    }
}
