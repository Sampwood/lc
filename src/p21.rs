/*
 * @lc app=leetcode.cn id=21 lang=rust
 *
 * [21] 合并两个有序链表
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if list1.is_none() {
            return list2;
        } else if list2.is_none() {
            return list1;
        }

        let mut head = ListNode::new(0);
        let mut cur = &mut head;
        let mut l1 = list1;
        let mut l2 = list2;

        while let (Some(i1), Some(i2)) = (l1.as_ref(), l2.as_ref()) {
            if i1.val <= i2.val {
                cur.next = l1;
                cur = cur.next.as_mut().unwrap();
                l1 = cur.next.take();
            } else {
                cur.next = l2;
                cur = cur.next.as_mut().unwrap();
                l2 = cur.next.take();
            }
        }

        if l1.is_none() {
            cur.next = l2;
        } else {
            cur.next = l1;
        }

        head.next
    }
}
// @lc code=end
