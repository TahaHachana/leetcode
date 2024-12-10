// 2090. K Radius Subarray Averages
// https://leetcode.com/problems/k-radius-subarray-averages/description/
// Time complexity: O(n)
// Space complexity: O(1)

// --------------------------------------------------

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k_usize = k as usize;

        // If k is 0, the radius is 0, so the average of each element is the element itself.
        if k_usize == 0 {
            return nums;
        }

        let window_size = 2 * k_usize + 1;
        let n = nums.len();
        let mut averages = vec![-1; n];

        // If the window size is greater than the length of the array, return the array filled with -1.
        if window_size > n {
            return averages;
        }

        let mut window_sum = 0usize;
        // Calculate the sum of the first window.
        for i in 0..window_size {
            window_sum += nums[i] as usize;
        }

        // Calculate the average for the first valid window position.
        averages[k_usize] = (window_sum / window_size) as i32;

        // Slide the window across the array and update the averages.
        for i in window_size..n {
            // Subtract the element that is left out of the window and add the new element.
            window_sum = window_sum - (nums[i - window_size] as usize) + nums[i] as usize;
            // Calculate the average for the current window position.
            averages[i - k_usize] = (window_sum / window_size) as i32;
        }

        averages
    }
}

// --------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_averages_1() {
        let nums = vec![7, 4, 3, 9, 1, 8, 5, 2, 6];
        let k = 3;
        assert_eq!(
            Solution::get_averages(nums, k),
            vec![-1, -1, -1, 5, 4, 4, -1, -1, -1]
        );
    }

    // --------------------------------------------------

    #[test]
    fn test_get_averages_2() {
        let nums = vec![100000];
        let k = 0;
        assert_eq!(Solution::get_averages(nums, k), vec![100000]);
    }

    // --------------------------------------------------

    #[test]
    fn test_get_averages_3() {
        let nums = vec![8];
        let k = 100000;
        assert_eq!(Solution::get_averages(nums, k), vec![-1]);
    }
}
