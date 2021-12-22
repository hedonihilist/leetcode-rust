/*
 * @lc app=leetcode.cn id=873 lang=rust
 *
 * [873] 最长的斐波那契子序列的长度
 */

// @lc code=start
use std::collections::HashSet;

impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        Self::brute_force(arr)
    }
    fn dp_solution(arr: Vec<i32>) -> i32 {
        // TODO
        0
    }
    fn brute_force(arr: Vec<i32>) -> i32 {
        /**
         * 暴力解法
         */
        let set: HashSet<i32> = arr.iter().map(|&x| x).collect();
        let mut res = 0i32;

        for i in 0..arr.len() {
            for j in i + 1..arr.len() {
                let mut a = arr[i];
                let mut b = arr[j];
                let mut t = 1;
                while set.contains(&b) {
                    t += 1;
                    b = a + b;
                    a = b - a;
                }
                res = res.max(t);
            }
        }

        if res >= 3 {
            res
        } else {
            0
        }
    }
}
// @lc code=end
