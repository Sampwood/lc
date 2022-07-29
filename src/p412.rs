/*
 * @lc app=leetcode.cn id=412 lang=rust
 *
 * [412] Fizz Buzz
 */

// @lc code=start
impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut results = Vec::new();

        for i in 0..n {
            let index = i + 1;
            let mut r = String::from("");
            if index % 3 == 0 {
                r.push_str("Fizz");
            }
            if index % 5 == 0 {
                r.push_str("Buzz");
            }

            if r.len() == 0 {
                r = index.to_string();
            }

            results.push(r);
        };

        results
    }
}
// @lc code=end

