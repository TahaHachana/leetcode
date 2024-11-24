// 977. Squares of a Sorted Array
// https://leetcode.com/problems/squares-of-a-sorted-array/description/

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();

        let mut rslt = vec![0; n];

        let mut left = 0;
        let mut right = n - 1;
        let mut i = n - 1;

        // Loop until the left pointer exceeds the right pointer
        while left <= right {
            let to_square: i32;

            // Compare the absolute values of the elements at the left and right pointers
            if nums[left].abs() < nums[right].abs() {
                to_square = nums[right];
                right -= 1; // Move the right pointer to the left
            } else {
                to_square = nums[left];
                left += 1; // Move the left pointer to the right
            }

            rslt[i] = to_square * to_square; // Square the chosen element and store it in the result vector

            // Decrement the index for the result vector, ensuring it doesn't go below zero
            if i > 0 {
                i -= 1;
            } else {
                break;
            }
        }

        rslt
    }
}
