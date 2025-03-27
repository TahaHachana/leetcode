// 83. Remove Duplicates from Sorted List
// https://leetcode.com/problems/remove-duplicates-from-sorted-list/description/
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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Create a mutable pointer to iterate the list.
        let mut head = head;
        let mut current = head.as_mut();

        while let Some(node) = current {
            // Continue removing duplicates as long as the next node exists
            // and whose value is equal to the current one.
            while let Some(next_node) = node.next.as_mut() {
                if node.val == next_node.val {
                    // Remove duplicate by taking the next pointer out of next_node.
                    node.next = next_node.next.take();
                } else {
                    break;
                }
            }
            // Move the pointer forward.
            current = node.next.as_mut();
        }

        head
    }
}

// --------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates_from_sorted_list_1() {
        assert_eq!(
            Solution::delete_duplicates(Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode { val: 2, next: None }))
                }))
            }))),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 2, next: None }))
            }))
        );
    }

    // --------------------------------------------------

    #[test]
    fn test_remove_duplicates_from_sorted_list_2() {
        assert_eq!(
            Solution::delete_duplicates(Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 3,
                            next: Some(Box::new(ListNode { val: 3, next: None }))
                        }))
                    }))
                }))
            }))),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 3, next: None }))
                }))
            }))
        );
    }
}
