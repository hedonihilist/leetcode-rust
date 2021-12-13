/*
 * @lc app=leetcode.cn id=69 lang=rust
 *
 * [69] Sqrt(x)
 */

// @lc code=start
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        /**
         * 思路：二分
         */
        let x = x as i64;
        let mut lo = 0i64;
        let mut hi = x + 1;
        while lo < hi {
            let mid = (lo + hi) / 2;
            let pow = mid * mid;
            if pow < x {
                lo = mid + 1;
            } else if pow == x {
                return mid as i32;
            } else {
                hi = mid;
            }
        }
        lo as i32 - 1
    }
}
// @lc code=end
