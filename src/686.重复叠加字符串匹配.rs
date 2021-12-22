/*
 * @lc app=leetcode.cn id=686 lang=rust
 *
 * [686] 重复叠加字符串匹配
 */

// @lc code=start
impl Solution {
    pub fn repeated_string_match(a: String, b: String) -> i32 {
        let mut s = a.to_string();
        let mut res = 1i32;
        while s.len() < b.len() {
            s.push_str(&a);
            res += 1;
        }

        if s.find(&b).is_some() {
            res
        } else if (s+&a).find(&b).is_some() {
            res+1
        } else {
            -1
        }
    }
}
// @lc code=end
