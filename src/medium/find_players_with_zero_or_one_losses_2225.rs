// 2225. Find Players With Zero or One Losses
// https://leetcode.com/problems/find-players-with-zero-or-one-losses/description/
// Time complexity: O(n log n)
// Space complexity: O(n)

// --------------------------------------------------

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::HashMap;

        // HashMap to store the number of losses for each player
        let mut losses_count = HashMap::<i32, i32>::new();

        // Iterate over each match
        for single_match in matches {
            // Ensure the winner is in the map with 0 losses
            losses_count.entry(single_match[0]).or_insert(0);

            // Increment the loser's loss count by 1
            *losses_count.entry(single_match[1]).or_insert(0) += 1;
        }

        // Vectors to store players with zero and one loss
        let mut zero_loss = Vec::<i32>::new();
        let mut one_loss = Vec::<i32>::new();
        
        // Categorize players based on their loss count
        for (player, count) in losses_count {
            if count == 0 {
                zero_loss.push(player);
            } else if count == 1 {
                one_loss.push(player);
            }
        }

        // Sort the vectors to meet the problem's requirements
        zero_loss.sort();
        one_loss.sort();

        // Return the result as a vector of vectors
        vec![zero_loss, one_loss]
    }
}

// --------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_winners_1() {
        let matches = vec![
            vec![1, 3],
            vec![2, 3],
            vec![3, 6],
            vec![5, 6],
            vec![5, 7],
            vec![4, 5],
            vec![4, 8],
            vec![4, 9],
            vec![10, 4],
            vec![10, 9],
        ];
        assert_eq!(
            Solution::find_winners(matches),
            vec![vec![1, 2, 10], vec![4, 5, 7, 8]]
        );
    }

    // --------------------------------------------------

    #[test]
    fn test_find_winners_2() {
        let matches = vec![vec![2, 3], vec![1, 3], vec![5, 4], vec![6, 4]];
        assert_eq!(
            Solution::find_winners(matches),
            vec![vec![1, 2, 5, 6], vec![]]
        );
    }
}
