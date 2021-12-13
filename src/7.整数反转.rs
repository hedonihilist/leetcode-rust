use std::env::consts::FAMILY;

/*
 * @lc app=leetcode.cn id=7 lang=rust
 *
 * [7] 整数反转
 */

// @lc code=start
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let x = x as i64;
        let sign: i64 = match x >= 0 {
            true => 1,
            false => -1,
        };
        let rev: String = x.abs().to_string().chars().rev().collect();
        let rev_x: i64 = sign * i64::from_str_radix(&rev, 10).unwrap();
        if i32::MIN as i64 <= rev_x && rev_x <= i32::MAX as i64 {
            return rev_x as i32;
        }
        0
    }
}
// @lc code=end
