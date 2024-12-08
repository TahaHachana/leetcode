// 2225. Find Players With Zero or One Losses
// https://leetcode.com/problems/find-players-with-zero-or-one-losses/description/
// Time complexity: O(n + k)
// Space complexity: O(k)

// --------------------------------------------------

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // Initialize a vector to count losses for each player.
        // -1 indicates that the player has not been encountered yet.
        let mut losses_count = vec![-1; 100001];

        // Iterate through each match to update the losses count.
        for match_ in matches {
            let winner = match_[0] as usize;
            let loser = match_[1] as usize;

            // If the winner has not been encountered before, set their losses to 0.
            if losses_count[winner] == -1 {
                losses_count[winner] = 0;
            }

            // If the loser has not been encountered before, set their losses to 1.
            // Otherwise, increment their losses count.
            if losses_count[loser] == -1 {
                losses_count[loser] = 1;
            } else {
                losses_count[loser] += 1;
            }
        }

        let mut answer = vec![Vec::new(), Vec::new()];

        // Iterate through the losses_count vector to populate the answer.
        for i in 1..100001 {
            if losses_count[i] == 0 {
                // Players with zero losses.
                answer[0].push(i as i32);
            } else if losses_count[i] == 1 {
                // Players with exactly one loss.
                answer[1].push(i as i32);
            }
        }

        answer
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
