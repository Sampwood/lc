/*
 * @lc app=leetcode.cn id=876 lang=rust
 *
 * [876] 链表的中间结点
 */

// pub struct Solution {}

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }

// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cur = head;
        let mut res = cur.clone();
        while let Some(origin) = cur {
            cur = origin.next;

            if let Some(next) = cur {
                cur = next.next;
                res = res.unwrap().next;
            }
        }

        res
    }
}
// @lc code=end
