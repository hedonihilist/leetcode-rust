/*
 * @lc app=leetcode.cn id=9 lang=rust
 *
 * [9] 回文数
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        Self::str_solution(x)
        //Self::num_solution(x)
    }

    pub fn num_solution(x: i32) -> bool {
        /**
         * 思路：反转一半数字
         * 注意特殊情况的处理：0结尾但不是0,一定不是回文
         */
        if x < 0 || x % 10 == 0 && x != 0 {
            return false;
        }
        let mut reverse = 0i32;
        let mut x = x;
        while reverse < x {
            let digit = x % 10;
            x /= 10;
            reverse = reverse * 10 + digit;
        }

        reverse == x || reverse / 10 == x
    }

    pub fn str_solution(x: i32) -> bool {
        x.to_string().chars().rev().collect::<String>() == x.to_string()
    }
}
// @lc code=end

