/*
 * @lc app=leetcode.cn id=1975 lang=rust
 *
 * [1975] 最大方阵和
 */

// @lc code=start

impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut abs_min = i32::MAX as i64;
        let mut neg_cnt = 0;
        let mut zero_cnt = 0;
        let mut res = 0i64;
        for r in 0..matrix.len() {
            for c in 0..matrix[0].len() {
                let e = matrix[r][c];
                abs_min = abs_min.min((e as i64).abs());
                if e < 0 {
                    neg_cnt += 1;
                } else if e == 0 {
                    zero_cnt += 1;
                }
                res += (e as i64).abs();
            }
        }
        if (neg_cnt & 0x1) != 0 && zero_cnt == 0 {
            res -= 2 * abs_min;
        }
        res
    }
}
// @lc code=end
