/*
 * @lc app=leetcode.cn id=66 lang=rust
 *
 * [66] 加一
 */

// @lc code=start
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut carry = 1i32;
        for i in (0..digits.len()).rev() {
            if carry == 0 {
                break;
            }
            digits[i] += carry;
            carry = digits[i] / 10;
            digits[i] %= 10;
        }
        if carry != 0 {
            digits.insert(0, carry);
        }
        digits
    }
}
// @lc code=end
