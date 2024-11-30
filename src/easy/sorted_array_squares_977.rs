// 977. Squares of a Sorted Array
// https://leetcode.com/problems/squares-of-a-sorted-array/description/

#![allow(dead_code)]

struct Solution;

impl Solution {
    /// This function takes a vector of integers, squares each element, and returns a new vector
    /// with the squared values sorted in non-decreasing order.
    ///
    /// # Arguments
    ///
    /// * `nums` - A vector of integers, which can include both negative and positive values.
    ///
    /// # Returns
    ///
    /// A vector of integers containing the squared values of the input vector, sorted in non-decreasing order.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) when considering the output vector
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut rslt = vec![0; n];
        let mut left = 0;
        let mut right = n - 1;

        for i in (0..n).rev() {
            if nums[left].abs() < nums[right].abs() {
                rslt[i] = nums[right] * nums[right];
                right -= 1;
            } else {
                rslt[i] = nums[left] * nums[left];
                left += 1;
            }
        }

        rslt
    }
}

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

    #[test]
    fn test_sorted_squares_2() {
        assert_eq!(
            Solution::sorted_squares(vec![-7, -3, 2, 3, 11]),
            vec![4, 9, 9, 49, 121]
        );
    }
}
