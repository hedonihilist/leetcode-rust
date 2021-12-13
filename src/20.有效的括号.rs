/*
 * @lc app=leetcode.cn id=20 lang=rust
 *
 * [20] 有效的括号
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = vec![];
        let map: HashMap<char, char> = vec![('}', '{'), (')', '('), (']', '[')]
            .into_iter()
            .collect();
        for (i, c) in s.chars().enumerate() {
            match c {
                '{' | '[' | '(' => {
                    stack.push(c);
                }
                _ => {
                    if stack.is_empty() || stack.last().unwrap() != map.get(&c).unwrap() {
                        return false;
                    }
                    stack.pop();
                }
            }
        }

        stack.is_empty()
    }
}
// @lc code=end
