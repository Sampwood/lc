/*
 * @lc app=leetcode.cn id=1672 lang=rust
 *
 * [1672] 最富有客户的资产总量
 */

// @lc code=start
use std::cmp;

impl Solution {
    fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut max = 0;
    
        for ac in accounts {
            max = cmp::max(ac.iter().sum(), max);
        }
    
        max
    }
}
// @lc code=end

