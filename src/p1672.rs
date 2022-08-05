/*
 * @lc app=leetcode.cn id=1672 lang=rust
 *
 * [1672] 最富有客户的资产总量
 */

// @lc code=start
impl Solution {
    fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut max = 0;
    
        for ac in accounts {
            let sum = ac.iter().sum();
            if sum > max {
                max = sum;
            }
        }
    
        max
    }
}
// @lc code=end

