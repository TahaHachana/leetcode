// 1004. Max Consecutive Ones III
// https://leetcode.com/problems/max-consecutive-ones-iii/description/

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        // Left and right pointers for the sliding window
        let mut left = 0;
        let mut right = 0;
        // Remaining number of zeros that can be flipped
        let mut remaining_flips = k;

        while right < nums.len() {
            // If the current element is 0, decrement the remaining flips
            if nums[right] == 0 {
                remaining_flips -= 1;
            }

            // If the remaining flips are less than 0, it means we have more than k zeros in the window
            if remaining_flips < 0 {
                // Adjust the remaining flips by adding back the value at the left pointer
                remaining_flips += 1 - nums[left];
                // Move the left pointer to the right to shrink the window
                left += 1;
            }

            // Move the right pointer to the right to expand the window
            right += 1;
        }

        // The length of the longest subarray with at most k zeros is the difference between right and left pointers
        (right - left) as i32
    }
}