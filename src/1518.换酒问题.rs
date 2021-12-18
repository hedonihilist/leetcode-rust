/*
 * @lc app=leetcode.cn id=1518 lang=rust
 *
 * [1518] 换酒问题
 */

// @lc code=start
impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        /**
         * 直接模拟
         */
        let mut empty = num_bottles;    // 当前的空酒瓶子
        let mut drink = num_bottles;

        while empty >= num_exchange {
            let t = empty / num_exchange;
            empty = empty % num_exchange;
            empty += t;
            drink += t;
        }

        drink
    }
}
// @lc code=end

