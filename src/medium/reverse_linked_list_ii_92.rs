// 92. Reverse Linked List II
// https://leetcode.com/problems/reverse-linked-list-ii/description/
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

// impl Solution {
//     pub fn reverse_between(
//         head: Option<Box<ListNode>>,
//         left: i32,
//         right: i32,
//     ) -> Option<Box<ListNode>> {
//         if head.is_none() {
//             return None;
//         }
        
//         let mut left = left;
//         let mut right = right;
//         let mut cur = head;
//         let mut prev = None;
//         let mut head = cur.clone();
        
//         while left > 1 {
//             prev = cur.clone();
//             cur = cur.unwrap().next;
//             left -= 1;
//             right -= 1;
//         }
        
//         let mut con = prev.clone();
//         let mut tail = cur.clone();
//         let mut third = None;
        
//         while right > 0 {
//             let next = cur.as_mut().unwrap().next.take();
//             cur.as_mut().unwrap().next = third;
//             third = cur;
//             cur = next;
//             right -= 1;
//         }
        
//         if let Some(node) = con.as_mut() {
//             node.next = prev.clone();
//         } else {
//             head = prev;
//         }
        
//         tail.as_mut().unwrap().next = cur;
//         head
//     }
// }

// --------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_between_1() {
        // Input = [1,2,3,4,5], left = 2, right = 4
        // Output: [1,4,3,2,5]
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
            val: 1,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        }));

        assert_eq!(Solution::reverse_between(input, 2, 4), output);
    }
}
