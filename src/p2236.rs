/*
 * @lc app=leetcode.cn id=2236 lang=rust
 *
 * [2236] 判断根结点是否等于子结点之和
 */

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = root {
            let val = node.borrow().val;
            let left =  node.borrow().left.as_ref().unwrap().borrow().val;
            let right = node.borrow().right.as_ref().unwrap().borrow().val;
    
            return val == (left + right);
        }
    
        false
    }
}
// @lc code=end

