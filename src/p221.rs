/*
 * @lc app=leetcode.cn id=221 lang=rust
 *
 * [221] 最大正方形
 */

// @lc code=start
use std::cmp;

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let mut result = 0;
        for i in 0..matrix.len() {
            let row = &matrix[i];
            let mut len = 0;

            for j in 0..row.len() {
                let item = row[j];
                if item == '0' {
                    len = 0;
                } else if len == 0 {
                    len = 1;
                    result = cmp::max(result, len);
                } else {
                    if i + len > matrix.len() - 1 {
                        len = 0;
                    } else {
                        let mut reduce = 0;
                        for k in 1..len + 1 {
                            if matrix[i + k][j] == '0' {
                                len = 0;
                                break;
                            }
                            if matrix[i + len][j - k] == '0' {
                                reduce = cmp::max(reduce, 1);
                            }
                        }
                        len = len - reduce;
                    }
                    len = len + 1;
                    result = cmp::max(result, len);
                }
            }
        }

        (result * result) as i32
    }
}
// @lc code=end
