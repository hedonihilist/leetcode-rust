/*
 * @lc app=leetcode.cn id=921 lang=rust
 *
 * [921] 使括号有效的最少添加
 */

// @lc code=start
impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut left = vec![];
        let mut res = 0i32;
        for c in s.chars() {
            if c == '(' {
                left.push(c);
            } else {
                // ')'
                if left.is_empty() {
                    res += 1;
                } else {
                    left.pop();
                }
            }
        }

        res + left.len() as i32
    }
}
// @lc code=end
