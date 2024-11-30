// 977. Squares of a Sorted Array
// https://leetcode.com/problems/squares-of-a-sorted-array/description/
// Time complexity: O(n)
// Space complexity: O(n) when considering the output vector

// --------------------------------------------------

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        // Initialize a result vector of the same length as nums with zeros
        let mut rslt = vec![0; n];
        // Initialize two pointers
        let mut left = 0;
        let mut right = n - 1;

        // Iterate from the end of the result vector to the beginning
        for i in (0..n).rev() {
            // Compare the absolute values of the elements at the left and right pointers
            if nums[left].abs() < nums[right].abs() {
                rslt[i] = nums[right] * nums[right];
                right -= 1; // Move the right pointer to the left
            } else {
                rslt[i] = nums[left] * nums[left];
                left += 1; // Move the left pointer to the right
            }
        }

        rslt
    }
}

// --------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_squares_1() {
        assert_eq!(
            Solution::sorted_squares(vec![-4, -1, 0, 3, 10]),
            vec![0, 1, 9, 16, 100]
        );
    }

    // --------------------------------------------------

    #[test]
    fn test_sorted_squares_2() {
        assert_eq!(
            Solution::sorted_squares(vec![-7, -3, 2, 3, 11]),
            vec![4, 9, 9, 49, 121]
        );
    }
}
