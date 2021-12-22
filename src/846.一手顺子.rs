/*
 * @lc app=leetcode.cn id=846 lang=rust
 *
 * [846] 一手顺子
 */

// @lc code=start
use std::collections::BTreeMap;

impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        // shortcut
        let group_size = group_size as usize;
        if hand.len() % group_size != 0 {
            return false;
        }

        // 洗牌
        let mut map = BTreeMap::<i32, i32>::new();
        for card in hand {
            map.insert(card, *map.get(&card).unwrap_or(&0) + 1);
        }

        // 码牌
        let mut cards = vec![0; map.len()];
        let mut cnts = vec![0; map.len()];
        for (i, entry) in map.into_iter().enumerate() {
            cards[i] = entry.0;
            cnts[i] = entry.1;
        }

        // 数顺子
        for i in 0..cards.len() {
            if cnts[i] == 0 {
                continue;
            }
            if i + group_size > cards.len() {
                return false;
            }
            if cards[i + group_size - 1] != cards[i] + group_size as i32 - 1 {
                return false;
            }
            let sz = cnts[i]; // 以cards[i]开头的顺子个数
            for j in i..i + group_size {
                if cnts[j] < sz {
                    return false;
                }
                cnts[j] -= sz;
            }
        }

        true
    }
}
// @lc code=end
