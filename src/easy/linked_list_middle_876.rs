// 876. Middle of the Linked List
// https://leetcode.com/problems/middle-of-the-linked-list/description/
// Time complexity: O(n)
// Space complexity: O(1)

// --------------------------------------------------

#![allow(dead_code)]

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Use references to traverse the list.
        let mut slow = &head;
        let mut fast = &head;

        // Move fast two steps and slow one step until fast reaches the end.
        while let Some(f_node) = fast {
            if let Some(next_fast) = &f_node.next {
                fast = &next_fast.next;
                slow = &slow.as_ref().unwrap().next;
            } else {
                break;
            }
        }

        // Now slow is a reference to the sublist starting at the middle node.
        slow.clone()
    }
}

// --------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_middle_node_1() {
        let input = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        }));

        let output = Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 5, next: None })),
            })),
        }));

        assert_eq!(Solution::middle_node(input), output);
    }

    // --------------------------------------------------

    #[test]
    fn test_middle_node_2() {
        let input = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode {
                            val: 5,
                            next: Some(Box::new(ListNode { val: 6, next: None })),
                        })),
                    })),
                })),
            })),
        }));

        let output = Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode { val: 6, next: None })),
            })),
        }));

        assert_eq!(Solution::middle_node(input), output);
    }
}
