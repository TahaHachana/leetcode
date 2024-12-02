// 268. Missing Number
// https://leetcode.com/problems/missing-number/description/
// Time complexity: O(n)
// Space complexity: O(1)

// --------------------------------------------------

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        // Get the length of the nums vector
        let n = nums.len() as i32;

        // Calculate the expected sum of the first n natural numbers
        let expected_sum = n * (n + 1) / 2;

        // Calculate the actual sum of the elements in the nums vector
        let actual_sum: i32 = nums.iter().sum();

        // The missing number is the difference between the expected sum and the actual sum
        expected_sum - actual_sum
    }
}

// --------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_number_1() {
        let nums = vec![3, 0, 1];
        assert_eq!(Solution::missing_number(nums), 2);
    }

    // --------------------------------------------------

    #[test]
    fn test_missing_number_2() {
        let nums = vec![0, 1];
        assert_eq!(Solution::missing_number(nums), 2);
    }

    // --------------------------------------------------

    #[test]
    fn test_missing_number_3() {
        let nums = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
        assert_eq!(Solution::missing_number(nums), 8);
    }
}
