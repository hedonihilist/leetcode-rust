/*
 * @lc app=leetcode.cn id=1346 lang=rust
 *
 * [1346] 检查整数及其两倍数是否存在
 */

// @lc code=start
use std::collections::HashSet;

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut set = HashSet::<i32>::new();
        for i in arr.into_iter() {
            if set.contains(&(i * 2)) || (i % 2 == 0) && set.contains(&(i / 2)) {
                return true;
            }
            set.insert(i);
        }
        false
    }
}
// @lc code=end
