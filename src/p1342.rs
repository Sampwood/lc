/*
 * @lc app=leetcode.cn id=1342 lang=rust
 *
 * [1342] 将数字变成 0 的操作次数
 */

// @lc code=start
impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        if 0 == num {
            return 0;
        }

        let str = format!("{num:b}");
        let res: u32 = str[1..].chars().map(|x| x.to_digit(10).unwrap() + 1).sum();
        (res as i32) + 1

        // let str = format!("{num:b}");
        // let sub_str = &str[1..];

        // let mut res = 0;

        // for c in sub_str.chars() {
        //     let v = c.to_digit(10).unwrap();
        //     res += (v as i32) + 1;
        // }

        // res + 1
    }
}
// @lc code=end
