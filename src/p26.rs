/*
 * @lc app=leetcode.cn id=26 lang=rust
 *
 * [26] 删除有序数组中的重复项
 */

// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut c = 0;
        for i in 1..nums.len() {
            if nums[i] != nums[i-1] {
                c += 1;
                nums[c] = nums[i]
            }
        }
    
        c as i32 + 1
    }
}
// @lc code=end

