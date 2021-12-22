/*
 * @lc app=leetcode.cn id=1653 lang=rust
 *
 * [1653] 使字符串平衡的最少删除次数
 */

// @lc code=start
impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let len = s.len();
        let mut b_count = s.chars().filter(|&x| x == 'b').count();
        let mut a_count = 0usize;
        let mut res = b_count.min(len - b_count);

        for c in s.chars() {
            if c == 'a' {
                a_count += 1;
            } else {
                b_count -= 1;
            }
            res = res.min(len - a_count - b_count);
        }

        res as i32
    }
}
// @lc code=end
