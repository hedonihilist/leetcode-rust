use std::backtrace::BacktraceStatus;

/*
 * @lc app=leetcode.cn id=14 lang=rust
 *
 * [14] 最长公共前缀
 */

// @lc code=start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        Self::ugly_version(strs)
    }

    fn ugly_version(strs: Vec<String>) -> String {
        /**
         * 思路：不想写Trie，直接记录下字符串，不断缩小前缀
         */
        if strs.is_empty() {
            return "".to_owned();
        }
        let base = &strs[0];
        let mut max = base.len();

        for i in 1..strs.len() {
            let s = &strs[i];
            max = Self::prefix_size(base, s, max.min(base.len()).min(s.len()));
        }
        return base[..max].to_owned();
    }

    fn prefix_size(a: &str, b: &str, n: usize) -> usize {
        let mut i: usize = 0;
        let mut itera = a.chars();
        let mut iterb = b.chars();
        while i < n {
            if itera.next().unwrap() != iterb.next().unwrap() {
                break;
            }
            i += 1;
        }
        i
    }
}
// @lc code=end
